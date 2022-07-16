use chrono::{DateTime, Duration, Utc};
use rand::Rng;

use crate::def::TimestampD;

pub struct Timestamp<'a> {
    def: &'a TimestampD,
    value: DateTime<Utc>,
    interval: i64,
}

impl<'a> Timestamp<'a> {
    pub fn new(def: &'a TimestampD, num_of_lines: u64) -> Self {
        let interval =
            (def.end.timestamp_millis() - def.begin.timestamp_millis()) / num_of_lines as i64;
        Timestamp {
            def,
            interval,
            value: def.begin,
        }
    }

    pub fn format(&self, format: &str) -> String {
        self.value.format(format).to_string()
    }

    pub fn inc(&mut self) {
        let mut i = 0;
        let max = self.interval;
        while i < 10 {
            let itvl = rand::thread_rng().gen_range(0..((max as f64) * 1.8) as i64);
            let new_value = self.value + Duration::milliseconds(itvl);
            if new_value < self.def.end {
                self.value = new_value;
                return;
            }

            i = i + 1;
        }

        self.value = self.def.end;
    }
}
