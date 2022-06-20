use std::time::{Duration, SystemTime};
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
    begin: SystemTime,
    end: SystemTime,
    interval_min: Duration,
    interval_max: Duration,
}

impl TimestampDef {
    pub fn new(begin: SystemTime, end: SystemTime, interval_max: Duration, interval_min: Duration) -> TimestampDef {
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
