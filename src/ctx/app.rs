use crate::def::AppD;
use super::Logger;
use super::Line;
use super::Timestamp;
use crate::Template;
use anyhow::Result;

use rand::Rng;

pub struct App<'a> {
    def: &'a AppD,
    timestamp: Timestamp<'a>,
    template: Template,
    loggers: Vec<Logger<'a>>,
}

impl<'a> App<'a> {

    pub fn new(def: &'a AppD) -> Result<Self> {
        let mut template = Template::new();
        def.post_init(&mut template)?;

        Ok(App {
            def,
            template,
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

    pub fn next_line(&mut self, line_index: u64) -> Result<Line> {
        let mut line = Line::new(line_index, &self.template, &self.timestamp.next());
        self.choose_logger().render(&mut line)?;
        Ok(line)
    }

    fn choose_logger(&self) -> &Logger {
        let mut i = 0;
        let max = self.loggers.len();
        let mut rng = rand::thread_rng();

        while i < 10 {
            let index = rng.gen_range(0..max * 2);
            if index < max {
                return &self.loggers[index];
            }

            i = i+1;
        }

        return &self.loggers[0];
    }

    pub fn generate(&mut self) -> Result<()> {
        let def = self.def;

        for i in 0..def.num_of_lines {
            let line = self.next_line(i)?;
            let line_text = line.render_with_template(&def.name)?;
            println!("{}", line_text);
        }

        Ok(())
    }

}
