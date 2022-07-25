use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

use crate::base::Line;

use super::AppenderT;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct AppenderDef {
    path: String,

    #[serde(default)]
    truncate: bool,
}

pub struct Appender<'a> {
    def: &'a AppenderDef,
    file: File,
}

impl<'a> Appender<'a> {
    pub fn new(def: &'a AppenderDef) -> Result<Box<Appender>> {
        Ok(Box::new(Self {
            def,
            file: {
                let mut opts = File::options();  
                opts.create(true);
                if def.truncate {
                    opts.write(true).truncate(def.truncate);
                } else {
                    opts.append(true);
                }
                opts.open(&def.path)
                .with_context(|| format!("failed to open file: {}", def.path))?
            }
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
