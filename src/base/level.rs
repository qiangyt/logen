use serde::{Deserialize, Serialize};

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

pub const FINE: &str = "FINE";
pub const TRACE: &str = "TRACE";
pub const DEBUG: &str = "DEBUG";
pub const INFO: &str = "INFO";
pub const WARN: &str = "WARN";
pub const ERROR: &str = "ERROR";
pub const FATAL: &str = "FATAL";
pub const DEFAULT: &str = INFO;

impl Default for Level {
    fn default() -> Self {
        Level::Info
    }
}

impl Level {
    pub fn name(&self) -> &str {
        match self {
            Level::Fine => FINE,
            Level::Trace => TRACE,
            Level::Debug => DEBUG,
            Level::Info => INFO,
            Level::Warn => WARN,
            Level::Error => ERROR,
            Level::Fatal => FATAL,
        }
    }
}
