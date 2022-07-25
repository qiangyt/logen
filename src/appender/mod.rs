use serde::{Serialize, Deserialize};
use anyhow::Result;

use crate::Line;

pub mod console;
pub use console::{Appender as ConsoleAppender, ConsoleSender};

pub mod file;
pub use file::{Appender as FileAppender, AppenderDef as FileAppenderDef};


#[derive(Debug, Serialize, Deserialize)]
pub enum AppenderDef {
    Console,
    File(FileAppenderDef),
}

impl Default for AppenderDef {
    fn default() -> Self {
        AppenderDef::Console
    }
}

impl AppenderDef {
    pub fn build_appender<'a>(&'a self, console: &'a ConsoleSender) -> Result<Box<dyn AppenderT + 'a>> {
        match self {
            AppenderDef::Console => Ok(ConsoleAppender::new(console)),
            AppenderDef::File(f) => Ok(FileAppender::new(f)?)
        }
    }
}

pub trait AppenderT {
    fn append(&mut self, line: &Line) -> Result<()>;
}