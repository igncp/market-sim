use super::StockExchange;
use crate::core::{
    money::Money,
    order::{Order, OrderSide, OrderType},
    stock::{Stock, StockOwner},
};
use std::{cmp::max, collections::HashSet};

impl StockExchange {
    pub fn execute_orders(&mut self) {
        let mut orders_to_remove: HashSet<Order> = Default::default();

        for order in &self.orders_book.0 {
            let corresponding_orders = self
                .orders_book
                .get_matching_orders(order, Some(&orders_to_remove));

            if orders_to_remove.contains(order) {
                continue;
            }

            let Some((payer_id, total_pay, affordable_order)) =
                corresponding_orders.iter().find_map(|other_order| {
                    let price = self.prices.get_average_price(&order.symbol)?;
                    let total = price.value * Money::from_u64(other_order.shares);
                    let payer_id = if order.order_side == OrderSide::Buy {
                        &order.owner_id
                    } else {
                        &other_order.owner_id
                    };

                    match order.clone().order_type {
                        OrderType::Market => {}
                        OrderType::Limit { price } => {
                            let limit_price = Money {
                                value: price.parse().unwrap(),
                                currency: Default::default(),
                            };

                            if order.order_side == OrderSide::Buy {
                                if limit_price.value < total {
                                    return None;
                                }
                            } else if limit_price.value > total {
                                return None;
                            }
                        }
                    };

                    match payer_id {
                        StockOwner::Investor(id) => {
                            let payer =
                                self.investors.mapping.iter().find(|i| i.0 == id).unwrap().1;

                            if payer.liquid_cash.value >= total {
                                let total_pay = Money {
                                    value: total,
                                    currency: price.currency,
                                };
                                Some((payer_id, total_pay, other_order.clone()))
                            } else {
                                None
                            }
                        }
                        StockOwner::MarketMaker(_) => {
                            let total_pay = Money {
                                value: total,
                                currency: price.currency,
                            };
                            Some((payer_id, total_pay, other_order.clone()))
                        }
                    }
                })
            else {
                continue;
            };

            orders_to_remove.insert(affordable_order.clone());
            orders_to_remove.insert(order.clone());

            {
                match payer_id {
                    StockOwner::MarketMaker(_) => {}
                    StockOwner::Investor(payer_id) => {
                        let payer = self
                            .investors
                            .mapping
                            .iter_mut()
                            .find(|i| i.0 == payer_id)
                            .unwrap()
                            .1;

                        payer.subtract_cash(&total_pay);
                    }
                };
            }

            {
                match affordable_order.owner_id {
                    StockOwner::MarketMaker(_) => {}
                    StockOwner::Investor(owner_id) => {
                        let seller = self
                            .investors
                            .mapping
                            .iter_mut()
                            .find(|i| i.0 == &owner_id)
                            .unwrap()
                            .1;

                        seller.add_cash(&total_pay);
                    }
                };
            }

            let new_stock = Stock {
                owner: *payer_id,
                price: self.prices.0.get(&order.symbol).unwrap().get_average(),
                quantity: affordable_order.shares,
                symbol: order.symbol.clone(),
            };

            let payer_stocs = self.owned_stocks.entry_with_default(payer_id);

            payer_stocs.push(new_stock.clone());

            let seller_all_stocks = self
                .owned_stocks
                .entry_with_default(&affordable_order.owner_id);

            let mut seller_company_stocks = seller_all_stocks
                .iter_mut()
                .filter(|stock| stock.symbol == order.symbol)
                .collect::<Vec<_>>();

            let mut shares = affordable_order.shares;
            for stock in seller_company_stocks.iter_mut() {
                let remaining_shares = max(0, shares as i128 - stock.quantity as i128) as u64;

                if remaining_shares == 0 {
                    stock.quantity -= shares;
                    break;
                } else {
                    shares = remaining_shares;
                    stock.quantity = 0;
                }
            }

            seller_all_stocks.retain(|stock| stock.quantity > 0);
        }

        for order in orders_to_remove {
            self.orders_book.0.retain(|o| o != &order);
        }
    }
}
