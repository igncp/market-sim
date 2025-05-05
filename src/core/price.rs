use super::{company::CompanySymbol, money::Money};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Price {
    pub ask: Money,
    pub bid: Money,
}

impl Price {
    pub fn get_average(&self) -> Money {
        let value = (self.ask.value + self.bid.value) / Money::from_f64(2.0);
        Money {
            currency: self.ask.currency,
            value,
        }
    }

    pub fn get_with_spread(&self, value: Decimal, spread: Decimal) -> Price {
        Price {
            ask: Money {
                currency: self.ask.currency,
                value: value + spread,
            },
            bid: Money {
                currency: self.bid.currency,
                value: (value - spread).abs(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Prices(pub BTreeMap<CompanySymbol, Price>);

impl Prices {
    pub fn get_lowest_bid_price(&self) -> Option<&Money> {
        self.0
            .values()
            .map(|price| &price.bid)
            .min_by(|a, b| a.value.partial_cmp(&b.value).unwrap())
    }

    pub fn get_average_price(&self, symbol: &CompanySymbol) -> Option<Money> {
        self.0.get(symbol).map(|price| price.get_average())
    }

    pub fn get_ask_price(&self, symbol: &CompanySymbol) -> Option<&Money> {
        self.0.get(symbol).map(|price| &price.ask)
    }
}
