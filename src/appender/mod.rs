
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::Line;

pub mod console;
pub use console::{Appender as ConsoleAppender, AppenderDef as ConsoleAppenderDef, ConsoleSender};

pub mod file;
pub use file::{Appender as FileAppender, AppenderDef as FileAppenderDef};


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
    pub fn build_appender<'a>(&'a self, console: &'a ConsoleSender) -> Result<Box<dyn AppenderT + 'a>> {
        match self {
            AppenderDef::Console(c) => Ok(ConsoleAppender::new(c, console)),
            AppenderDef::File(f) => Ok(FileAppender::new(f)?)
        }
    }
}

pub trait AppenderT {
    fn append(&mut self, line: Arc<Line>) -> Result<()>;
}