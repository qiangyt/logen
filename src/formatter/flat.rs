use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::{template::Template, ctx::line::Line};

use super::Formatter;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct FlatFormatterD {
    pub template: String,
}

impl FlatFormatterD {
    pub fn with_template(&self, tmpl_name: &str, tmpl: &mut Template) -> Result<()> {
        tmpl.add_template(tmpl_name, &self.template)
    }
}

pub struct FlatFormatter<'a> {
    def: &'a FlatFormatterD
}

impl <'a> FlatFormatter<'a> {
    pub fn new(def: &'a FlatFormatterD) -> Self {
        Self { def }
    }
}

impl <'a> Formatter for FlatFormatter<'a> {
    fn format(&self, line: &Line) -> Result<String> {
        line.render_with_template(line.app().name())
    }
}