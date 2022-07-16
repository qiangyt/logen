use anyhow::Result;

pub mod flat;
pub use flat::FlatFormatterD;

pub mod json;
pub use json::JsonFormatterD;

use crate::{template::Template, timestamp::Timestamp};

pub trait Formatter {
    fn format_timestamp(&self, timestamp: &Timestamp) -> String;
    fn format(&self, t: &Template, template_name: &str) -> Result<String>;
}
