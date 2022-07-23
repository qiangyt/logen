use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::{
    fmt::{FlatFormatter, Formatter, JsonFormatter},
    appender::{Appender, AppenderDef, SenderConsole},
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

    pub fn build_formatter(&self) -> &dyn Formatter {
        match self {
            OutputFormat::Flat(f) => f,
            OutputFormat::Json(j) => j,
        }
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Output {
    #[serde(default)]
    format: OutputFormat,

    #[serde(default)]
    appender: AppenderDef,
}

impl Output {
    pub fn init(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        self.format.init(tmpl_name, tmpl)
    }

    pub fn default_appenders() -> Vec<AppenderDef> {
        vec!(AppenderDef::default())
    }

    pub fn build_formatter(&self) -> &dyn Formatter {
        self.format.build_formatter()
    }

    pub fn build_appender<'a>(&'a self, console: SenderConsole) -> Result<Box<dyn Appender + 'a>> {
        self.appender.build_appender(console)
    }
}
