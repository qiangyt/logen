use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use crate::base::Line;

use super::AppenderT;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppenderDef {
    path: String,
    append: bool,
}

pub struct Appender<'a> {
    def: &'a AppenderDef,
    file: File,
}

impl<'a> Appender<'a> {
    pub fn new(def: &'a AppenderDef) -> Result<Box<Appender>> {
        Ok(Box::new(Self {
            def,
            file: File::options()
                .append(def.append)
                .open(&def.path)
                .with_context(|| format!("failed to open file: {}", def.path))?,
        }))
    }
}

impl<'a> AppenderT for Appender<'a> {
    fn append(&mut self, line: &Line) -> Result<()> {
        let data = format!("{}\n", line.text);
        self.file
            .write_all(data.as_bytes())
            .with_context(|| format!("failed to write to file: {}", self.def.path))
    }
}
