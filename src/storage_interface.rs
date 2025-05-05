use std::collections::BTreeMap;

pub trait StorageConfigFile {
    async fn get_config_file(&self, file_name: &str) -> Result<Option<String>, String>;
}

pub trait StorageRedis {
    fn append_sorted_set(&mut self, key: &str, score: u64, value: &str) -> Result<(), String>;
    fn save_key(&mut self, key: &str, value: &str) -> Result<(), String>;
    fn load_key(&mut self, key: &str) -> Result<String, String>;
    fn flush_data(&mut self) -> Result<(), String>;
}

#[derive(Default)]
pub struct PrometheusMetric {
    pub name: String,
    pub value: f64,
    pub labels: BTreeMap<String, String>,
}
impl PrometheusMetric {
    pub fn simple(name: &str, value: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
            ..Default::default()
        }
    }
}
pub type PrometheusMetrics = Vec<PrometheusMetric>;

pub trait StoragePrometheus {
    fn get_metrics_text(&self, prefix: &str, metrics: &PrometheusMetrics)
        -> Result<String, String>;
    async fn flush_metrics(&self) -> Result<(), String>;
}
