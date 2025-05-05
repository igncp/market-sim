use std::collections::BTreeMap;

use crate::core::{
    order::{Order, OrderSide, OrderStatus, OrderType},
    price::Prices,
    stock::StockOwner,
    stock_exchange::StockExchange,
    time::TimeHandler,
};
use rand::{seq::SliceRandom, Rng};
use rust_decimal::{prelude::FromPrimitive, Decimal};

use super::Simulation;

mod verify_holidays;
mod verify_investors;

impl Simulation {
    fn create_new_orders(&mut self, se: &mut StockExchange, time: &TimeHandler) {
        let new_orders_num = self.r.gen_range(0..=self.settings.max_orders_per_tick);

        for _ in 0..new_orders_num {
            let investor = se.investors.get_random(&mut self.r);
            let lowest_bid_price = se.prices.get_lowest_bid_price();
            let can_buy = if let Some(lowest_bid_price) = lowest_bid_price {
                investor.liquid_cash.value > lowest_bid_price.value
            } else {
                true
            };
            let stock_owner = StockOwner::Investor(investor.id);
            let has_orders = se.orders_book.has_orders(&stock_owner);

            // @settings
            if has_orders {
                continue;
            }

            let stock_owner = StockOwner::Investor(investor.id);
            let has_stocks = se.owned_stocks.has_stocks(&stock_owner);

            let order_side = {
                if can_buy && has_stocks {
                    Some(if self.r.gen_bool(0.5) {
                        OrderSide::Buy
                    } else {
                        OrderSide::Sell
                    })
                } else if can_buy {
                    Some(OrderSide::Buy)
                } else if has_stocks {
                    Some(OrderSide::Sell)
                } else {
                    None
                }
            };

            if order_side.is_none() {
                continue;
            }

            let order_side = order_side.unwrap();

            match order_side {
                OrderSide::Sell => {
                    let stock_to_sell = se
                        .owned_stocks
                        .0
                        .get(&stock_owner)
                        .unwrap()
                        .choose(&mut self.r)
                        .unwrap();

                    let lot_size = se
                        .listed_companies
                        .mapping
                        .values()
                        .find(|company| company.symbol == stock_to_sell.symbol)
                        .unwrap()
                        .lot_size;
                    let lots_to_sell = self.r.gen_range(1..=stock_to_sell.quantity / lot_size);
                    let shares = lots_to_sell * lot_size;
                    let owner_id = StockOwner::Investor(investor.id);
                    let order_type = OrderType::Market;
                    let new_order = Order {
                        order_side: OrderSide::Sell,
                        order_type,
                        owner_id,
                        shares,
                        status: OrderStatus::Init,
                        symbol: stock_to_sell.symbol.clone(),
                    };

                    se.place_order(&new_order, time).unwrap_or_else(|e| {
                        println!("Error placing order: {:?}", e);
                        std::process::exit(1);
                    });
                }
                OrderSide::Buy => {
                    let afforded_companies = se
                        .listed_companies
                        .mapping
                        .values()
                        .filter(|company| {
                            let price = se.prices.get_ask_price(&company.symbol).unwrap();
                            let price_per_lot = price
                                .value
                                .checked_mul(Decimal::new(company.lot_size as i64, 0))
                                .unwrap();

                            investor.liquid_cash.value > price_per_lot
                        })
                        .collect::<Vec<_>>();
                    let afforded_company = afforded_companies.choose(&mut self.r);

                    if afforded_company.is_none() {
                        continue;
                    }

                    let company = afforded_company.unwrap();
                    let max_affordable_lots = (investor.liquid_cash.value
                        / (se
                            .prices
                            .get_ask_price(&company.symbol)
                            .unwrap()
                            .value
                            .checked_mul(Decimal::new(company.lot_size as i64, 0)))
                        .unwrap())
                    .floor();

                    if max_affordable_lots.is_zero() {
                        continue;
                    }

                    let lots_to_buy = self
                        .r
                        .gen_range(1..=max_affordable_lots.try_into().unwrap());
                    let shares = lots_to_buy * company.lot_size;
                    let owner_id = StockOwner::Investor(investor.id);
                    let order_type = OrderType::Market;
                    let new_order = Order {
                        order_side: OrderSide::Buy,
                        order_type,
                        owner_id,
                        shares,
                        status: OrderStatus::Init,
                        symbol: company.symbol.clone(),
                    };

                    se.place_order(&new_order, time).unwrap_or_else(|e| {
                        println!("Error placing order: {:?}", e);
                        std::process::exit(1);
                    });
                }
            }
        }
    }

    fn update_prices(&mut self, se: &mut StockExchange) {
        let mut new_prices = BTreeMap::new();

        for (symbol, price) in &se.prices.0 {
            let average = price.get_average().value;

            let price_change = Decimal::from_f64(self.r.gen_range(-0.1..=0.1))
                .unwrap()
                .round_dp(2);
            let new_price = (average + price_change).round_dp(2);
            let spread = Decimal::from_f64(self.r.gen_range(0.1..=2.0))
                .unwrap()
                .round_dp(2);

            new_prices.insert(symbol.clone(), price.get_with_spread(new_price, spread));
        }

        se.prices = Prices(new_prices);
    }

    pub fn run(&mut self, se: &mut StockExchange, time: &TimeHandler) -> Result<(), String> {
        // WIP: Steps to run:
        // - Introduce brokers
        // - List new companies via IPO
        // - Remove comanies via delisting
        // - Introduce random price changes due to good/bad news of companies

        let current_day = time.get_virtual_day_formatted();
        if self.daily_checks != Some(current_day) {
            self.verify_holidays(se, time)?;
            self.verify_investors(se, time)?;

            let current_day = time.get_virtual_day_formatted();
            self.daily_checks = Some(current_day);
        }

        if se.can_trade_now(time) {
            self.create_new_orders(se, time);
            se.execute_orders();
        } else {
            se.flush_orders();
        }

        self.update_prices(se);
        self.price_storage
            .save_historic_price(&se.prices, time)
            .map_err(|e| format!("Error saving historic price: {:?}", e))?;

        Ok(())
    }
}
