use chrono::{DateTime, Datelike, TimeZone, Timelike};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

mod test;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Timezone {
    Utc,
    Hk,
}

impl Timezone {
    pub fn to_string(&self) -> String {
        match self {
            Timezone::Utc => "UTC".to_string(),
            Timezone::Hk => "UTC+08".to_string(),
        }
    }

    pub fn get_tz(&self) -> Tz {
        match self {
            Timezone::Utc => chrono_tz::UTC,
            Timezone::Hk => chrono_tz::Asia::Hong_Kong,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TimeHandler {
    pub initial_time: u64,
    /** How many seconds in real life is one second in the simulation */
    pub secs_factor: u64,
    pub tz: Timezone,

    millis_to_wait_millis: u64,
    time: u64,
}

pub const DEFAULT_TIMEZONE: Timezone = Timezone::Hk;

// Pure
impl TimeHandler {
    pub fn new(initial_time: u64, secs_factor: Option<u64>, millis_to_wait_millis: u64) -> Self {
        let secs_factor = secs_factor.unwrap_or(/* 45 minutes */ 60 * 45);

        TimeHandler {
            initial_time,
            millis_to_wait_millis,
            secs_factor,
            time: 0,
            tz: DEFAULT_TIMEZONE,
        }
    }

    pub fn get_n_days_from_now_unix_timestamp(&self, n: u64) -> u64 {
        let now = self.get_now_unix_timestamp();
        now + n * 24 * 60 * 60
    }

    pub fn get_now_unix_timestamp(&self) -> u64 {
        let factor = self.millis_to_wait_millis as f64 / 1000.0;

        (factor * (self.time as f64) * (self.secs_factor as f64) + (self.initial_time as f64))
            as u64
    }

    pub fn get_running_seconds(&self) -> u64 {
        let millis_span = self.time * self.millis_to_wait_millis;
        let seconds_passed =
            chrono::Duration::milliseconds(millis_span as i64).num_seconds() as u64;

        seconds_passed
    }

    fn get_virtual_time(&self) -> DateTime<Tz> {
        let now = self.get_now_unix_timestamp();
        let tz = &self.tz.get_tz();

        tz.timestamp_opt(now as i64, 0).unwrap()
    }

    pub fn get_weekday(&self) -> u32 {
        self.get_virtual_time().weekday().num_days_from_monday()
    }

    pub fn get_year_weekdays(&self, year: &str) -> Vec<String> {
        let tz = &self.tz.get_tz();
        let mut weekdays = Vec::new();
        let year_num = year.parse::<i32>().unwrap();

        for i in 0..=366 {
            let date =
                tz.with_ymd_and_hms(year_num, 1, 1, 0, 0, 0).unwrap() + chrono::Duration::days(i);
            let weekday = date.weekday().num_days_from_monday();
            let year = date.year();

            if weekday == 5 || weekday == 6 || year != year_num {
                continue;
            }

            weekdays.push(format!("{}", date.format("%Y-%m-%d")));
        }

        weekdays
    }

    pub fn get_day24hour(&self) -> u32 {
        self.get_virtual_time().hour()
    }

    pub fn get_virtual_day_formatted(&self) -> String {
        let date = self.get_virtual_time();

        format!("{}", date.format("%Y-%m-%d"))
    }

    pub fn get_virtual_year_formatted(&self) -> String {
        let date = self.get_virtual_time();

        format!("{}", date.format("%Y"))
    }

    pub fn get_virtual_time_formatted(&self) -> String {
        self.get_virtual_time().to_string()
    }

    pub fn get_time_running(&self) -> String {
        let millis_span = self.time * self.millis_to_wait_millis;
        let seconds_passed =
            chrono::Duration::milliseconds(millis_span as i64).num_seconds() as u64;

        let hours = if seconds_passed >= 3600 {
            let hours = seconds_passed / 3600;
            format!("{}h", hours)
        } else {
            "".to_string()
        };
        let minutes = if seconds_passed >= 60 {
            let minutes = (seconds_passed % 3600) / 60;
            format!("{}m", minutes)
        } else {
            "".to_string()
        };
        let seconds = if seconds_passed >= 1 {
            let seconds = seconds_passed % 60;
            format!("{}s", seconds)
        } else {
            "".to_string()
        };

        format!("{}{}{}", hours, minutes, seconds)
    }

    pub fn get_wait_millis(&self) -> u64 {
        self.millis_to_wait_millis
    }
}

// Impure
impl TimeHandler {
    pub fn tick(&mut self) {
        self.time += 1;
    }
}
