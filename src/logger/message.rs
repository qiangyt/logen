use crate::base::Level;
use serde::{Deserialize, Serialize};
use tera::{Tera};

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
    pub fn new(def: &'a MessageDef, id: String, tera: &mut Tera) -> Message<'a> {
        tera.add_raw_template(&id, &def.template)
        .expect(format!("failed to register message template {}: {}", id, def.template).as_str());

        Message {def, id}
    }

    //#[allow(unused_mut)]
    pub fn next(&self, data: &mut tera::Context, tera: &Tera) {
        let def = self.def;

        data.insert("file", &def.file);
        data.insert("line", &def.line);
        data.insert("method", &def.method);
        data.insert("level", &def.level);

        let text = tera.render(self.id.as_str(), data).unwrap();
        data.insert("message".to_string(), &text);
    }
}
