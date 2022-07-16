use rand::Rng;

use crate::def::LoggerD;
use anyhow::Result;

use super::{message::Message, line::Line};


pub struct Logger<'a> {
    def: &'a LoggerD,
    message: Vec<Message<'a>>,
}

impl<'a> Logger<'a> {
    pub fn new(def: &'a LoggerD, id: String) -> Self {
        Logger {
            def,
            message: {
                let mut v = Vec::new();
                for (i, message_d) in def.message.iter().enumerate() {
                    let msg_id = format!("{}/{}", id, i);
                    v.push(Message::new(message_d, msg_id));
                }
                v
            },
        }
    }

    pub fn render(&self, line: &mut Line) -> Result<()> {
        line.var("logger", &self.def);
        self.choose_message().render(line)
    }

    fn choose_message(&self) -> &Message {
        let mut i = 0;
        let max = self.message.len();
        let mut rng = rand::thread_rng();

        while i < 10   {
            let index = rng.gen_range(0..max * 2);
            if index < max {
                return &self.message[index]
            }

            i = i+1;
        }

        &self.message[0]
    }

}
