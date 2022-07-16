use serde_json::{json, Value};

use crate::{level, Template};

pub const LEVEL_TRACE: i8 = 10;
pub const LEVEL_DEBUG: i8 = 20;
pub const LEVEL_INFO: i8 = 30;
pub const LEVEL_WARN: i8 = 40;
pub const LEVEL_ERROR: i8 = 50;
pub const LEVEL_FATAL: i8 = 60;
pub const LEVEL_DEFAULT: i8 = LEVEL_INFO;

// for ex., 2020-07-09T17:47:21.918Z
pub const TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f";

pub fn level_i8(name: &str) -> i8 {
    match name {
        level::FINE => LEVEL_TRACE,
        level::TRACE => LEVEL_TRACE,
        level::DEBUG => LEVEL_DEBUG,
        level::INFO => LEVEL_INFO,
        level::WARN => LEVEL_WARN,
        level::ERROR => LEVEL_ERROR,
        level::FATAL => LEVEL_FATAL,
        _ => LEVEL_DEFAULT,
    }
}

pub fn build(t: &Template, level_name: &str) -> Value {
    let level = level_i8(level_name);

    json!({
        "name": t.get("app"),
        "hostname": t.get("host"),
        "pid": t.get("pid"),
        "id": t.get("logger"),
        "level": level,
        "msg": t.get("message"),
        "time": t.get("timestamp"),
        "v": 0
    })
}
