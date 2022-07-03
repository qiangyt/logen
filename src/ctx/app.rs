use crate::def::AppDef;
use super::Logger;
use super::Line;
use super::Timestamp;
use crate::Template;

use rand::Rng;

pub struct App<'a> {
    def: &'a AppDef,
    timestamp: Timestamp<'a>,
    template: &'a Template,
    loggers: Vec<Logger<'a>>,
}

impl<'a> App<'a> {

    pub fn new(def: &'a AppDef, template: &'a Template) -> App<'a> {
        App {
            def, template,
            timestamp: Timestamp::new(&def.timestamp, def.lines),
            loggers: {
                let mut v = Vec::new();
                for (i, logger_def) in def.loggers.iter().enumerate() {
                    let logger_id = format!("{}/{}", def.name, i);
                    v.push(Logger::new(logger_def, logger_id));
                }
                v
            },
        }
    }

    pub fn next(&mut self, index: u64) -> String {
        let mut line = Line::new(index, self.template);
        line.var("app", self.def);
        line.var("timestamp", &self.timestamp.next());

        self.choose_logger().next(&mut line);

        line.render(&self.def.name)
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
