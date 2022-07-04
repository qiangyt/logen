use crate::def::*;
use super::Line;


pub struct Message<'a> {
    def: &'a MessageDef,
    id: String,
}

impl<'a> Message<'a> {
    pub fn new(def: &'a MessageDef, id: String) -> Self {
        Message {def, id}
    }

    //#[allow(unused_mut)]
    pub fn next(&self, line: &mut Line) {
        let def = self.def;

        line.var("file", &def.file);
        line.var("line", &def.line);
        line.var("method", &def.method);
        line.var("level", &def.level);

        let message = line.render(&self.id);
        line.var("message", &message);
    }
}
