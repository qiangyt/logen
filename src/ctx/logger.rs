use rand::Rng;

use crate::def::LoggerDef;
use super::Message;
use super::Line;


pub struct Logger<'a> {
    def: &'a LoggerDef,
    messages: Vec<Message<'a>>,
}

impl<'a> Logger<'a> {
    pub fn new(def: &'a LoggerDef, id: String) -> Logger<'a> {
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

    pub fn next(&self, line: &mut Line) {
        line.var("logger", &self.def);

        let msg = self.next_message();
        msg.next(line);
    }

    fn next_message(&self) -> &Message {
        let mut i = 0;
        let max = self.messages.len();
        while i < 10   {
            let index = rand::thread_rng().gen_range(0..max * 2);
            if index < max {
                return &self.messages[index]
            }

            i = i+1;
        }

        return &self.messages[0];
    }

}
