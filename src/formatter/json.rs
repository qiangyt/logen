use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::Template;

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

