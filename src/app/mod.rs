use super::base::*;
use super::logger::{Logger,LoggerDef};

use rand::Rng;
use serde::{Serialize, Deserialize};
use tera::Tera;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppDef {
    name: String,
    template: String,
    style: StyleDef,
    lines: u32,
    format: FormatDef,
    timestamp: TimestampDef,
    loggers: Vec<LoggerDef>,
}

pub struct App<'a> {
    def: &'a AppDef,
    timestamp: Timestamp<'a>,
    loggers: Vec<Logger<'a>>,
}

impl<'a> App<'a> {

    pub fn new(def: &'a AppDef, tera: &mut Tera) -> App<'a> {
        let name = &def.name;

        tera.add_raw_template(&name, &def.template)
            .expect(format!("failed to register app template {}: {}", name, def.template).as_str());

        let timestamp = Timestamp::new(&def.timestamp, def.lines);
        App {
            def, timestamp,
            loggers: {
                let mut v = Vec::new();
                for (i, logger_def) in def.loggers.iter().enumerate() {
                    let logger_id = format!("{}/{}", name, i);
                    v.push(Logger::new(logger_def, logger_id, tera));
                }
                v
            }
        }
    }

    pub fn next(&mut self, tera: &Tera) -> tera::Result<String> {
        let mut data = tera::Context::new();
        data.insert("app", self.def);
        data.insert("timestamp", &self.timestamp.next());

        self.choose_logger().next(&mut data, tera);

        tera.render(self.def.name.as_str(), &data)
    }

    fn choose_logger(&self) -> &Logger {
        let mut i = 0;
        let max = self.loggers.len();
        while i < 10 {
            let index = rand::thread_rng().gen_range(0..max * 2);
            if index < max {
                return &self.loggers[index];
            }

            i = i+1;
        }

        return &self.loggers[0];
    }

    pub fn generate(&mut self, tera: &Tera) {
        for i in 0..self.def.lines {
            println!("{}", self.next(&tera).unwrap());
        }
    }

}
