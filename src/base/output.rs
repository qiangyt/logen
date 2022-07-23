use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::{
    fmt::{FlatFormatter, FormatterT, JsonFormatter},
    appender::{AppenderT, AppenderDef, ConsoleSender},
    TemplateEngine,
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum OutputFormat {
    Flat(FlatFormatter),
    Json(JsonFormatter),
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Flat(FlatFormatter::default())
    }
}

impl OutputFormat {
    pub fn init(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        match self {
            OutputFormat::Flat(flat) => flat.init(tmpl_name, tmpl),
            OutputFormat::Json(_) => Ok(()),
        }
    }

    pub fn build_formatter(&self) -> &dyn FormatterT {
        match self {
            OutputFormat::Flat(f) => f,
            OutputFormat::Json(j) => j,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Output {
    #[serde(default)]
    format: OutputFormat,

    #[serde(default = "Output::default_appenders")]
    appenders: Vec<AppenderDef>,
}

impl Default for Output {
    fn default() -> Self {
        Self { 
            format: OutputFormat::default(), 
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
        for appenderD in &self.appenders {
            r.push(appenderD.build_appender(console)?);
        }

        Ok(r)
    }
}
