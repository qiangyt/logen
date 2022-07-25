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

    pub fn amount_of_apps_which_needs_console(&self) -> u32 {
        let mut r = 0 as u32;
        for app in self.apps.values() {
            if app.need_console() {
                r += 1;
            }
        }
        return r;
    }

    pub fn generate(&'static self) -> Result<()> {
        let mut app_handles = vec![];
        let apps = &self.apps;
        let (sender, rx) = mpsc::channel::<Line>();

        let console_h = thread::spawn(move || {
            for line in rx {
                if self.amount_of_apps_which_needs_console() == 1 {
                    println!("{}", line.text);
                } else {
                    println!("{} | {}", line.name, line.text);
                }
            }
        });

        for (app_name, app) in apps {
            let target_console = ConsoleSender::new(&sender);
            let app_h = thread::spawn(move || {
                match app.generate(target_console) {
                    Err(err) => println!(
                        "failed to generate log from app `{}`, cause: {}",
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
