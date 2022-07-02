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
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
    interval_min: u32,
    interval_max: u32,
}

impl TimestampDef {
    pub fn new(begin: DateTime<Utc>, end: DateTime<Utc>, interval_max: u32, interval_min: u32) -> TimestampDef {
        TimestampDef {begin, end, interval_max, interval_min}
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
