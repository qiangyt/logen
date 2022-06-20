use std::time::{Duration, SystemTime};
use serde::{Serialize, Deserialize};
use serde_json::value::{Map};
use handlebars::{Handlebars, RenderError, to_json};
use super::base::*;
use super::logger::{Logger,LoggerDef};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppDef {
    name: String,
    template: String,
    style: StyleDef,
    format: FormatDef,
    loggers: Vec<LoggerDef>,
}

pub struct App<'a> {
    def: &'a AppDef,
    timestamp: TimestampDef,
    loggers: Vec<Logger<'a>>,
}

impl<'a> App<'a> {

    pub fn new(def: &'a AppDef, handlebars: &mut Handlebars) -> App<'a> {
        let name = &def.name;

        handlebars.register_template_string(&name, &def.template)
        .expect(format!("failed to register app handlebars template {}: {}", name, def.template).as_str());

        let timestamp = TimestampDef::new(SystemTime::now(), SystemTime::now(), Duration::from_millis(10), Duration::from_secs(10));
        App {
            def, timestamp,
            loggers: {
                let mut v = Vec::new();
                for (i, logger_def) in def.loggers.iter().enumerate() {
                    let logger_id = format!("{}/{}", name, i);
                    v.push(Logger::new(logger_def, logger_id, handlebars));
                }
                v
            }
        }
    }

    pub fn next(&self, handlbars: &Handlebars) -> Result<String, RenderError> {
        let def = &self.def;

        let logger_text = {
            let mut logger_data = Map::new();
            logger_data.insert("app".to_string(), to_json(def));

            self.next_logger().next(handlbars, logger_data)?
        };

        let mut data = Map::new();
        data.insert("name".to_string(), to_json(def.name.as_str()));
        data.insert("logger".to_string(), to_json(logger_text));

        handlbars.render(&def.name, &data)
    }

    fn next_logger(&self) -> &Logger {
        &self.loggers[0]
    }

    pub fn generate(&self, handlebars: &Handlebars) {
        for i in 1..10 {
            println!("{} {}", i, self.next(&handlebars).unwrap());
        }
    }

}
