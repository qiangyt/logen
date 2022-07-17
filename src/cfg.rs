use crate::app::simple::App;
use crate::assets::Asset;
use crate::TemplateEngine;
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::str;

static ASSET_EXAMPLE_FILE: &str = "example.yaml";
static ARG_EXAMPLE_CONFIG_FILE: &str = "<example>";

/// Log generator
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct CliArgs {
    /// Path of the configuration file
    #[clap(value_parser, required = false, default_value = ARG_EXAMPLE_CONFIG_FILE)]
    pub config_file: String,

    /// Generates using the example configuration file
    #[clap(short, long, value_parser, required = false, default_value_t = false)]
    pub example: bool,
}

pub fn with_cli_args() -> Result<Option<Logen>> {
    let args = CliArgs::parse();

    if args.example {
        let a = Asset::get(ASSET_EXAMPLE_FILE).unwrap();
        let yaml = String::from_utf8(a.data.as_ref().to_vec()).unwrap();
        return Ok(Some(Logen::from_yaml(&yaml)));
    }

    let f = args.config_file;
    if f == ARG_EXAMPLE_CONFIG_FILE {
        // output embedded example configuration
        let a = Asset::get(ASSET_EXAMPLE_FILE).unwrap();
        println!("{}", String::from_utf8(a.data.as_ref().to_vec())?);
        return Ok(None);
    }

    let yaml =
        fs::read_to_string(&f).with_context(|| format!("failed to open config file: {}", f))?;
    return Ok(Some(Logen::from_yaml(&yaml)));
}

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
