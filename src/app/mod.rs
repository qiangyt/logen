use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::base::*;//  ::base::{TimestampField, LogStyle, LogFormat};
use super::logger::LoggerDef;


pub struct AppDef {
    name: String,
    timestamp: TimestampDef,
    style: StyleDef,
    format: FormatDef,
    loggers: Vec<LoggerDef>,
}

impl AppDef {
    
    pub fn new(name: String) -> AppDef {
        let timestamp =  TimestampDef::new(SystemTime::now(), SystemTime::now(), Duration::from_millis(10), Duration::from_secs(10));
        AppDef {
            name,
            timestamp,
            style: StyleDef::Bunyan,
            format: FormatDef::Flat,
            loggers: vec![
                LoggerDef::new("HttpFilter".to_string()),
            ]
        }
    }

    pub fn next(&mut self) -> String {
        let logger = &self.loggers[0];
        logger.next()
    }

    pub fn generate(&mut self) {
        for i in 1..10 {
            println!("{}: {}", i, self.next());
        }
    }

}
