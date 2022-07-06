
use chrono::prelude::*;
use chrono::{Utc};
use serde::{Deserialize, Serialize};
use anyhow::Result;

use super::template::Template;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LevelDef {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}



#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormatDef {
    Flat,
    Json,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StyleDef {
    Bunyan,
}




#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct TimestampDef {
    pub format: String,
    pub begin: DateTime<Utc>,//rfc3339
    pub end: DateTime<Utc>,
}

impl TimestampDef {
    pub fn new(format: String, begin: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        TimestampDef {format, begin, end}
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageDef {
    pub template: String,
    pub file: String,
    pub line: usize,
    pub method: String,
    pub level: LevelDef,
}

impl MessageDef {
    pub fn post_init(&self, id: &str, tmpl: &mut Template) -> Result<()> {
        self.with_template(id, tmpl)
    }

    pub fn with_template(&self, id: &str, tmpl: &mut Template) -> Result<()>{
        tmpl.add_template(id, &self.template)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoggerDef {
    pub name: String,
    pub messages: Vec<MessageDef>,
}

impl LoggerDef {
    pub fn post_init(&self, id: &str, tmpl: &mut Template) -> Result<()> {
        self.post_init_messagess(id, tmpl)
    }

    pub fn post_init_messagess(&self, id: &str, tmpl: &mut Template) -> Result<()> {
        for (i, message_def) in self.messages.iter().enumerate() {
            let msg_id = format!("{}/{}", id, i);
            message_def.post_init(&msg_id, tmpl)?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AppDef {
    pub name: String,
    pub template: String,
    pub style: StyleDef,
    pub lines: u64,
    pub format: FormatDef,
    pub timestamp: TimestampDef,
    pub loggers: Vec<LoggerDef>,
}

impl AppDef {
    pub fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str::<Self>(yaml)
            .expect(&format!("failed to parse config yaml: {}", yaml))
    }

    pub fn post_init(&self, tmpl: &mut Template) -> Result<()> {
        self.post_init_loggers(tmpl)
    }

    pub fn post_init_loggers(&self, tmpl: &mut Template) -> Result<()> {
        self.with_template(tmpl)?;

        for (i, logger_def) in self.loggers.iter().enumerate() {
            let logger_id = format!("{}/{}", self.name, i);
            logger_def.post_init(&logger_id, tmpl)?;
        }

        Ok(())
    }

    pub fn with_template(&self, tmpl: &mut Template) -> Result<()> {
        tmpl.add_template(&self.name, &self.template)
    }
}