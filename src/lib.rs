pub mod appender;
pub use appender::*;
pub mod apps;
pub use apps::*;
pub mod assets;
pub use assets::*;
pub mod base;
pub use base::*;
pub mod cli;
pub use cli::*;
pub mod fmt;
pub use fmt::*;
pub mod util;
pub use util::*;

use std::thread;
use std::{collections::HashMap, sync::mpsc};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Logen {
    apps: HashMap<String, Box<dyn AppT>>,
}

impl Logen {
    pub fn from_yaml(yaml: &str) -> Box<Self> {
        serde_yaml::from_str(yaml).expect(&format!("failed to parse config yaml: {}", yaml))
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
            let target_console = ConsoleSender::new(&sender);
            let app_h = thread::spawn(move || {
                match app.generate(target_console) {
                    Err(err) => println!(
                        "failed to generate log from app: {}, error is {}",
                        app_name, err
                    ),
                    Ok(()) => {}
                };
            });
            app_handles.push(app_h);
        }

        for app_h in app_handles {
            app_h.join().unwrap(); //TODO
        }

        drop(sender);
        console_h.join().unwrap(); //TODO

        Ok(())
    }
}
