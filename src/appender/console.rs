use anyhow::{Context, Result};
use std::sync::mpsc::Sender;

use super::AppenderT;
use crate::base::Line;

pub struct ConsoleSender {
    sender: Sender<Line>,
}

impl ConsoleSender {
    pub fn new(sender: &Sender<Line>) -> Self {
        Self {
            sender: sender.clone(),
        }
    }

    pub fn write(&self, line: &Line) -> Result<()> {
        let line = line.clone();
        self.sender
            .send(line)
            .with_context(|| "failed to write to console")
    }
}

pub struct Appender<'a> {
    console: &'a ConsoleSender,
}

impl<'a> Appender<'a> {
    pub fn new(console: &'a ConsoleSender) -> Box<Appender<'a>> {
        Box::new(Self { console })
    }
}

impl<'a> AppenderT for Appender<'a> {
    fn append(&mut self, line: &Line) -> anyhow::Result<()> {
        self.console.write(line)
    }
}
