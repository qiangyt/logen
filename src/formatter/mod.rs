use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::Template;

pub mod flat;
pub use flat::FlatFormatterD as FlatFormatterD;

pub mod json;
pub use json::JsonFormatterD as JsonFormatterD;

