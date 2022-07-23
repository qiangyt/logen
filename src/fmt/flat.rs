use anyhow::Result;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::{Template, TemplateEngine};

use super::FormatterT;

pub const DEFAULT_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
pub const DEFAULT_PATTERN: &str = r#"{{timestamp}} <{{level | upper | align_left(width=5)}}> [{{mdc | map}}] {{logger}} {{file}}/{{line}} {{method}} - {{message}}"#;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct FlatFormatter {
    #[serde(default = "FlatFormatter::default_time_format")]
    time_format: String,

    #[serde(default = "FlatFormatter::default_pattern")]
    pattern: String,
}

impl Default for FlatFormatter {
    fn default() -> Self {
        Self {
            time_format: FlatFormatter::default_time_format(),
            pattern: FlatFormatter::default_pattern(),
        }
    }
}

impl FlatFormatter {
    pub fn default_time_format() -> String {
        DEFAULT_TIME_FORMAT.to_string()
    }

    pub fn default_pattern() -> String {
        DEFAULT_PATTERN.to_string()
    }

    pub fn init(&self, tmpl_name: &str, tmpl: &mut TemplateEngine) -> Result<()> {
        tmpl.add_template(tmpl_name, &self.pattern)
    }
}

impl FormatterT for FlatFormatter {
    fn format_timestamp(&self, timestamp: &DateTime<Utc>) -> String {
        timestamp.format(&self.time_format).to_string()
    }

    fn format(&self, t: &Template, template_name: &str) -> Result<String> {
        t.render(template_name)
    }
}
