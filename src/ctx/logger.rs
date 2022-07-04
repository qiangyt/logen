use rand::Rng;

use crate::def::LoggerDef;
use super::Message;
use super::Line;
use anyhow::Result;


pub struct Logger<'a> {
    def: &'a LoggerDef,
    messages: Vec<Message<'a>>,
}

impl<'a> Logger<'a> {
    pub fn new(def: &'a LoggerDef, id: String) -> Self {
        Logger {
            messages: {
                let mut v = Vec::new();
                for (i, message_def) in def.messages.iter().enumerate() {
                    let msg_id = format!("{}/{}", id, i);
                    v.push(Message::new(message_def, msg_id));
                }
                v
            },
            def,
        }
    }

    pub fn next(&self, line: &mut Line) -> Result<Option<()>>{
        line.var("logger", &self.def);

        if let Some(msg) = self.next_message() {
            msg.next(line)?;
            return Ok(Some(()));
        }

        Ok(None)
    }

    fn next_message(&self) -> Option<&Message> {
        let mut i = 0;
        let max = self.messages.len();
        while i < 10   {
            let index = rand::thread_rng().gen_range(0..max * 2);
            if index < max {
                return Some(&self.messages[index])
            }

            i = i+1;
        }

        if self.messages.len() == 0 {
            return Some(&self.messages[0]);
        }

        None
    }

}
