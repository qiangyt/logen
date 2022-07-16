use anyhow::{anyhow, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::Level;
use crate::fmt::FormatterD;
use crate::template::{TemplateEngine, Template};
use crate::timestamp::{TimestampD, Timestamp};



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct MessageD {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: String,

    pub template: String,
    pub file: String,
    pub line: usize,
    pub method: String,
    pub level: Level,
}

impl MessageD {
    pub fn post_init(&mut self, id: String, tmpl: &mut TemplateEngine) -> Result<()> {
        self.id = id;
        self.with_template(tmpl)
    }

    fn with_template(&self, tmpl: &mut TemplateEngine) -> Result<()> {
        tmpl.add_template(&self.id, &self.template)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct LoggerD {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: String,

    pub name: String,
    pub message: Vec<MessageD>,
}

impl LoggerD {
    pub fn post_init(&mut self, id: String, tmpl: &mut TemplateEngine) -> Result<()> {
        self.id = id;
        self.post_init_message(tmpl)
    }

    pub fn post_init_message(&mut self, tmpl: &mut TemplateEngine) -> Result<()> {
        if self.message.len() == 0 {
            return Err(anyhow!(
                "app {} should configure at least 1 message",
                self.name
            ));
        }

        for (i, message_d) in self.message.iter_mut().enumerate() {
            let msg_id = format!("{}/{}", self.id, i);
            message_d.post_init(msg_id, tmpl)?;
        }

        return Ok(());
    }

    pub fn choose_message(&self) -> &MessageD {
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
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct AppD {
    pub name: String,
    pub formatter: FormatterD,
    pub num_of_lines: u64,
    pub timestamp: TimestampD,
    pub host: Vec<String>,
    pub logger: Vec<LoggerD>,
}

impl AppD {
    pub fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str::<Self>(yaml).expect(&format!("failed to parse config yaml: {}", yaml))
    }

    pub fn post_init(&mut self, tmpl: &mut TemplateEngine) -> Result<()> {
        self.post_init_logger(tmpl)
    }

    pub fn post_init_logger(&mut self, tmpl: &mut TemplateEngine) -> Result<()> {
        self.formatter.with_template(&self.name, tmpl)?;

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
            logger_d.post_init(logger_id, tmpl)?;
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

    pub fn choose_logger(&self) -> &LoggerD {
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
        let f = self.formatter.new_formatter();
        let mut ts = Timestamp::new(&self.timestamp, self.num_of_lines);

        for i in 0..self.num_of_lines {
            ts.inc();

            let t = &mut self.new_template(i, template_engine);
            t.set("timestamp", &f.format_timestamp(&ts));

            let l = self.choose_logger();
            let m = l.choose_message();

            self.populate_logger(t, l)?;
            self.populate_message(t, m)?;

            println!("{}", f.format(t, &self.name)?);
        }

        Ok(())
    }
    //
    fn new_template<'a>(&'a self, index: u64, template_engine: &'a TemplateEngine) -> Template {
        let mut r = Template::new(template_engine);

        r.set("app", &self.name);
        r.set("index", &index);
        r.set("host", self.choose_host());
        r.set("pid", &rand::thread_rng().gen::<u16>());

        r
    }

    fn populate_logger(&self, t: &mut Template, d: &LoggerD) -> Result<()> {
        t.set("logger", &d.name);
        Ok(())
    }

    fn populate_message(&self, t: &mut Template, d: &MessageD) -> Result<()> {
        t.set("file", &d.file);
        t.set("line", &d.line);
        t.set("method", &d.method);
        t.set(
            "level",
            match d.level {
                Level::Fine => "FINE",
                Level::Trace => "TRACE",
                Level::Debug => "DEBUG",
                Level::Info => "INFO",
                Level::Warn => "WARN",
                Level::Error => "ERROR",
                Level::Fatal => "FATAL",
            },
        );

        let msg_text = t.render(&d.id)?;
        t.set("message", &msg_text);

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        let f = serde_yaml::from_str::<FormatterD>(r#"
            flat:
                timestamp_format: "%Y-%m-%d %H:%M:%S"
                template: '{{timestamp}} <{{level | upper | align_left(width=5)}}> {{logger.name}} {{file}}/{{line}} {{method}} - {{message}}'
            json:
                style: bunyan"#).unwrap();
        //assert!(f."abc".is_err());
    }
}
