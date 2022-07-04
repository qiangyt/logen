use crate::def::AppDef;
use super::Logger;
use super::Line;
use super::Timestamp;
use crate::Template;
use anyhow::Result;

use rand::Rng;

pub struct App<'a> {
    def: &'a AppDef,
    timestamp: Timestamp<'a>,
    template: Template,
    loggers: Vec<Logger<'a>>,
}

impl<'a> App<'a> {

    pub fn new(def: &'a AppDef) -> Result<Self> {
        let mut template = Template::new();
        def.post_init(&mut template)?;

        Ok(App {
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
        })
    }

    pub fn next(&mut self, index: u64) -> Result<String> {
        let mut line = Line::new(index, &self.template, &self.timestamp.next());

        let logger = self.choose_logger();
        logger.next(&mut line)?;

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

    pub fn generate(&mut self) -> Result<()> {
        for i in 0..self.def.lines {
            println!("{}", self.next(i)?);
        }

        Ok(())
    }

}
