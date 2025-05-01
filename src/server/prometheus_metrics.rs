use std::collections::HashMap;

use crate::{
    core::{stock_exchange::StockExchange, time::TimeHandler},
    simulation::{
        metrics::{
            METRICS_PREFIX, METRIC_DAY_HOUR, METRIC_RUNNING_SIMULATION_SECONDS,
            METRIC_TOTAL_COMPANIES, METRIC_WEEKDAY,
        },
        settings::SimulationSettings,
    },
    storage::prometheus::StoragePrometheusImpl,
    storage_interface::{PrometheusMetric, PrometheusMetrics, StoragePrometheus},
};

pub fn build_server_prometheus_metrics(
    time: TimeHandler,
    exchange: StockExchange,
    simulation_settings: SimulationSettings,
) -> Result<String, String> {
    let prometheus_storage: StoragePrometheusImpl = simulation_settings.into();
    let mut metrics: PrometheusMetrics = Default::default();

    metrics.push(PrometheusMetric::simple(
        METRIC_WEEKDAY,
        time.get_weekday() as f64,
    ));
    metrics.push(PrometheusMetric::simple(
        METRIC_DAY_HOUR,
        time.get_day24hour() as f64,
    ));
    metrics.push(PrometheusMetric::simple(
        METRIC_TOTAL_COMPANIES,
        exchange.companies.mapping.len() as f64,
    ));

    metrics.push(PrometheusMetric::simple(
        METRIC_RUNNING_SIMULATION_SECONDS,
        time.get_running_seconds() as f64,
    ));

    for (company_id, price) in exchange.prices.0.iter() {
        let company = exchange.companies.mapping.get(company_id);
        if company.is_none() {
            continue;
        }
        let company = company.unwrap();
        let labels: HashMap<String, String> = vec![
            ("name".to_string(), company.name.clone()),
            ("symbol".to_string(), company.symbol.0.clone()),
        ]
        .into_iter()
        .collect();

        metrics.push(PrometheusMetric {
            name: "price_ask".to_string(),
            value: price.ask.to_f64(),
            labels,
        });
    }

    let metrics_text = prometheus_storage.get_metrics_text(METRICS_PREFIX, &metrics)?;

    return Ok(metrics_text);
}
