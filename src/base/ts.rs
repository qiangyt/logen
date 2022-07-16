use chrono::{DateTime, Duration, Utc};
use rand::Rng;

#[derive(Debug)]
pub struct Timestamp {
    end: DateTime<Utc>,
    current: DateTime<Utc>,
    interval: i64,
}

impl Timestamp {
    pub fn new(begin: &DateTime<Utc>, end: &DateTime<Utc>, num_of_lines: u64) -> Self {
        Timestamp {
            interval: (end.timestamp_millis() - begin.timestamp_millis()) / num_of_lines as i64,
            current: *begin,
            end: end.clone(),
        }
    }

    pub fn format(&self, format: &str) -> String {
        self.current.format(format).to_string()
    }

    pub fn inc(&mut self) {
        let mut i = 0;
        let max = self.interval;
        while i < 10 {
            let itvl = rand::thread_rng().gen_range(0..((max as f64) * 1.8) as i64);
            let new_value = self.current + Duration::milliseconds(itvl);
            if new_value < self.end {
                self.current = new_value;
                return;
            }

            i = i + 1;
        }

        self.current = self.end;
    }
}
