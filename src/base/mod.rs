use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum LevelDef {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
#[derive(Serialize, Deserialize)]
pub struct TimestampDef {
    begin: SystemTime,
    end: SystemTime,
    intervalMin: Duration,
    intervalMax: Duration,
}

impl TimestampDef {
    pub fn new(begin: SystemTime, end: SystemTime, intervalMax: Duration, intervalMin: Duration) -> TimestampDef {
        TimestampDef {begin, end, intervalMax, intervalMin}
    }
}


#[derive(Serialize, Deserialize)]
pub enum FormatDef {
    Flat,
    Json,
}


#[derive(Serialize, Deserialize)]
pub enum StyleDef {
    Bunyan,
}

pub struct AppContext {

}

pub struct LineContext {
    
}
