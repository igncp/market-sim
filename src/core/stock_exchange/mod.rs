use crate::core::{
    broker::Brokers,
    company::{Companies, Ipos, ListedCompanies},
    investor::Investors,
    market_maker::MarketMakers,
    money::Currency,
    order::CentralOrderBook,
    price::Prices,
    stock::OwnedStocks,
};
use serde::{Deserialize, Serialize};

mod methods;
mod order_matching;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct StockExchangeSettings {
    pub currency: Currency,
    pub location: String,
    pub name: String,
    pub timezone: String,
    pub trading_days: Vec<u8>,
    pub trading_hours: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct StockExchange {
    pub brokers: Brokers,
    pub companies: Companies,
    pub investors: Investors,
    pub ipos: Ipos,
    pub listed_companies: ListedCompanies,
    pub market_makers: MarketMakers,
    pub orders_book: CentralOrderBook,
    pub owned_stocks: OwnedStocks,
    pub prices: Prices,
    pub settings: StockExchangeSettings,
}

impl StockExchange {
    pub fn new(settings: StockExchangeSettings) -> Self {
        StockExchange {
            settings,
            ..Default::default()
        }
    }
}
