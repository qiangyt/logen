use std::sync::mpsc::Sender;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

use super::Appender;
use crate::base::Line;


pub struct SenderConsole {
    sender: Sender<Line>,
}

impl SenderConsole {

    pub fn new(sender: Sender<Line>) -> Self {
        Self {sender: sender}
    }

    pub fn write(&self, line: Line) -> Result<()> {
        self.sender.send(line).with_context(|| "failed to write to console")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleAppenderDef {}

pub struct ConsoleAppender<'a> {
    def: &'a ConsoleAppenderDef,
    console: SenderConsole,
}

impl <'a> ConsoleAppender<'a> {

    pub fn new(def: &ConsoleAppenderDef, console: SenderConsole) -> Box<ConsoleAppender> {
        Box::new(ConsoleAppender { def, console })
    }

}

impl <'a> Appender for ConsoleAppender<'a> {

    fn append(&mut self, line: Line) -> anyhow::Result<()> {
        self.console.write(line)
    }

}