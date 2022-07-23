use std::sync::{mpsc::Sender, Arc};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

use super::Appender;
use crate::base::Line;


pub struct SenderConsole {
    sender: Sender<Arc<Line>>,
}

impl SenderConsole {

    pub fn new(sender: Sender<Arc<Line>>) -> Self {
        Self {sender: sender}
    }

    pub fn write(&self, line: Arc<Line>) -> Result<()> {
        self.sender.send(line).with_context(|| "failed to write to console")
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsoleAppenderDef {}

pub struct ConsoleAppender<'a> {
    def: &'a ConsoleAppenderDef,
    console: &'a SenderConsole,
}

impl <'a> ConsoleAppender<'a> {

    pub fn new(def: &'a ConsoleAppenderDef, console: &'a SenderConsole) -> Box<ConsoleAppender<'a>> {
        Box::new(ConsoleAppender { def, console })
    }

}

impl <'a> Appender for ConsoleAppender<'a> {

    fn append(&mut self, line: Arc<Line>) -> anyhow::Result<()> {
        self.console.write(line)
    }

}