use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::{tpl::KEY_LEVEL, Template};

use super::{bunyan, Formatter};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum JsonStyle {
    Bunyan,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct JsonFormatter {
    pub style: JsonStyle,
}

impl Formatter for JsonFormatter {
    fn format_timestamp(&self, timestamp: &DateTime<Utc>) -> String {
        let ts_format = match self.style {
            JsonStyle::Bunyan => bunyan::TIME_FORMAT,
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
