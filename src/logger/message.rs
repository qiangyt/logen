use crate::base::Level;
use serde::{Deserialize, Serialize};
use crate::logger::template::Template;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MessageDef {
    template: String,
    file: String,
    line: usize,
    method: String,
    level: Level,
}

pub struct Message<'a> {
    def: &'a MessageDef,
    id: String,
}

impl<'a> Message<'a> {
    pub fn new(def: &'a MessageDef, id: String, tmpl: &mut Template) -> Message<'a> {
        tmpl.add_raw_template(&id, &def.template);
        
        Message {def, id}
    }

    //#[allow(unused_mut)]
    pub fn next(&self, data: &mut tera::Context, tmpl: &Template) {
        let def = self.def;

        data.insert("file", &def.file);
        data.insert("line", &def.line);
        data.insert("method", &def.method);
        data.insert("level", &def.level);

        let text = tmpl.render(self.id.as_str(), data);
        data.insert("message".to_string(), &text);
    }
}
