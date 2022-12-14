use crate::util::teras;
use anyhow::{Context, Result};
use serde::Serialize;
use tera::{to_value, Tera, Value};

pub const KEY_LEVEL: &str = "level";

#[derive(Debug)]
pub struct Engine {
    tera: Tera,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            tera: teras::default(),
        }
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

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Template<'a> {
    data: tera::Context,
    engine: &'a Engine,
}

impl<'a> Template<'a> {
    pub fn new(engine: &'a Engine) -> Self {
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
