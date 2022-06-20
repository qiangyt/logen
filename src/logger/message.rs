
use serde_json::value::{Map, Value};
use serde::{Deserialize, Serialize};
use handlebars::{Handlebars, to_json, RenderError};

use crate::base::FormatDef;

#[derive(Serialize, Deserialize)]
pub struct MessageDef {
    id: String,
    template: String,
    format: FormatDef,
    file: String,
    line: usize,
    method: String
}

impl MessageDef {
    pub fn new(id: String, handlebars: &mut Handlebars) -> MessageDef {
        let tpl = "{{file}}/{{line}} {{method}} blah ";
        handlebars.register_template_string(&id, tpl)
            .expect(format!("failed to register message handlebars template {}: {}", id, tpl).as_str());

        MessageDef {
            id,
            template: tpl.to_string(),
            format: FormatDef::Flat,
            file: "app.cpp".to_string(),
            line: 62,
            method: "main".to_string(),
        }
    }

    pub fn next(&self, handlebars: &Handlebars, mut data: Map<String, Value>) -> Result<String, RenderError> {
        data.insert("file".to_string(), to_json(self.file.as_str()));
        data.insert("line".to_string(), to_json(self.line));
        data.insert("method".to_string(), to_json(self.method.as_str()));

        handlebars.render(&self.id, &data)
    }
}
