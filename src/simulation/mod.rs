use crate::{
    core::{price::Prices, stock_exchange::StockExchange, time::TimeHandler},
    storage_interface::{StoragePrometheus, StorageRedis},
};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use settings::SimulationSettings;

mod fake_data;
mod init;
pub mod metrics;
mod run;
pub mod settings;

#[derive(Debug)]
pub enum SaveHistoricPriceError {
    Fatal(String),
    Unknown(String),
}

pub trait PriceStorage {
    fn save_historic_price(
        &mut self,
        prices: &Prices,
        time_handler: &TimeHandler,
    ) -> Result<(), SaveHistoricPriceError>;
}

pub struct Simulation {
    r: StdRng,
    price_storage: Box<dyn PriceStorage>,

    pub settings: SimulationSettings,
}

impl Simulation {
    pub fn new(
        seed: [u8; 32],
        settings: SimulationSettings,
        price_storage: Box<dyn PriceStorage>,
    ) -> Self {
        let r = StdRng::from_seed(seed);

        Simulation {
            r,
            settings,
            price_storage,
        }
    }

    pub async fn flush_data(
        redis_storage: &mut dyn StorageRedis,
        prometheus_storage: &impl StoragePrometheus,
    ) -> Result<(), String> {
        redis_storage.flush_data()?;
        prometheus_storage.flush_metrics().await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SimulationState {
    pub time: TimeHandler,
    pub se: StockExchange,
}
