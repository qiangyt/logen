use chrono::prelude::*;
use chrono::{Utc, Duration};
use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LevelDef {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimestampDef {
    format: String,
    begin: DateTime<Utc>,//rfc3339
    end: DateTime<Utc>,
}

impl TimestampDef {
    pub fn new(format: String, begin: DateTime<Utc>, end: DateTime<Utc>) -> TimestampDef {
        TimestampDef {format, begin, end}
    }
}

pub struct Timestamp<'a> {
    def: &'a TimestampDef,
    value: DateTime<Utc>,
    interval: i64,
}

impl<'a> Timestamp<'a> {
    pub fn new(def: &TimestampDef, lines: u32) -> Timestamp {
        Timestamp {
            def,
            value: def.begin,
            interval: (def.end.timestamp_millis() - def.begin.timestamp_millis()) / lines as i64,
        }
    }

    pub fn next(&mut self) -> String {
        let i = rand::thread_rng().gen_range(0..self.interval);
        self.value = self.value + Duration::milliseconds(i);
        self.value.format(self.def.format.as_str()).to_string()
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormatDef {
    Flat,
    Json,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StyleDef {
    Bunyan,
}

pub struct AppContext {

}

pub struct LineContext {

}
