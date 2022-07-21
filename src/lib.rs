pub mod app;
pub mod assets;

pub mod base;
use std::sync::mpsc::Sender;
use std::{collections::HashMap, sync::mpsc};
use std::thread;

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
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

pub struct Line {
    name: String,
    timestamp: DateTime<Utc>,
    text: String,
}

#[typetag::serde(tag = "type")]
pub trait App: Sync {
    fn init(&mut self, name: &str) -> Result<()>;
    fn generate(&self, sender: Sender<Line>) -> Result<()>;
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Logen {
    apps: HashMap<String, Box<dyn App>>,
}

impl Logen {
    pub fn from_yaml(yaml: &str) -> Box<Self> {
        serde_yaml::from_str(yaml)
            .expect(&format!("failed to parse config yaml: {}", yaml))
    }

    pub fn init(&mut self) -> Result<()> {
        for (app_name, app) in &mut self.apps {
            app.init(app_name.as_str())
                .with_context(|| format!("failed to init app: {}", app_name))?;
        }
        Ok(())
    }

    pub fn generate(&'static self) -> Result<()> {
        let mut app_handles = vec![];
        let apps = &self.apps;
        let (sender, rx) = mpsc::channel::<Line>();

        let console_h = thread::spawn(move || {
            for line in rx {
                println!("{} | {}", line.name, line.text);
            }
        });

        for (app_name, app) in apps {
            let app_sender = sender.clone();
            let app_h = thread::spawn(move || {
                match app.generate(app_sender) {
                    Err(err) => println!("failed to generate log from app: {}, error is {}", app_name, err),
                    Ok(()) => {}
                };
            });
            app_handles.push(app_h);
        }

        drop(sender);

        for app_h in app_handles {
            app_h.join().unwrap(); //TODO
        }

        console_h.join().unwrap();//TODO

        Ok(())
    }

}
