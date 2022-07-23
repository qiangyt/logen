use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::{tpl::KEY_LEVEL, Template};

use crate::{bunyan, FormatterT};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Style {
    Bunyan,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Formatter {
    pub style: Style,
}

impl FormatterT for Formatter {
    fn format_timestamp(&self, timestamp: &DateTime<Utc>) -> String {
        let ts_format = match self.style {
            Style::Bunyan => bunyan::TIME_FORMAT,
        };

        timestamp.format(ts_format).to_string()
    }

    fn format(&self, t: &Template, _: &str) -> Result<String> {
        let lv = match t.get(KEY_LEVEL) {
            None => "INFO",
            Some(l) => l.as_str().unwrap(),
        };

        let j = bunyan::build(t, lv);

        Ok(serde_json::to_string(&j)?)
    }
}
