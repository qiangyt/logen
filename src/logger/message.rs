
use serde_json::value::{Map, Value};
use serde::{Deserialize, Serialize};
use handlebars::{Handlebars, RenderError};


#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDef {
    id: String,
    template: String,
}

impl MessageDef {
    pub fn new(id: String, handlebars: &mut Handlebars) -> MessageDef {
        let tpl = "blah";
        handlebars.register_template_string(&id, tpl)
            .expect(format!("failed to register message handlebars template {}: {}", id, tpl).as_str());

        MessageDef {
            id,
            template: tpl.to_string(),
        }
    }

    #[allow(unused_mut)]
    pub fn next(&self, handlebars: &Handlebars, mut data: Map<String, Value>) -> Result<String, RenderError> {
        handlebars.render(&self.id, &data)
    }
}
