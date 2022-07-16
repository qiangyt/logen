use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{tpl::Template, ts::Timestamp};

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
    fn format_timestamp(&self, timestamp: &Timestamp) -> String {
        let ts_format = match self.style {
            JsonStyle::Bunyan => bunyan::TIME_FORMAT,
        };

        timestamp.format(ts_format)
    }

    fn format(&self, t: &Template, _: &str) -> Result<String> {
        let lv = match t.get("level") {
            None => "INFO",
            Some(l) => l.as_str().unwrap(),
        };

        let level = match lv {
            "FINE" => bunyan::LEVEL_TRACE,
            "TRACE" => bunyan::LEVEL_TRACE,
            "DEBUG" => bunyan::LEVEL_DEBUG,
            "INFO" => bunyan::LEVEL_INFO,
            "WARN" => bunyan::LEVEL_WARN,
            "ERROR" => bunyan::LEVEL_ERROR,
            "FATAL" => bunyan::LEVEL_FATAL,
            _ => bunyan::LEVEL_DEFAULT,
        };

        let j = json!({
            "name": t.get("app"),
            "hostname": t.get("host"),
            "pid": t.get("pid"),
            "id": t.get("logger"),
            "level": level,
            "msg": t.get("message"),
            "time": t.get("timestamp"),
            "v": 0
        });

        Ok(serde_json::to_string(&j)?)
    }
}
