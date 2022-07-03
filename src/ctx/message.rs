use crate::def::*;
use crate::template::Template;
use crate::ctx::line::Line;


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
    pub fn next(&self, line: &mut Line) {
        let def = self.def;
        let data = &mut line.data;
        
        data.insert("file", &def.file);
        data.insert("line", &def.line);
        data.insert("method", &def.method);
        data.insert("level", &def.level);

        let text = line.template.render(&self.id, data);
        data.insert("message".to_string(), &text);
    }
}
