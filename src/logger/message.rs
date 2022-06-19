use crate::base::FormatDef;

pub struct MessageDef {
    format: FormatDef,
    file: String,
    line: usize,
    method: String,
}

impl MessageDef {
    pub fn new() -> MessageDef {
        MessageDef {
            format: FormatDef::Flat,
            file: "app.cpp".to_string(),
            line: 62,
            method: "main".to_string(),
        }
    }

    pub fn next(&self) -> String {
        format!("{}/line{}-{}()", self.file, self.line, self.method)
    }
}
