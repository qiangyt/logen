use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::HashMap;
use tera::{to_value, try_get_value, Tera, Value};

pub const KEY_level: &str = "level";

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut tera = Tera::default();

        // disable autoescaping completely
        tera.autoescape_on(vec![]);

        let mut r = TemplateEngine { tera };
        r.register_default_filters();

        return r;
    }

    pub fn register_default_filters(&mut self) {
        self.tera.register_filter(
            "align_left",
            Box::new(TemplateEngine::tera_filter_align_left),
        );
        self.tera.register_filter(
            "align_right",
            Box::new(TemplateEngine::tera_filter_align_right),
        );
    }

    pub fn tera_filter_align_left(
        value: &Value,
        args: &HashMap<String, Value>,
    ) -> tera::Result<Value> {
        let mut value = try_get_value!("align_left", "value", String, value);

        let width = match args.get("width") {
            Some(width) => try_get_value!("align_left", "width", i32, width),
            None => {
                return Err(tera::Error::msg(
                    "filter `align_left` expected an arg called `width`",
                ))
            }
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

    pub fn tera_filter_align_right(
        value: &Value,
        args: &HashMap<String, Value>,
    ) -> tera::Result<Value> {
        let mut value = try_get_value!("align_right", "value", String, value);

        let width = match args.get("width") {
            Some(width) => try_get_value!("align_right", "width", i32, width),
            None => {
                return Err(tera::Error::msg(
                    "filter `align_right` expected an arg called `width`",
                ))
            }
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
        self.tera
            .add_raw_template(template_name, content)
            .with_context(|| format!("failed to add template '{}': {}", template_name, content))
    }

    pub fn render(&self, template_name: &str, data: &tera::Context) -> Result<String> {
        self.tera
            .render(template_name, data)
            .with_context(|| format!("failed to render template '{}': {:?}", template_name, data))
    }
}

pub struct Template<'a> {
    data: tera::Context,
    engine: &'a TemplateEngine,
}

impl<'a> Template<'a> {
    pub fn new(engine: &'a TemplateEngine) -> Self {
        Self {
            data: tera::Context::new(),
            engine,
        }
    }

    pub fn set<T: Serialize + ?Sized>(&mut self, key: &str, val: &T) {
        self.data.insert(key, &to_value(val).unwrap())
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }

    pub fn into_json(self) -> Value {
        self.data.into_json()
    }

    pub fn render(&self, template_name: &str) -> Result<String> {
        self.engine.render(template_name, &self.data)
    }
}
