use std::collections::HashSet;

use super::{company::CompanySymbol, stock::StockOwner};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum OrderType {
    Market,
    Limit { price: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub enum OrderStatus {
    Filled,
    Init,
    Pending,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Hash, Eq)]
pub struct Order {
    pub owner_id: StockOwner,
    pub order_side: OrderSide,
    pub order_type: OrderType,
    pub shares: u64,
    pub status: OrderStatus,
    pub symbol: CompanySymbol,
}

#[derive(Debug)]
pub enum OrderVerifyError {
    NoShares,
}

impl Order {
    pub fn verify(&self) -> Result<(), OrderVerifyError> {
        if self.shares == 0 {
            return Err(OrderVerifyError::NoShares);
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct CentralOrderBook(pub Vec<Order>);

impl CentralOrderBook {
    pub fn has_orders(&self, owner_id: &StockOwner) -> bool {
        self.0.iter().any(|order| order.owner_id == *owner_id)
    }

    pub fn get_matching_orders(
        &self,
        order: &Order,
        skipped: Option<&HashSet<Order>>,
    ) -> Vec<Order> {
        self.0
            .iter()
            .filter(|o| {
                o.symbol == order.symbol
                    && o.order_side != order.order_side
                    && o.owner_id != order.owner_id
                    && skipped.map_or(true, |skipped| !skipped.contains(o))
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_orders() {
        assert_eq!(
            Order {
                owner_id: StockOwner::default(),
                order_side: OrderSide::Buy,
                order_type: OrderType::Market,
                shares: 1,
                status: OrderStatus::Init,
                symbol: CompanySymbol::new("AAPL".to_string()),
            },
            Order {
                owner_id: StockOwner::default(),
                order_side: OrderSide::Buy,
                order_type: OrderType::Market,
                shares: 1,
                status: OrderStatus::Init,
                symbol: CompanySymbol::new("AAPL".to_string()),
            }
        )
    }
}
