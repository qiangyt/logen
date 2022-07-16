use anyhow::Result;

pub mod flat;
pub use flat::FlatFormatterD;

pub mod json;
pub use json::JsonFormatterD;
use serde::{Deserialize, Serialize};

use crate::{
    template::{Template, TemplateEngine},
    timestamp::Timestamp,
};

use self::{flat::FlatFormatter, json::JsonFormatter};

pub trait Formatter {
    fn format_timestamp(&self, timestamp: &Timestamp) -> String;
    fn format(&self, t: &Template, template_name: &str) -> Result<String>;
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
