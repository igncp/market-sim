use super::{
    company::CompanySymbol, investor::InvestorId, market_maker::MarketMakerId, money::Money,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Copy)]
pub enum StockOwner {
    Investor(InvestorId),
    MarketMaker(MarketMakerId),
}

impl Serialize for StockOwner {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StockOwner::Investor(id) => {
                format!("I{}", serde_json::to_string(id).unwrap()).serialize(serializer)
            }
            StockOwner::MarketMaker(id) => {
                format!("M{}", serde_json::to_string(id).unwrap()).serialize(serializer)
            }
        }
    }
}

impl<'a> Deserialize<'a> for StockOwner {
    fn deserialize<D: serde::Deserializer<'a>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if let Some(stripped) = s.strip_prefix('I') {
            Ok(StockOwner::Investor(
                serde_json::from_str(stripped).unwrap(),
            ))
        } else if let Some(stripped) = s.strip_prefix('M') {
            Ok(StockOwner::MarketMaker(
                serde_json::from_str(stripped).unwrap(),
            ))
        } else {
            Err(serde::de::Error::custom("Invalid StockOwner"))
        }
    }
}

impl Default for StockOwner {
    fn default() -> Self {
        StockOwner::Investor(InvestorId::init())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stock {
    pub owner: StockOwner,
    pub price: Money,
    pub quantity: u64, // TODO: This can be a float if fractional shares are allowed
    pub symbol: CompanySymbol,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct OwnedStocks(pub HashMap<StockOwner, Vec<Stock>>);

impl OwnedStocks {
    pub fn has_stocks(&self, owner: &StockOwner) -> bool {
        self.0.get(owner).is_some_and(|stocks| !stocks.is_empty())
    }

    pub fn get_prices(&self, symbol: &CompanySymbol) -> Vec<Money> {
        self.0
            .values()
            .flatten()
            .filter(|stock| &stock.symbol == symbol)
            .map(|stock| stock.price)
            .collect::<Vec<_>>()
    }
}

impl OwnedStocks {
    pub fn entry_with_default(&mut self, owner: &StockOwner) -> &mut Vec<Stock> {
        self.0.entry(*owner).or_default()
    }
}
