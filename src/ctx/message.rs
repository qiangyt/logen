use crate::def::*;
use anyhow::Result;

use super::line::Line;


pub struct Message<'a> {
    def: &'a MessageD,
    id: String,
}

impl<'a> Message<'a> {
    pub fn new(def: &'a MessageD, id: String) -> Self {
        Message {def, id}
    }

    //#[allow(unused_mut)]
    pub fn render(&self, line: &mut Line) -> Result<()> {
        let def = self.def;

        line.var("file", &def.file);
        line.var("line", &def.line);
        line.var("method", &def.method);
        line.var("level", &def.level);

        let message = line.render_with_template(&self.id)?;
        line.var("message", &message);

        Ok(())
    }
}
