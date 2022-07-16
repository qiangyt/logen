use serde::Serialize;
use serde_json::to_value;
use anyhow::Result;

use crate::template::Template;

use super::{app::App, timestamp::Timestamp};

pub struct Line<'a> {
    index: u64,
    data: tera::Context,
    app: &'a App<'a>,
    timestamp: &'a Timestamp<'a>,
    template: &'a Template,
}

impl<'a> Line<'a> {

    pub fn new(index: u64, app: &'a App, template: &'a Template, timestamp: &'a Timestamp<'a>) -> Self {
        let mut r = Line {
            index,
            data: tera::Context::new(),
            app,
            timestamp,
            template,
        };

        r.var("index", &index);

        return r;
    }

    pub fn app(&self) -> &App {
        self.app
    }

    pub fn timestamp(&self) -> &Timestamp {
        self.timestamp
    }

    pub fn var<T: Serialize + ?Sized>(&mut self, key: &str, val: &T) {
        self.data.insert(key, &to_value(val).unwrap())
    }

    pub fn render_with_template(&self, template_name: &str) -> Result<String> {
        self.template.render(template_name, &self.data)
    }

}
