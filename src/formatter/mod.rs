use anyhow::Result;

pub mod flat;
pub use flat::FlatFormatterD as FlatFormatterD;

pub mod json;
pub use json::JsonFormatterD as JsonFormatterD;

use crate::ctx::line::Line;

pub trait Formatter {
  fn prepare(&self, line: &mut Line);
  fn format(&self, line: &Line) -> Result<String>;
}

