use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    tpl::{Template, TemplateEngine},
    ts::Timestamp,
};

use super::Formatter;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct FlatFormatter {
    #[serde(default = "FlatFormatter::default_time_format")]
    time_format: String,

    #[serde(default = "FlatFormatter::default_pattern")]
    pattern: String,
}

impl FlatFormatter {
    pub fn default_time_format() -> String {
        "%Y-%m-%d %H:%M:%S".to_string()
    }

    pub fn default_pattern() -> String {
        "{{timestamp}} <{{level | upper | align_left(width=5)}}> {{logger}} {{file}}/{{line}} {{method}} - {{message}}".to_string()
    }

    pub fn with_template(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        tmpl.add_template(tmpl_name, &self.pattern)
    }
}

impl Formatter for FlatFormatter {
    fn format_timestamp(&self, timestamp: &Timestamp) -> String {
        timestamp.format(&self.time_format)
    }

    fn format(&self, t: &Template, template_name: &str) -> Result<String> {
        t.render(template_name)
    }
}
