
use chrono::prelude::*;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use anyhow::{anyhow, Result};

use crate::formatter::Formatter;
use crate::formatter::flat::FlatFormatter;
use crate::formatter::json::JsonFormatter;
use crate::template::Template;
use crate::formatter::FlatFormatterD;
use crate::formatter::JsonFormatterD;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum LevelD {
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
    pub fn with_template(&self, tmpl_name: &str, tmpl: &mut Template) -> Result<()> {
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
    pub format: String,
    pub begin: DateTime<Utc>,//rfc3339
    pub end: DateTime<Utc>,
}

impl TimestampD {
    pub fn new(format: String, begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        TimestampD {format, begin, end}
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct MessageD {
    pub template: String,
    pub file: String,
    pub line: usize,
    pub method: String,
    pub level: LevelD,
}

impl MessageD {
    pub fn post_init(&self, id: &str, tmpl: &mut Template) -> Result<()> {
        self.with_template(id, tmpl)
    }

    pub fn with_template(&self, id: &str, tmpl: &mut Template) -> Result<()>{
        tmpl.add_template(id, &self.template)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct LoggerD {
    pub name: String,
    pub messages: Vec<MessageD>,
}

impl LoggerD {
    pub fn post_init(&self, id: &str, tmpl: &mut Template) -> Result<()> {
        self.post_init_messagess(id, tmpl)
    }

    pub fn post_init_messagess(&self, id: &str, tmpl: &mut Template) -> Result<()> {
        if self.messages.len() == 0 {
            return Err(anyhow!("app {} should configure at least 1 message", self.name));
        }

        for (i, message_d) in self.messages.iter().enumerate() {
            let msg_id = format!("{}/{}", id, i);
            message_d.post_init(&msg_id, tmpl)?;
        }

        return Ok(());
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct AppD {
    pub name: String,
    pub num_of_lines: u64,
    pub formatter: FormatterD,
    pub timestamp: TimestampD,
    pub loggers: Vec<LoggerD>,
}

impl AppD {
    pub fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str::<Self>(yaml)
            .expect(&format!("failed to parse config yaml: {}", yaml))
    }

    pub fn post_init(&self, tmpl: &mut Template) -> Result<()> {
        self.post_init_loggers(tmpl)
    }

    pub fn post_init_loggers(&self, tmpl: &mut Template) -> Result<()> {
        self.formatter.with_template(&self.name, tmpl)?;

        if self.loggers.len() == 0 {
            return Err(anyhow!("app {} should configure at least 1 logger", self.name));
        }

        for (i, logger_d) in self.loggers.iter().enumerate() {
            let logger_id = format!("{}/{}", self.name, i);
            logger_d.post_init(&logger_id, tmpl)?;
        }

        return Ok(());
    }
}