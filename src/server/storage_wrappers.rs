use super::SimulationState;
use crate::{
    core::{price::Prices, time::TimeHandler},
    simulation::{settings::SimulationSettings, PriceStorage, SaveHistoricPriceError},
    storage::{prometheus::StoragePrometheusImpl, redis::StorageRedisImpl},
    storage_interface::StorageRedis,
};

pub fn save_simulation_state(
    redis: &mut StorageRedisImpl,
    state: &SimulationState,
) -> Result<(), String> {
    let state_str =
        serde_json::to_string(state).map_err(|e| format!("Failed to serialize state: {}", e))?;

    redis.save_key("simulation_state", &state_str)?;

    Ok(())
}

pub enum LoadSimulationStateError {
    Empty,
    Unknown(String),
}

pub fn load_simulation_state(
    settings: &SimulationSettings,
) -> Result<SimulationState, LoadSimulationStateError> {
    let mut redis: StorageRedisImpl = settings.clone().into();
    let state_str = redis
        .load_key("simulation_state")
        .map_err(|_| LoadSimulationStateError::Empty)?;

    let state = serde_json::from_str(&state_str).map_err(|e| {
        LoadSimulationStateError::Unknown(format!("Failed to deserialize state: {}", e))
    })?;

    Ok(state)
}

pub struct RedisPriceStorage {
    redis: StorageRedisImpl,
}

impl RedisPriceStorage {
    pub fn new(settings: &SimulationSettings) -> Box<Self> {
        let redis: StorageRedisImpl = settings.clone().into();

        Box::new(Self { redis })
    }
}

impl PriceStorage for RedisPriceStorage {
    fn save_historic_price(
        &mut self,
        prices: &Prices,
        time: &TimeHandler,
    ) -> Result<(), SaveHistoricPriceError> {
        let time = time.get_now_unix_timestamp();

        for (symbol, price) in prices.0.iter() {
            let symbol_str = symbol.to_string();

            self.redis
                .append_sorted_set(
                    &format!("price:{}", symbol_str),
                    time,
                    &format!("{},{}", time, price.get_average().value),
                )
                .map_err(SaveHistoricPriceError::Unknown)?;
        }

        Ok(())
    }
}

impl From<SimulationSettings> for StoragePrometheusImpl {
    fn from(settings: SimulationSettings) -> Self {
        Self {
            job_name: settings.prometheus_job_name.clone(),
            url: settings.prometheus_url.clone(),
        }
    }
}

impl From<SimulationSettings> for StorageRedisImpl {
    fn from(settings: SimulationSettings) -> Self {
        Self::new(&settings.redis_url)
    }
}
