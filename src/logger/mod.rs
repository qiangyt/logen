use handlebars::{Handlebars, RenderError, to_json};
use serde::{Serialize, Deserialize};
use serde_json::value::{Map, Value};

mod message;
pub use message::MessageDef;


#[derive(Serialize, Deserialize)]
pub struct LoggerDef {
    id: String,
    name: String,
    template: String,
    messages: Vec<MessageDef>,
}

impl LoggerDef {
    pub fn new(id: String, name: String) -> LoggerDef {
        LoggerDef {
            template: "{{name}} {{message}}".to_string(),
            messages: vec![
                MessageDef::new(format!("{}/{}", id.clone(), 0))
            ],
            id, name
        }
    }

    pub fn next(&self, handlebars: &Handlebars, data: &mut Map<String, Value>) -> Result<String, RenderError> {
        let messageText = {
            let mut messageData = Map::new();
            messageData.insert("logger".to_string(), to_json(self));
            
            self.nextMessage().next(handlebars, messageData)?
        };
        
        data.insert("name".to_string(), to_json(self.name));
        data.insert("message".to_string(), to_json(messageText));

        handlebars.render(&self.id, data)
    }

    fn nextMessage(&self) -> &MessageDef {
        &self.messages[0]
    }

}
