use anyhow::Result;
use chrono::{DateTime, Utc};

mod flat;
pub use flat::FlatFormatter;

mod json;
pub use json::JsonFormatter;

pub mod bunyan;

use crate::Template;

pub trait FormatterT {
    fn format_timestamp(&self, timestamp: &DateTime<Utc>) -> String;
    fn format(&self, t: &Template, template_name: &str) -> Result<String>;
}
