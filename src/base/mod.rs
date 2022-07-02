use chrono::prelude::*;
use chrono::{Utc};
use serde::{Deserialize, Serialize};

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
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    interval_min: u32,
    interval_max: u32,
}

impl TimestampDef {
    pub fn new(format: String, begin: DateTime<Utc>, end: DateTime<Utc>, interval_max: u32, interval_min: u32) -> TimestampDef {
        TimestampDef {format, begin, end, interval_max, interval_min}
    }
}

pub struct Timestamp<'a> {
    def: &'a TimestampDef,
    value: DateTime<Utc>,
}

impl<'a> Timestamp<'a> {
    pub fn new(def: &TimestampDef) -> Timestamp {
        Timestamp {
            def,
            value: def.begin,
        }
    }

    pub fn next(&self) -> String {
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
