use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::{
    fmt::{FlatFormatter, Formatter, JsonFormatter},
    TemplateEngine
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Output {
    Flat(FlatFormatter),
    Json(JsonFormatter),
}

impl Default for Output {
    fn default() -> Self {
        Output::Flat(FlatFormatter::default())
    }
}

impl Output {
    pub fn init(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        match self {
            Output::Flat(flat) => flat.init(tmpl_name, tmpl),
            Output::Json(_) => Ok(()),
        }
    }

    pub fn formatter(&self) -> &dyn Formatter {
        match self {
            Output::Flat(f) => f,
            Output::Json(j) => j,
        }
    }
}
