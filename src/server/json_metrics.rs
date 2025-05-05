use std::collections::BTreeSet;

use serde_json::{json, Value};

use crate::{
    core::{stock_exchange::StockExchange, time::TimeHandler},
    simulation::settings::SimulationSettings,
};

pub fn build_json_metrics(
    time: TimeHandler,
    simulation_settings: SimulationSettings,
    se: StockExchange,
) -> Result<Value, String> {
    let current_year = time.get_virtual_year_formatted();
    let default_holidays = BTreeSet::new();
    let current_year_holidays = se.holidays.get(&current_year).unwrap_or(&default_holidays);

    let response = json!({
        "current_time": time.get_virtual_time_formatted(),
        "year_holidays": current_year_holidays,
        "currency": se.settings.currency,
        "simulation_settings": {
            "flush_storage": simulation_settings.flush_storage,
            "max_duration_seconds": simulation_settings.max_duration_seconds,
            "max_investor_age": simulation_settings.max_investor_age,
            "max_orders_per_tick": simulation_settings.max_orders_per_tick,
        }
    });

    Ok(response)
}
