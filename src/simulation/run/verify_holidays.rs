use std::collections::BTreeSet;

use crate::core::{stock_exchange::StockExchange, time::TimeHandler};
use rand::Rng;

use super::Simulation;

impl Simulation {
    pub(super) fn verify_holidays(
        &mut self,
        se: &mut StockExchange,
        time: &TimeHandler,
    ) -> Result<(), String> {
        let current_year = time.get_virtual_year_formatted();
        let current_holidays = se.holidays.get(&current_year);

        if current_holidays.is_none() {
            let mut year_holidays = BTreeSet::<String>::new();

            // https://en.wikipedia.org/wiki/List_of_countries_by_number_of_public_holidays
            let num_days = self.r.gen_range(15..=20);

            let year_weekdays = time.get_year_weekdays(&current_year);

            loop {
                let random_day_idx = self.r.gen_range(0..=year_weekdays.len() - 1);
                let random_date = year_weekdays[random_day_idx].clone();

                if year_holidays.contains(&random_date) {
                    continue;
                }

                year_holidays.insert(random_date);

                if year_holidays.len() >= num_days as usize {
                    break;
                }
            }

            se.holidays.insert(current_year.clone(), year_holidays);

            return Ok(());
        }

        Ok(())
    }
}
