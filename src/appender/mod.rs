
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::base::Line;

pub use self::console::{ConsoleAppender, ConsoleAppenderDef, SenderConsole};
pub use self::file::{FileAppender, FileAppenderDef};

pub mod console;
pub mod file;

#[derive(Debug, Serialize, Deserialize)]
pub enum AppenderDef {
    Console(ConsoleAppenderDef),
    File(FileAppenderDef),
}

impl Default for AppenderDef {
    fn default() -> Self {
        AppenderDef::Console(ConsoleAppenderDef{})
    }
}

impl AppenderDef {
    pub fn build_appender<'a>(&'a self, console: &'a SenderConsole) -> Result<Box<dyn AppenderT + 'a>> {
        match self {
            AppenderDef::Console(c) => Ok(ConsoleAppender::new(c, console)),
            AppenderDef::File(f) => Ok(FileAppender::new(f)?)
        }
    }
}

pub trait AppenderT {
    fn append(&mut self, line: Arc<Line>) -> Result<()>;
}