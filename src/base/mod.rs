use chrono::{DateTime, Utc};

use anyhow::Result;
use serde::{Deserialize, Serialize};


pub mod level;
pub use level::Level;

mod output;
pub use output::{Output,OutputFormat};

pub mod tpl;
pub use tpl::{Template, TemplateEngine};

mod ts;
pub use ts::Timestamp;

use crate::appender::console::SenderConsole;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum AppType {
    Simple,
}

pub struct Line {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub text: String,
}

#[typetag::serde(tag = "type")]
pub trait AppT: Sync {
    fn init(&mut self, name: &str) -> Result<()>;
    fn generate(&self, console: SenderConsole) -> Result<()>;
}


