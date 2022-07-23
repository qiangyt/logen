
use serde::{Serialize, Deserialize};
use anyhow::Result;

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
    pub fn build_appender<'a>(&'a self, console: SenderConsole) -> Result<Box<dyn Appender + 'a>> {
        match self {
            AppenderDef::Console(c) => Ok(ConsoleAppender::new(c, console)),
            AppenderDef::File(f) => Ok(FileAppender::new(f)?)
        }
    }
}

pub trait Appender {
    fn append(&mut self, line: crate::base::Line) -> Result<()>;
}