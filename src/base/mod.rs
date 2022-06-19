use std::time::{Duration, SystemTime, UNIX_EPOCH};


pub enum LevelDef {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

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

pub enum FormatDef {
    Flat,
    Json,
}

pub enum StyleDef {
    Bunyan,
}

pub struct AppContext {

}

pub struct LineContext {
    
}
