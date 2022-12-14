pub mod simple;
pub use simple::App as SimpleApp;

use crate::ConsoleSender;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[typetag::serde(tag = "type")]
pub trait AppT: Sync {
    fn init(&mut self, name: &str) -> Result<()>;
    fn generate(&self, console: ConsoleSender) -> Result<()>;
    fn need_console(&self) -> bool;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum AppType {
    Simple,
}
