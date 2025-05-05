use crate::storage_interface::StorageConfigFile;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct SimulationSettings {
    pub address: String,
    pub flush_storage: bool,
    pub max_duration_seconds: Option<u64>,
    pub max_investor_age: u64,
    pub max_orders_per_tick: u64,
    pub port: String,
    pub prometheus_job_name: String,
    pub prometheus_url: String,
    pub redis_url: String,
    pub time_to_wait_millis: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SimulationSettingsBuilder {
    pub address: Option<String>,
    pub flush_storage: Option<bool>,
    pub max_duration_seconds: Option<u64>,
    pub max_investor_age: Option<u64>,
    pub max_orders_per_tick: Option<u64>,
    pub port: Option<String>,
    pub prometheus_job_name: Option<String>,
    pub prometheus_url: Option<String>,
    pub redis_url: Option<String>,
    pub time_to_wait_millis: Option<u64>,
}

const SETTINGS_FILE_NAME: &str = "market-sim-settings.json";

const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_MAX_INVESTOR_AGE: u64 = 100;
const DEFAULT_ORDERS_PER_TICK: u64 = 4000;
const DEFAULT_PROMETHEUS_JOB_NAME: &str = "market-sim";
const DEFAULT_PROMETHEUS_URL: &str = "http://localhost:9090";
const DEFAULT_REDIS_URL: &str = "redis://127.0.0.1";
const DEFAULT_TIME_TO_WAIT: u64 = 1000;
pub const DEFAULT_PORT: &str = "9000";

impl SimulationSettingsBuilder {
    fn merge(&self, other: &Self) -> Self {
        Self {
            address: other.address.clone().or(self.address.clone()),
            flush_storage: other.flush_storage.or(self.flush_storage),
            max_orders_per_tick: other.max_orders_per_tick.or(self.max_orders_per_tick),
            port: other.port.clone().or(self.port.clone()),
            max_investor_age: other.max_investor_age.or(self.max_investor_age),
            redis_url: other.redis_url.clone().or(self.redis_url.clone()),
            prometheus_job_name: other
                .prometheus_job_name
                .clone()
                .or(self.prometheus_job_name.clone()),
            prometheus_url: other.prometheus_url.clone().or(self.prometheus_url.clone()),
            max_duration_seconds: other.max_duration_seconds.or(self.max_duration_seconds),
            time_to_wait_millis: other.time_to_wait_millis.or(self.time_to_wait_millis),
        }
    }

    pub async fn load_from_storage(
        &self,
        config_file: &impl StorageConfigFile,
    ) -> Result<SimulationSettings, String> {
        let file_settings = config_file
            .get_config_file(SETTINGS_FILE_NAME)
            .await?
            .map(|s| {
                serde_json::from_str::<Self>(&s)
                    .map_err(|e| format!("Error parsing settings: {}", e))
            });

        let mut builder = self.clone();

        if let Some(file_settings) = file_settings {
            let file_settings = file_settings?;
            builder = builder.merge(&file_settings);
        }

        Ok(builder.into())
    }
}

impl From<SimulationSettingsBuilder> for SimulationSettings {
    fn from(builder: SimulationSettingsBuilder) -> Self {
        Self {
            address: builder.address.unwrap_or(DEFAULT_ADDRESS.to_string()),
            flush_storage: builder.flush_storage.unwrap_or(false),
            max_orders_per_tick: builder
                .max_orders_per_tick
                .unwrap_or(DEFAULT_ORDERS_PER_TICK),
            port: builder.port.unwrap_or(DEFAULT_PORT.to_string()),
            max_investor_age: builder.max_investor_age.unwrap_or(DEFAULT_MAX_INVESTOR_AGE),
            prometheus_job_name: builder
                .prometheus_job_name
                .unwrap_or(DEFAULT_PROMETHEUS_JOB_NAME.to_string()),
            prometheus_url: builder
                .prometheus_url
                .unwrap_or(DEFAULT_PROMETHEUS_URL.to_string()),
            redis_url: builder.redis_url.unwrap_or(DEFAULT_REDIS_URL.to_string()),
            max_duration_seconds: builder.max_duration_seconds,
            time_to_wait_millis: builder.time_to_wait_millis.unwrap_or(DEFAULT_TIME_TO_WAIT),
        }
    }
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            address: DEFAULT_ADDRESS.to_string(),
            flush_storage: false,
            max_duration_seconds: None,
            max_investor_age: DEFAULT_MAX_INVESTOR_AGE,
            max_orders_per_tick: DEFAULT_ORDERS_PER_TICK,
            port: DEFAULT_PORT.to_string(),
            prometheus_job_name: DEFAULT_PROMETHEUS_JOB_NAME.to_string(),
            prometheus_url: DEFAULT_PROMETHEUS_URL.to_string(),
            redis_url: DEFAULT_REDIS_URL.to_string(),
            time_to_wait_millis: DEFAULT_TIME_TO_WAIT,
        }
    }
}
