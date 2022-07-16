use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    template::{Template, TemplateEngine},
    timestamp::Timestamp,
};

use super::Formatter;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct FlatFormatterD {
    pub timestamp_format: String,
    pub template: String,
}

impl FlatFormatterD {
    pub fn with_template(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        tmpl.add_template(tmpl_name, &self.template)
    }
}

pub struct FlatFormatter<'a> {
    def: &'a FlatFormatterD,
}

impl<'a> FlatFormatter<'a> {
    pub fn new(def: &'a FlatFormatterD) -> Self {
        Self { def }
    }
}

impl<'a> Formatter for FlatFormatter<'a> {
    fn format_timestamp(&self, timestamp: &Timestamp) -> String {
        timestamp.format(&self.def.timestamp_format)
    }

    fn format(&self, t: &Template, template_name: &str) -> Result<String> {
        t.render(template_name)
    }
}
