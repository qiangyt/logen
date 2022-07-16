use anyhow::Result;

use fmt::{FlatFormatter, Formatter, JsonFormatter};
use serde::{Deserialize, Serialize};
use tpl::TemplateEngine;

pub mod app;
pub mod assets;
pub mod cfg;
pub mod fmt;
pub mod tpl;
pub mod ts;

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
pub enum Output {
    Flat(FlatFormatter),
    Json(JsonFormatter),
}

impl Output {
    pub fn with_template(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        match self {
            Output::Flat(flat) => flat.with_template(tmpl_name, tmpl),
            Output::Json(_) => Ok(()),
        }
    }

    pub fn formatter(&self) -> &dyn Formatter {
        match self {
            Output::Flat(f) => f,
            Output::Json(j) => j,
        }
    }
}
