use anyhow::Result;

pub mod flat;
pub use flat::FlatFormatterD as FlatFormatterD;

pub mod json;
pub use json::JsonFormatterD as JsonFormatterD;

use crate::{template::Template, ctx::timestamp::Timestamp};

pub trait Formatter {
  fn format_timestamp(&self, timestamp: &Timestamp) -> String;
  fn format(&self, t: &Template, template_name: &str) -> Result<String>;
}

