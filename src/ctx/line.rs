use serde::Serialize;
use serde_json::to_value;
use anyhow::Result;

use crate::Template;

pub struct Line<'a> {
    data: tera::Context,
    template: &'a Template,
}

impl<'a> Line<'a> {

    pub fn new(index: u64, template: &'a Template, timestamp: &str) -> Self {
        let mut r = Line {
            data: tera::Context::new(),
            template,
        };

        r.var("index", &index);
        r.var("timestamp", timestamp);

        r
    }

    pub fn var<T: Serialize + ?Sized, S: Into<String>>(&mut self, key: S, val: &T) {
        self.data.insert(key.into(), &to_value(val).unwrap());
    }

    pub fn render(&self, template_name: &str) -> Result<String> {
        self.template.render(template_name, &self.data)
    }

}
