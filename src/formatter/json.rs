use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::ctx::line::Line;

use super::Formatter;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum StyleD {
    Bunyan,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct JsonFormatterD {
  pub style: StyleD,
}

pub struct JsonFormatter<'a> {
  #[allow(dead_code)]
  def: &'a JsonFormatterD,
}

impl<'a>  JsonFormatter<'a> {
  pub fn new(def: &'a JsonFormatterD) -> Self {
    Self {def}
  }
}

impl<'a> Formatter for JsonFormatter<'a> {
    fn format(&self, line: &Line) -> Result<String> {
      //{"name":"local-agent","hostname":"c83ae3c3aaed","pid":16,"id":"ApiServer","level":30,"msg":"start server initialization","time":"2020-07-09T17:47:21.918Z","v":0}

        todo!()
    }
}