use handlebars::{Handlebars, RenderError, to_json};
use serde::{Serialize, Deserialize};
use serde_json::value::{Map, Value};
use rand::Rng;

mod message;
pub use message::MessageDef;


#[derive(Debug, Serialize, Deserialize)]
pub struct LoggerDef {
    id: String,
    name: String,
    template: String,
    file: String,
    line: usize,
    method: String,
    messages: Vec<MessageDef>,
}

impl LoggerDef {
    pub fn new(id: String, name: String, handlebars: &mut Handlebars) -> LoggerDef {
        let tpl = "<{{name}}>({{file}}/{{line}} {{method}}()) {{message}}";
        handlebars.register_template_string(&id, tpl)
            .expect(format!("failed to register logger handlebars template {}: {}", id, tpl).as_str());

        LoggerDef {
            template: tpl.to_string(),
            messages: vec![
                MessageDef::new(format!("{}/{}", id, 0), handlebars)
            ],
            id, name,
            file: "app.cpp".to_string(),
            line: 62,
            method: "main".to_string(),
        }
    }

    pub fn next(&self, handlebars: &Handlebars, mut data: Map<String, Value>) -> Result<String, RenderError> {
        let message_text = {
            let mut message_data = Map::new();
            message_data.insert("logger".to_string(), to_json(self));

            self.next_message().next(handlebars, message_data)?
        };

        data.insert("file".to_string(), to_json(self.file.as_str()));
        data.insert("line".to_string(), to_json(self.line));
        data.insert("method".to_string(), to_json(self.method.as_str()));
        data.insert("name".to_string(), to_json(self.name.as_str()));
        data.insert("message".to_string(), to_json(message_text));

        handlebars.render(&self.id, &data)
    }

    fn next_message(&self) -> &MessageDef {
        rand::thread_rng().gen_range(0..self.messages.len());
        &self.messages[0]
    }

}
