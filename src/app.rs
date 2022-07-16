use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::tpl::{Template, TemplateEngine};
use crate::ts::Timestamp;
use crate::{Level, Output};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Message {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: String,

    pub template: String,
    pub file: String,
    pub line: usize,
    pub method: String,
    pub level: Level,
}

impl Message {
    pub fn init(&mut self, id: String, tmpl: &mut TemplateEngine) -> Result<()> {
        self.id = id;
        self.with_template(tmpl)
    }

    fn with_template(&self, tmpl: &mut TemplateEngine) -> Result<()> {
        tmpl.add_template(&self.id, &self.template)
    }

    fn populate(&self, t: &mut Template) -> Result<()> {
        t.set("file", &self.file);
        t.set("line", &self.line);
        t.set("method", &self.method);
        t.set(
            "level",
            match self.level {
                Level::Fine => "FINE",
                Level::Trace => "TRACE",
                Level::Debug => "DEBUG",
                Level::Info => "INFO",
                Level::Warn => "WARN",
                Level::Error => "ERROR",
                Level::Fatal => "FATAL",
            },
        );

        let msg_text = t.render(&self.id)?;
        t.set("message", &msg_text);

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Logger {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: String,

    pub name: String,
    pub message: Vec<Message>,
}

impl Logger {
    pub fn init(&mut self, id: String, tmpl: &mut TemplateEngine) -> Result<()> {
        self.id = id;
        self.init_message(tmpl)
    }

    pub fn init_message(&mut self, tmpl: &mut TemplateEngine) -> Result<()> {
        if self.message.len() == 0 {
            return Err(anyhow!(
                "app {} should configure at least 1 message",
                self.name
            ));
        }

        for (i, message_d) in self.message.iter_mut().enumerate() {
            let msg_id = format!("{}/{}", self.id, i);
            message_d.init(msg_id, tmpl)?;
        }

        return Ok(());
    }

    pub fn choose_message(&self) -> &Message {
        let mut i = 0;
        let max = self.message.len();
        let mut rng = rand::thread_rng();

        while i < 10 {
            let index = rng.gen_range(0..max * 2);
            if index < max {
                return &self.message[index];
            }

            i = i + 1;
        }

        &self.message[0]
    }

    fn populate(&self, t: &mut Template) -> Result<()> {
        t.set("logger", &self.name);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct App {
    name: String,
    output: Output,
    num_of_lines: u64,
    begin_time: DateTime<Utc>, //rfc3339
    end_time: DateTime<Utc>,
    host: Vec<String>,
    logger: Vec<Logger>,
}

impl App {
    pub fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str::<Self>(yaml).expect(&format!("failed to parse config yaml: {}", yaml))
    }

    pub fn init(&mut self, tmpl: &mut TemplateEngine) -> Result<()> {
        self.init_logger(tmpl)
    }

    pub fn init_logger(&mut self, tmpl: &mut TemplateEngine) -> Result<()> {
        self.output.with_template(&self.name, tmpl)?;

        if self.host.len() == 0 {
            return Err(anyhow!(
                "app {} should configure at least 1 host",
                self.name
            ));
        }

        if self.logger.len() == 0 {
            return Err(anyhow!(
                "app {} should configure at least 1 logger",
                self.name
            ));
        }

        for (i, logger_d) in self.logger.iter_mut().enumerate() {
            let logger_id = format!("{}/{}", self.name, i);
            logger_d.init(logger_id, tmpl)?;
        }

        return Ok(());
    }

    pub fn choose_host(&self) -> &str {
        let mut k = 0;
        let max = self.host.len();
        let mut rng = rand::thread_rng();

        while k < 10 {
            let i = rng.gen_range(0..max * 2);
            if i < max {
                return &self.host[i];
            }

            k = k + 1;
        }

        return &self.host[0];
    }

    pub fn choose_logger(&self) -> &Logger {
        let mut k = 0;
        let max = self.logger.len();
        let mut rng = rand::thread_rng();

        while k < 10 {
            let i = rng.gen_range(0..max * 2);
            if i < max {
                return &self.logger[i];
            }

            k = k + 1;
        }

        return &self.logger[0];
    }

    pub fn generate(&self, template_engine: &TemplateEngine) -> Result<()> {
        let f = self.output.formatter();
        let mut ts = Timestamp::new(&self.begin_time, &self.end_time, self.num_of_lines);

        for i in 0..self.num_of_lines {
            ts.inc();

            let t = &mut self.new_template(i, template_engine);
            t.set("timestamp", &f.format_timestamp(&ts));

            let logger = self.choose_logger();
            logger.populate(t)?;
            logger.choose_message().populate(t)?;

            println!("{}", f.format(t, &self.name)?);
        }

        Ok(())
    }

    fn new_template<'a>(&'a self, index: u64, template_engine: &'a TemplateEngine) -> Template {
        let mut r = Template::new(template_engine);

        r.set("app", &self.name);
        r.set("index", &index);
        r.set("host", self.choose_host());
        r.set("pid", &rand::thread_rng().gen::<u16>());

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        let f = serde_yaml::from_str::<Output>(r#"
            flat:
                timestamp_format: "%Y-%m-%d %H:%M:%S"
                template: '{{timestamp}} <{{level | upper | align_left(width=5)}}> {{logger.name}} {{file}}/{{line}} {{method}} - {{message}}'
            json:
                style: bunyan"#).unwrap();
        //assert!(f."abc".is_err());
    }
}
