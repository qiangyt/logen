use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};
use serde_json::value::{Map};
use handlebars::{Handlebars, RenderError, to_json};
use super::base::*;
use super::logger::LoggerDef;

#[derive(Serialize, Deserialize)]
pub struct AppDef {
    name: String,
    template: String,
    timestamp: TimestampDef,
    style: StyleDef,
    format: FormatDef,
    loggers: Vec<LoggerDef>,
}

impl AppDef {
    
    pub fn new(name: String) -> AppDef {
        let timestamp =  TimestampDef::new(SystemTime::now(), SystemTime::now(), Duration::from_millis(10), Duration::from_secs(10));
        AppDef {
            template: "{{timestamp}} {{logger}}".to_string(),
            timestamp,
            style: StyleDef::Bunyan,
            format: FormatDef::Flat,
            loggers: vec![
                LoggerDef::new(format!("{}/{}", name, 0), "HttpFilter".to_string()),
            ],
            name, 
        }
    }

    pub fn next(&self, handlbars: &Handlebars) -> Result<String, RenderError> {
        let loggerText = {
            let mut loggerData = Map::new();
            loggerData.insert("app".to_string(), to_json(self));

            self.nextLogger().next(handlbars, loggerData)?
        };

        let data = Map::new();
        data.insert("name".to_string(), to_json(self.name));
        data.insert("logger".to_string(), to_json(loggerText));

        handlbars.render(&self.id, &data)
    }

    fn nextLogger(&self) -> &LoggerDef {
        &self.loggers[0]
    }

    pub fn generate(&self) {
        let handlebars = Handlebars::new();
        for i in 1..10 {
            println!("{} {}", i, self.next(&handlebars).unwrap());
        }
    }

}
