use rand::Rng;
use serde::{Serialize, Deserialize};

pub mod message;
pub use message::{Message, MessageDef};

pub mod template;
pub use template::Template;


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoggerDef {
    name: String,
    messages: Vec<MessageDef>,
}

pub struct Logger<'a> {
    def: &'a LoggerDef,
    messages: Vec<Message<'a>>,
}

impl<'a> Logger<'a> {
    pub fn new(def: &'a LoggerDef, id: String, tmpl: &mut Template) -> Logger<'a> {
        Logger {
            messages: {
                let mut v = Vec::new();
                for (i, message_def) in def.messages.iter().enumerate() {
                    let msg_id = format!("{}/{}", id, i);
                    v.push(Message::new(message_def, msg_id, tmpl));
                }
                v
            },
            def,
        }
    }

    pub fn next(&self, data: &mut tera::Context, tmpl: &Template) {
        data.insert("logger", &self.def);
        self.next_message().next(data, tmpl);
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
