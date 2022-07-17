pub mod app;
pub mod assets;

pub mod base;
use std::collections::HashMap;

use anyhow::{Context, Result};
use app::simple::App;
pub use base::{level, tpl, Level, Output, Template, TemplateEngine, Timestamp};
use serde::{Deserialize, Serialize};

pub mod cli;
pub mod fmt;

pub mod util;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum AppType {
    Simple,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Logen {
    apps: HashMap<String, App>,

    #[serde(skip)]
    template_engine: TemplateEngine,
}

impl Logen {
    pub fn from_yaml(yaml: &str) -> Self {
        let mut r = serde_yaml::from_str::<Self>(yaml)
            .expect(&format!("failed to parse config yaml: {}", yaml));

        r.init().expect(&format!("failed to init: {}", yaml));

        return r;
    }

    pub fn init(&mut self) -> Result<()> {
        for (app_name, app) in self.apps.iter_mut() {
            app.init(app_name.clone(), &mut self.template_engine)
                .with_context(|| format!("failed to init app: {}", app_name))?;
        }

        Ok(())
    }

    pub fn generate(&self) -> Result<()> {
        for app in self.apps.values() {
            app.generate(&self.template_engine)
                .with_context(|| format!("failed to generate logs on app: {}", app.name()))?;
        }

        Ok(())
    }
}
