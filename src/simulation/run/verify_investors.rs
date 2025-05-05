use crate::core::{stock_exchange::StockExchange, time::TimeHandler};
use log::debug;
use rand::Rng;

use super::Simulation;

impl Simulation {
    pub(super) fn verify_investors(
        &mut self,
        se: &mut StockExchange,
        time: &TimeHandler,
    ) -> Result<(), String> {
        // - TODO: Check if any investor is too poor
        let mut investors_to_remove = Vec::new();

        for investor in se.investors.mapping.values() {
            let investor_age = investor.get_age(time);

            if investor_age > self.settings.max_investor_age as f64 {
                investors_to_remove.push(investor.id);
                debug!(
                    "Removed investor in simulation because too old: {:?}",
                    investor.id
                );
                continue;
            }

            // @settings
            let age_death_rate = (0.25 * investor_age / 365.0).ceil() as u32;

            let is_investor_dead = self.r.gen_ratio(age_death_rate, 100);

            if is_investor_dead {
                investors_to_remove.push(investor.id);
                debug!(
                    "Removed investor in simulation because dead: {:?}",
                    investor.id
                );
                continue;
            }
        }

        for investor_id in investors_to_remove {
            se.investors.mapping.remove(&investor_id);
        }

        let investors_to_add = self.r.gen_range(0..=10) - 7;
        if investors_to_add > 0 {
            for _ in 0..investors_to_add {
                let new_investor = self.create_valid_new_investor(se, time)?;

                se.investors.mapping.insert(new_investor.id, new_investor);
            }
        }

        Ok(())
    }
}
