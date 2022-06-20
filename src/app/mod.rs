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

    pub fn new(name: String, handlebars: &mut Handlebars) -> AppDef {
        let tpl = "{{timestamp}} {{logger}}";
        handlebars.register_template_string(&name, tpl)
        .expect(format!("failed to register app handlebars template {}: {}", name, tpl).as_str());

        let timestamp =  TimestampDef::new(SystemTime::now(), SystemTime::now(), Duration::from_millis(10), Duration::from_secs(10));
        AppDef {
            template: tpl.to_string(),
            timestamp,
            style: StyleDef::Bunyan,
            format: FormatDef::Flat,
            loggers: vec![
                LoggerDef::new(format!("{}/{}", name, 0), "HttpFilter".to_string(), handlebars),
            ],
            name,
        }
    }

    pub fn next(&self, handlbars: &Handlebars) -> Result<String, RenderError> {
        let logger_text = {
            let mut logger_data = Map::new();
            logger_data.insert("app".to_string(), to_json(self));

            self.next_logger().next(handlbars, logger_data)?
        };

        let mut data = Map::new();
        data.insert("name".to_string(), to_json(self.name.as_str()));
        data.insert("logger".to_string(), to_json(logger_text));

        handlbars.render(&self.name, &data)
    }

    fn next_logger(&self) -> &LoggerDef {
        &self.loggers[0]
    }

    pub fn generate(&self, handlebars: &Handlebars) {
        for i in 1..10 {
            println!("{} {}", i, self.next(&handlebars).unwrap());
        }
    }

}
