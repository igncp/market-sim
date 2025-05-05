use super::StockExchange;
use crate::core::{
    order::{CentralOrderBook, Order},
    time::TimeHandler,
};
use std::collections::BTreeSet;

impl StockExchange {
    pub fn can_trade_now(&self, time: &TimeHandler) -> bool {
        let num_weekday = time.get_weekday() as u8;

        if !self.settings.trading_days.contains(&num_weekday) {
            return false;
        }

        let current_day = time.get_virtual_day_formatted();
        let current_year = time.get_virtual_year_formatted();
        let default_holidays = BTreeSet::new();
        let year_holidays = self
            .holidays
            .get(&current_year)
            .unwrap_or(&default_holidays);

        if year_holidays.contains(&current_day) {
            return false;
        }

        let num_hour = time.get_day24hour() as u8;

        self.settings.trading_hours.contains(&num_hour)
    }
}

#[derive(Debug)]
pub enum PlaceOrderError {
    CantTradeNow,
    InvalidOrder,
}

impl StockExchange {
    pub fn place_order(
        &mut self,
        order: &Order,
        time: &TimeHandler,
    ) -> Result<(), PlaceOrderError> {
        if !self.can_trade_now(time) {
            return Err(PlaceOrderError::CantTradeNow);
        }

        order.verify().map_err(|_| PlaceOrderError::InvalidOrder)?;

        self.orders_book.0.push(order.clone());

        Ok(())
    }

    pub fn flush_orders(&mut self) {
        if self.orders_book.0.is_empty() {
            return;
        }

        self.orders_book = CentralOrderBook::default();
    }
}

#[cfg(test)]
mod test {
    use crate::core::stock_exchange::StockExchangeSettings;

    use super::*;

    #[test]
    fn test_can_trade_now() {
        let mut time = TimeHandler::new(0, Some(1), 100);
        let se = StockExchange::new(StockExchangeSettings {
            trading_days: vec![0, 1, 2, 3, 4],
            trading_hours: vec![9, 10, 11, 12, 13, 14, 15],
            ..Default::default()
        });

        assert_eq!(time.get_virtual_time_formatted(), "1970-01-01 08:00:00 HKT");
        assert_eq!(se.can_trade_now(&time), false);

        time.set_time(60 * 60 * 10);

        assert_eq!(time.get_virtual_time_formatted(), "1970-01-01 09:00:00 HKT");
        assert_eq!(se.can_trade_now(&time), true);
    }
}
