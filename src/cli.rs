use crate::assets::Asset;
use anyhow::Context;
use anyhow::Result;
use clap::Parser;
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

pub fn yaml_with_args() -> Result<Option<String>> {
    let args = CliArgs::parse();

    if args.example {
        let a = Asset::get(ASSET_EXAMPLE_FILE).unwrap();
        let yaml = String::from_utf8(a.data.as_ref().to_vec()).unwrap();
        return Ok(Some(yaml));
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
    return Ok(Some(yaml));
}
