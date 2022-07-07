use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::Template;

pub mod flat;
pub use flat::FlatFormatterDef as FlatFormatterDef;

pub mod json;
pub use json::JsonFormatterDef as JsonFormatterDef;

