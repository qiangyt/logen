use handlebars::{Handlebars, to_json};
use serde::{Serialize, Deserialize};
use serde_json::value::{Map, Value};
use rand::Rng;

mod message;
pub use message::{Message, MessageDef};


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
    pub fn new(def: &'a LoggerDef, id: String, handlbars: &mut Handlebars) -> Logger<'a> {
        Logger {
            messages: {
                let mut v = Vec::new();
                for (i, message_def) in def.messages.iter().enumerate() {
                    let msg_id = format!("{}/{}", id, i);
                    v.push(Message::new(message_def, msg_id, handlbars));
                }
                v
            },
            def,
        }
    }

    pub fn next(&self, data: &mut Map<String, Value>, handlbars: &Handlebars) {
        data.insert("logger".to_string(), to_json(self.def));
        self.next_message().next(data, handlbars);
    }

    fn next_message(&self) -> &Message {
        let i = rand::thread_rng().gen_range(0..self.messages.len());
        &self.messages[i]
    }

}
