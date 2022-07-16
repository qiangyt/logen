use crate::{def::AppD, formatter::Formatter, template::Template};
use anyhow::Result;

use rand::Rng;

use super::{timestamp::Timestamp, logger::Logger, line::Line};

pub struct App<'a> {
    def: &'a AppD,
    formatter: Box<dyn Formatter + 'a>,
    template: Template,
    loggers: Vec<Logger<'a>>,
}

impl <'a> App<'a> {

    pub fn new(def: &'a AppD) -> Result<App<'a>> {
        let mut template = Template::new();
        def.post_init(&mut template)?;

        Ok(App {
            def,
            formatter: def.formatter.new_formatter(),
            template,
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

    pub fn name(&self) -> &str {
        &self.def.name
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
        let formatter = self.formatter.as_ref();
        let mut timestamp = Timestamp::new(&def.timestamp, def.num_of_lines);

        for i in 0..def.num_of_lines {
            unsafe {
            let line = self.new_line(i, &mut timestamp)?;
            let line_text = formatter.format(&line)?;
            println!("{}", line_text);
            }
        }

        Ok(())
    }

    pub fn new_line(&self, line_index: u64, timestamp: &mut Timestamp) -> Result<Line> {
        let timestamp = timestamp.next();
        let mut line = Line::new(line_index, self, &self.template, &timestamp);
        self.choose_logger().render(&mut line)?;
        Ok(line)
    }

}
