use chrono::{Utc};
use rand::Rng;
use serde::{Serialize, Deserialize};
use serde_json::value::{Map};
use handlebars::{Handlebars, RenderError, to_json};
use super::base::*;
use super::logger::{Logger,LoggerDef};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

        let timestamp = TimestampDef::new(Utc::now(), Utc::now(), 10_000, 10);
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

    pub fn next(&self, handlebars: &Handlebars) -> Result<String, RenderError> {
        let mut data = Map::new();
        data.insert("app".to_string(), to_json(self.def));

        self.next_logger().next(&mut data, handlebars);

        handlebars.render(self.def.name.as_str(), &data)
    }

    fn next_logger(&self) -> &Logger {
        let i = rand::thread_rng().gen_range(0..self.loggers.len());
        &self.loggers[i]
    }

    pub fn generate(&self, handlebars: &Handlebars) {
        for i in 1..10 {
            println!("{} {}", i, self.next(&handlebars).unwrap());
        }
    }

}
