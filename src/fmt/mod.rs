use anyhow::Result;

mod flat;
pub use flat::FlatFormatter;

mod json;
pub use json::JsonFormatter;

mod bunyan;

use crate::{tpl::Template, ts::Timestamp};

pub trait Formatter {
    fn format_timestamp(&self, timestamp: &Timestamp) -> String;
    fn format(&self, t: &Template, template_name: &str) -> Result<String>;
}
