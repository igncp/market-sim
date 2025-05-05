#[cfg(test)]
mod test {
    use crate::core::time::TimeHandler;

    impl TimeHandler {
        pub fn set_time(&mut self, time: u64) {
            self.time = time;
        }
    }

    mod get_time_running {
        use crate::core::time::TimeHandler;

        #[test]
        fn returns_correct_values() {
            let mut time_handler = TimeHandler::new(0, None, 100);

            time_handler.millis_to_wait = 1000;

            time_handler.time = 10;
            assert_eq!(time_handler.get_time_running(), "10s");

            time_handler.time = 30;
            assert_eq!(time_handler.get_time_running(), "30s");

            time_handler.time = 90;
            assert_eq!(time_handler.get_time_running(), "1m30s");

            time_handler.time = 3600 + 6 * 60 + 40;
            assert_eq!(time_handler.get_time_running(), "1h6m40s");
        }
    }
}
