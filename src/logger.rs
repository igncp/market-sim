use log::{Level, LevelFilter, Metadata, Record};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Self
    }

    pub fn setup_level(&self, lvl: &LevelFilter) {
        log::set_max_level(*lvl);

        log::set_boxed_logger(Box::new(SimpleLogger))
            .map(|()| log::set_max_level(LevelFilter::Info))
            .unwrap();
    }
}
