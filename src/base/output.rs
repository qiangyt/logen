use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::{
    FlatFormatter, FormatterT, JsonFormatter,
    AppenderT, AppenderDef, ConsoleSender,
    TemplateEngine,
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Format {
    Flat(FlatFormatter),
    Json(JsonFormatter),
}

impl Default for Format {
    fn default() -> Self {
        Format::Flat(FlatFormatter::default())
    }
}

impl Format {
    pub fn init(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        match self {
            Format::Flat(flat) => flat.init(tmpl_name, tmpl),
            Format::Json(_) => Ok(()),
        }
    }

    pub fn build_formatter(&self) -> &dyn FormatterT {
        match self {
            Format::Flat(f) => f,
            Format::Json(j) => j,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Output {
    #[serde(default)]
    format: Format,

    #[serde(default = "Output::default_appenders")]
    appenders: Vec<AppenderDef>,
}

impl Default for Output {
    fn default() -> Self {
        Self { 
            format: Format::default(), 
            appenders: Output::default_appenders(),
        }
    }
}

impl Output {
    pub fn init(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        self.format.init(tmpl_name, tmpl)
    }

    pub fn default_appenders() -> Vec<AppenderDef> {
        vec!(AppenderDef::default())
    }

    pub fn build_formatter(&self) -> &dyn FormatterT {
        self.format.build_formatter()
    }

    pub fn build_appenders<'a>(&'a self, console: &'a ConsoleSender) -> Result<Vec<Box<dyn AppenderT + 'a>>> {
        let mut r = Vec::with_capacity(self.appenders.len());
        for appender_d in &self.appenders {
            r.push(appender_d.build_appender(console)?);
        }

        Ok(r)
    }
}
