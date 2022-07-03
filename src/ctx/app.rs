use crate::def::AppDef;
use super::Logger;
use super::Line;
use super::Timestamp;
use crate::Template;

use rand::Rng;

pub struct App<'a> {
    def: &'a AppDef,
    timestamp: Timestamp<'a>,
    template: Template,
    loggers: Vec<Logger<'a>>,
}

impl<'a> App<'a> {

    pub fn new(def: &'a AppDef) -> App<'a> {
        let name = &def.name;

        let mut template = Template::new();
        template.add_raw_template(&name, &def.template);

        let loggers = {
            let mut v = Vec::new();
            for (i, logger_def) in def.loggers.iter().enumerate() {
                let logger_id = format!("{}/{}", name, i);
                v.push(Logger::new(logger_def, logger_id, &mut template));
            }
            v
        };

        let timestamp = Timestamp::new(&def.timestamp, def.lines);
        App {
            def, timestamp, template, loggers,
        }
    }

    pub fn next(&mut self, index: u64) -> String {
        let mut data = tera::Context::new();
        data.insert("app", self.def);
        data.insert("timestamp", &self.timestamp.next());

        let mut line = Line::new(index, &mut data, &self.template);
        self.choose_logger().next(&mut line);

        self.template.render(&self.def.name, &data)
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

    pub fn generate(&mut self) {
        for i in 0..self.def.lines {
            println!("{}", self.next(i));
        }
    }

}
