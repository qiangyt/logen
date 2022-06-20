use handlebars::{Handlebars, RenderError, to_json};
use serde::{Serialize, Deserialize};
use serde_json::value::{Map, Value};
use rand::Rng;

mod message;
pub use message::{Message, MessageDef};


#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerDef {
    name: String,
    template: String,
    file: String,
    line: usize,
    method: String,
    messages: Vec<MessageDef>,
}

pub struct Logger<'a> {
    def: &'a LoggerDef,
    id: String,
    messages: Vec<Message<'a>>,
}

impl<'a> Logger<'a> {
    pub fn new(def: &'a LoggerDef, id: String, handlebars: &mut Handlebars) -> Logger<'a> {
        handlebars.register_template_string(&id, &def.template)
            .expect(format!("failed to register logger handlebars template {}: {}", id, &def.template).as_str());

        Logger {
            messages: {
                let mut v = Vec::new();
                for (i, message_def) in def.messages.iter().enumerate() {
                    let msg_id = format!("{}/{}", id, i);
                    v.push(Message::new(message_def, msg_id, handlebars));
                }
                v
            },
            def, id,
        }
    }

    pub fn next(&self, handlebars: &Handlebars, mut data: Map<String, Value>) -> Result<String, RenderError> {
        let def = &self.def;

        let message_text = {
            let mut message_data = Map::new();
            message_data.insert("logger".to_string(), to_json(def));

            self.next_message().next(handlebars, message_data)?
        };

        data.insert("file".to_string(), to_json(def.file.as_str()));
        data.insert("line".to_string(), to_json(def.line));
        data.insert("method".to_string(), to_json(def.method.as_str()));
        data.insert("name".to_string(), to_json(def.name.as_str()));
        data.insert("message".to_string(), to_json(message_text));

        handlebars.render(&self.id, &data)
    }

    fn next_message(&self) -> &Message {
        rand::thread_rng().gen_range(0..self.messages.len());
        &self.messages[0]
    }

}
