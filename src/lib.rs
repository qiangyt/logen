use serde::{Serialize, Deserialize};

pub mod assets;
pub mod config;
pub mod def;
pub mod fmt;
pub mod template;
pub mod timestamp;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Level {
    Fine,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}