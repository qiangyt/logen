use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::{
    fmt::{FlatFormatter, Formatter, JsonFormatter},
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

    pub fn formatter(&self) -> &dyn Formatter {
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
}

impl Output {
    pub fn init(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        self.format.init(tmpl_name, tmpl)
    }

    pub fn formatter(&self) -> &dyn Formatter {
        self.format.formatter()
    }
}
