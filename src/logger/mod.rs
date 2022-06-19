use std::{time::{Duration, SystemTime, UNIX_EPOCH}, str::FromStr};

mod message;
pub use message::MessageDef;


pub struct LoggerDef {
    name: String,
    messages: Vec<MessageDef>,
}

impl LoggerDef {
    pub fn new(name: String) -> LoggerDef {
        LoggerDef {
            name,
            messages: vec![
                MessageDef::new()
            ]
        } 
    }

    pub fn next(&self) -> String {
        let msg = &self.messages[0];
        format!("{} {}", self.name, msg.next())
    }
}
