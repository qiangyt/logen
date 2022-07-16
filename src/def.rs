use anyhow::{anyhow, Result};
use chrono::prelude::*;
use chrono::Utc;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::formatter::flat::FlatFormatter;
use crate::formatter::json::JsonFormatter;
use crate::formatter::FlatFormatterD;
use crate::formatter::Formatter;
use crate::formatter::JsonFormatterD;
use crate::template::TemplateEngine;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Level {
    Fine,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum FormatterD {
    Flat(FlatFormatterD),
    Json(JsonFormatterD),
}

impl FormatterD {
    pub fn with_template(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        match self {
            FormatterD::Flat(flat) => flat.with_template(tmpl_name, tmpl),
            FormatterD::Json(_) => Ok(()),
        }
    }

    pub fn new_formatter<'a>(&'a self) -> Box<dyn Formatter + 'a> {
        match self {
            FormatterD::Flat(f) => Box::new(FlatFormatter::new(f)),
            FormatterD::Json(j) => Box::new(JsonFormatter::new(j)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct TimestampD {
    pub begin: DateTime<Utc>, //rfc3339
    pub end: DateTime<Utc>,
}

impl TimestampD {
    pub fn new(begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        TimestampD { begin, end }
    }
}

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
