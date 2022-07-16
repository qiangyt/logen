use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{tpl::Template, ts::Timestamp};

use super::Formatter;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Style {
    Bunyan,
}

static BUNYAN_TRACE: u8 = 10;
static BUNYAN_DEBUG: u8 = 20;
static BUNYAN_INFO: u8 = 30;
static BUNYAN_WARN: u8 = 40;
static BUNYAN_ERROR: u8 = 50;
static BUNYAN_FATAL: u8 = 60;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct JsonFormatterD {
    pub style: Style,
}

pub struct JsonFormatter<'a> {
    def: &'a JsonFormatterD,
}

impl<'a> JsonFormatter<'a> {
    pub fn new(def: &'a JsonFormatterD) -> Self {
        Self { def }
    }
}

impl<'a> Formatter for JsonFormatter<'a> {
    fn format_timestamp(&self, timestamp: &Timestamp) -> String {
        let ts_format = match self.def.style {
            Style::Bunyan => "%Y-%m-%dT%H:%M:%S%.3f", //2020-07-09T17:47:21.918Z
        };

        timestamp.format(ts_format)
    }

    fn format(&self, t: &Template, _: &str) -> Result<String> {
        let lv = match t.get("level") {
            None => "INFO",
            Some(l) => l.as_str().unwrap(),
        };

        let level = match lv {
            "FINE" => BUNYAN_TRACE,
            "TRACE" => BUNYAN_TRACE,
            "DEBUG" => BUNYAN_DEBUG,
            "INFO" => BUNYAN_INFO,
            "WARN" => BUNYAN_WARN,
            "ERROR" => BUNYAN_ERROR,
            "FATAL" => BUNYAN_FATAL,
            _ => BUNYAN_INFO,
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
