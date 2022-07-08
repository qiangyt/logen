use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::Template;

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
