use std::sync::{mpsc::Sender, Arc};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

use super::AppenderT;
use crate::base::Line;


pub struct ConsoleSender {
    sender: Sender<Arc<Line>>,
}

impl ConsoleSender {

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
    console: &'a ConsoleSender,
}

impl <'a> ConsoleAppender<'a> {

    pub fn new(def: &'a ConsoleAppenderDef, console: &'a ConsoleSender) -> Box<ConsoleAppender<'a>> {
        Box::new(ConsoleAppender { def, console })
    }

}

impl <'a> AppenderT for ConsoleAppender<'a> {

    fn append(&mut self, line: Arc<Line>) -> anyhow::Result<()> {
        self.console.write(line)
    }

}