pub mod app;
pub mod assets;

pub mod base;
use std::collections::HashMap;
use std::thread::{self, JoinHandle};

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
}

impl Logen {
    pub fn from_yaml(yaml: &str) -> Self {
        serde_yaml::from_str::<Self>(yaml)
            .expect(&format!("failed to parse config yaml: {}", yaml))
    }

    pub fn init(&mut self) -> Result<()> {
        for (app_name, app) in &mut self.apps {
            app.init(app_name.as_str())
                .with_context(|| format!("failed to init app: {}", app_name))?;
        }
        Ok(())
    }

    pub fn generate(&mut self) -> Result<()> {
        let mut thread_handles = Vec::<JoinHandle<()>>::new();
        let apps = &mut self.apps;
        for (_, app) in apps.drain() {
            let h = thread::spawn(move || {
                let app_name = app.name().to_string();
                match app.generate() {
                    Err(err) => println!("failed to generate log from app: {}, error is {}", app_name, err),
                    Ok(()) => {}
                };
            });
            thread_handles.push(h);
        }

        for h in thread_handles {
            h.join().unwrap(); //TODO
        }

        Ok(())
    }
}
