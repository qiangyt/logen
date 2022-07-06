use clap::Parser;

/// Log generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Path of the configuration file
    #[clap(value_parser, default_value = "<embedded example>")]
    pub config_file: String,

    /// Print the example configuration file
    #[clap(short, long, value_parser, default_value_t = false)]
    pub example_config_file: bool,
}


pub struct Options {
    pub config_yaml: String,
}

impl Options {

}