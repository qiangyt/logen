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
            timestamp: Timestamp::new(&def.timestamp, def.num_of_lines),
            loggers: {
                let mut v = Vec::new();
                for (i, logger_d) in def.loggers.iter().enumerate() {
                    let logger_id = format!("{}/{}", def.name, i);
                    v.push(Logger::new(logger_d, logger_id));
                }
                v
            },
        })
    }

    pub fn next(&mut self, index: u64) -> Result<Option<String>> {
        let mut line = Line::new(index, &self.template, &self.timestamp.next());

        if let Some(logger) = self.choose_logger() {
            if let Some(_) = logger.next(&mut line)? {
                return Ok(Some(line.render_with_template(&self.def.name)?))
            }
        }

        Ok(None)
    }

    fn choose_logger(&self) -> Option<&Logger> {
        let mut i = 0;
        let max = self.loggers.len();
        while i < 10 {
            let index = rand::thread_rng().gen_range(0..max * 2);
            if index < max {
                return Some(&self.loggers[index]);
            }

            i = i+1;
        }

        if self.loggers.len() > 0 {
            return Some(&self.loggers[0]);
        }

        None
    }

    pub fn generate(&mut self) -> Result<()> {
        for i in 0..self.def.num_of_lines {
            if let Some(line_text) = self.next(i)? {
                println!("{}", line_text);
            } else {
                break;
            }
        }

        Ok(())
    }

}
