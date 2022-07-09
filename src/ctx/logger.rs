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
            def,
            messages: {
                let mut v = Vec::new();
                for (i, message_d) in def.messages.iter().enumerate() {
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
        let max = self.messages.len();
        let mut rng = rand::thread_rng();

        while i < 10   {
            let index = rng.gen_range(0..max * 2);
            if index < max {
                return &self.messages[index]
            }

            i = i+1;
        }

        &self.messages[0]
    }

}
