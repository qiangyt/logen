use serde::Serialize;
use serde_json::to_value;
use anyhow::Result;

use crate::template::Template;

use super::app::App;

pub struct Line<'a> {
    data: tera::Context,
    app: &'a App<'a>,
    template: &'a Template,
}

impl<'a> Line<'a> {

    pub fn new(index: u64, app: &'a App, template: &'a Template, timestamp: &str) -> Self {
        let mut r = Line {
            data: tera::Context::new(),
            app,
            template,
        };

        r.var("index", &index);
        r.var("timestamp", timestamp);

        return r;
    }

    pub fn app(&self) -> &App {
        self.app
    }

    pub fn var<T: Serialize + ?Sized>(&mut self, key: &str, val: &T) {
        self.data.insert(key, &to_value(val).unwrap())
    }

    pub fn render_with_template(&self, template_name: &str) -> Result<String> {
        self.template.render(template_name, &self.data)
    }

}
