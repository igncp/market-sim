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

    daily_checks: Option<String>,

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
            daily_checks: None,
            price_storage,
            r,
            settings,
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

mod test {
    use super::*;
    #[allow(unused_imports)]
    use rand::Rng as _;

    struct PriceStorageMock;

    impl PriceStorage for PriceStorageMock {
        fn save_historic_price(
            &mut self,
            _prices: &Prices,
            _time_handler: &TimeHandler,
        ) -> Result<(), SaveHistoricPriceError> {
            Ok(())
        }
    }

    #[test]
    fn test_seed_is_deterministic() {
        let mut sim = Simulation::new(
            [0; 32],
            SimulationSettings::default(),
            Box::new(PriceStorageMock),
        );

        let rand_num = sim.r.gen_range(0..100);

        assert_eq!(rand_num, 41);
    }
}
