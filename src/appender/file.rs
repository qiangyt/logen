use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use crate::base::Line;

use super::AppenderT;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAppenderDef {
    path: String,
    append: bool
}

pub struct FileAppender<'a> {
    def: &'a FileAppenderDef,
    file: File,
}

impl <'a> FileAppender<'a> {

    pub fn new(def: &FileAppenderDef) -> Result<Box<FileAppender>> {
        Ok(Box::new(FileAppender {
            def,
            file: File::options().append(def.append).open(&def.path)
                    .with_context(|| format!("failed to open file: {}", def.path))?,
        }))
    }

}

impl <'a> AppenderT for FileAppender<'a> {

    fn append(&mut self, line: Arc<Line>) -> Result<()> {
        let data = format!("{}\n", line.text);
        self.file.write_all(data.as_bytes())
            .with_context(|| format!("failed to write to file: {}", self.def.path))
    }

}