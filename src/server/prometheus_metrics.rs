use std::collections::BTreeMap;

use crate::{
    core::{stock_exchange::StockExchange, time::TimeHandler},
    simulation::{
        metrics::{
            METRICS_PREFIX, METRIC_AVERAGE_STOCKS_PER_INVESTOR, METRIC_DAY_HOUR,
            METRIC_RUNNING_SIMULATION_SECONDS, METRIC_TOTAL_COMPANIES, METRIC_TOTAL_INVESTORS,
            METRIC_TOTAL_IPOS, METRIC_TOTAL_LISTED_COMPANIES, METRIC_TOTAL_MARKET_MAKERS,
            METRIC_TOTAL_STOCKS, METRIC_TRADING_NOW, METRIC_WEEKDAY,
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

    metrics.push(PrometheusMetric::simple(
        METRIC_TOTAL_INVESTORS,
        exchange.investors.mapping.len() as f64,
    ));

    metrics.push(PrometheusMetric::simple(
        METRIC_TOTAL_LISTED_COMPANIES,
        exchange.listed_companies.mapping.len() as f64,
    ));

    metrics.push(PrometheusMetric::simple(
        METRIC_TOTAL_MARKET_MAKERS,
        exchange.market_makers.mapping.len() as f64,
    ));

    metrics.push(PrometheusMetric::simple(
        METRIC_TOTAL_STOCKS,
        exchange
            .owned_stocks
            .0
            .iter()
            .map(|m| m.1.len())
            .sum::<usize>() as f64,
    ));

    metrics.push(PrometheusMetric::simple(
        METRIC_AVERAGE_STOCKS_PER_INVESTOR,
        exchange
            .owned_stocks
            .0
            .iter()
            .map(|m| m.1.len())
            .sum::<usize>() as f64
            / exchange.owned_stocks.0.len() as f64,
    ));

    metrics.push(PrometheusMetric::simple(
        METRIC_TOTAL_IPOS,
        exchange.ipos.mapping.len() as f64,
    ));

    metrics.push(PrometheusMetric::simple(
        METRIC_TRADING_NOW,
        if exchange.can_trade_now(&time) {
            1.0
        } else {
            0.0
        },
    ));

    for (company_id, price) in exchange.prices.0.iter() {
        let company = exchange.companies.mapping.get(company_id);
        if company.is_none() {
            continue;
        }
        let company = company.unwrap();
        let labels: BTreeMap<String, String> = vec![
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
