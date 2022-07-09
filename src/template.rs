use tera::{Tera, try_get_value, Value, to_value};
use std::collections::HashMap;
use anyhow::{Context, Result};

pub struct Template {
    tera: Tera,
}


impl Template {

    pub fn new() -> Self {
        let mut tera = Tera::default();

        // disable autoescaping completely
        tera.autoescape_on(vec![]);

        let mut r = Template {tera};
        r.register_default_filters();

        return r;
    }


    pub fn register_default_filters(&mut self) {
        self.tera.register_filter("align_left", Box::new(Template::tera_filter_align_left));
        self.tera.register_filter("align_right", Box::new(Template::tera_filter_align_right));
    }


    pub fn tera_filter_align_left(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let mut value = try_get_value!("align_left", "value", String, value);

        let width = match args.get("width") {
            Some(width) => try_get_value!("align_left", "width", i32, width),
            None => return Err(tera::Error::msg("filter `align_left` expected an arg called `width`")),
        };

        let mut len = 0;
        for _ in value.chars() {
            len = len + 1;
        }
        while len < width {
            value.push(' ');
            len = len + 1;
        }

        return Ok(to_value(value).unwrap());
    }

    pub fn tera_filter_align_right(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
        let mut value = try_get_value!("align_right", "value", String, value);

        let width = match args.get("width") {
            Some(width) => try_get_value!("align_right", "width", i32, width),
            None => return Err(tera::Error::msg("filter `align_right` expected an arg called `width`")),
        };

        let mut len = 0;
        for _ in value.chars() {
            len = len + 1;
        }
        while len < width {
            value.insert(0, ' ');
            len = len + 1;
        }

        return Ok(to_value(value).unwrap());
    }

    pub fn add_template(&mut self, template_name: &str, content: &str) -> Result<()> {
        self.tera.add_raw_template(template_name, content)
            .with_context(|| format!("failed to add template '{}': {}", template_name, content))
    }

    pub fn render(&self, template_name: &str, data: &tera::Context) -> Result<String> {
        self.tera.render(template_name, data)
            .with_context(|| format!("failed to render template '{}': {:?}", template_name, data))
    }

}