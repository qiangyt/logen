use serde::{Deserialize, Serialize};

pub mod app;
pub mod assets;
pub mod cfg;
pub mod fmt;
pub mod tpl;
pub mod ts;

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
