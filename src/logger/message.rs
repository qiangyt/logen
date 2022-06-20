
use serde_json::value::{Map, Value};
use serde::{Deserialize, Serialize};
use handlebars::{Handlebars, RenderError, to_json};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageDef {
    template: String,
    file: String,
    line: usize,
    method: String,
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

    //#[allow(unused_mut)]
    pub fn next(&self, handlebars: &Handlebars, mut data: Map<String, Value>) -> Result<String, RenderError> {
        let def = &self.def;

        data.insert("file".to_string(), to_json(def.file.as_str()));
        data.insert("line".to_string(), to_json(def.line));
        data.insert("method".to_string(), to_json(def.method.as_str()));

        handlebars.render(&self.id, &data)
    }
}
