
use serde_json::value::{Map, Value};
use serde::{Deserialize, Serialize};
use handlebars::{Handlebars, RenderError};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDef {
    template: String,
}

pub struct Message<'a> {
    def: &'a MessageDef,
    id: String,
}

impl<'a> Message<'a> {
    pub fn new(def: &'a MessageDef, id: String, handlebars: &mut Handlebars) -> Message<'a> {
        handlebars.register_template_string(&id, &def.template)
            .expect(format!("failed to register message handlebars template {}: {}", id, &def.template).as_str());

        Message {def, id}
    }

    #[allow(unused_mut)]
    pub fn next(&self, handlebars: &Handlebars, mut data: Map<String, Value>) -> Result<String, RenderError> {
        handlebars.render(&self.id, &data)
    }
}
