use std::sync::mpsc::Sender;
use anyhow::{Result, Context};
use crate::{Line};

use super::Appender;

pub struct SenderConsole {
    sender: Sender<Line>,
}

impl SenderConsole {

    pub fn new(sender: Sender<Line>) -> Self {
        Self {sender: sender}
    }

    pub fn write(&self, line: crate::Line) -> Result<()> {
        self.sender.send(line).with_context(|| "failed to write to console")
    }
}


pub struct ConsoleAppender {
    console: SenderConsole,
}

impl Appender for ConsoleAppender {

    fn append(&mut self, line: crate::Line) -> anyhow::Result<()> {
        self.console.write(line)
    }

}

impl ConsoleAppender {

    pub fn new(console: SenderConsole) -> ConsoleAppender {
        ConsoleAppender { console }
    }

}