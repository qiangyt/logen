use crate::util::text;
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use tera::{to_value, try_get_value, Tera, Value};

pub const KEY_LEVEL: &str = "level";

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
        self.tera.register_filter("map", tera_filter_map);
        self.tera
            .register_filter("align_left", tera_filter_align_left);
        self.tera
            .register_filter("align_right", tera_filter_align_right);
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

pub fn tera_filter_map(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = try_get_value!("map", "value", BTreeMap<String,Value>, value);

    let mut sep = match args.get("sep") {
        Some(sep) => try_get_value!("map", "sep", String, sep),
        None => "=".to_string(),
    };
    if sep.len() == 0 {
        sep = "=".to_string();
    }

    let mut delimit = match args.get("delimit") {
        Some(delimit) => try_get_value!("map", "delimit", String, delimit),
        None => ",".to_string(),
    };
    if delimit.len() == 0 {
        delimit = ",".to_string();
    }

    let mut r = String::with_capacity(value.len() * 64);
    let mut first = true;
    for (k, v) in value.iter() {
        if first {
            first = false;
        } else {
            r.push_str(&delimit);
        }

        r.push_str(k);
        r.push_str(&sep);
        r.push_str(&v.to_string());
    }
    return Ok(to_value(r).unwrap());
}

pub fn tera_filter_align_left(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let value = try_get_value!("align_left", "value", String, value);

    let width = match args.get("width") {
        Some(width) => try_get_value!("align_left", "width", usize, width),
        None => {
            return Err(tera::Error::msg(
                "filter `align_left` expected an arg called `width`",
            ))
        }
    };

    let r = text::align_left(&value, width);
    return Ok(to_value(r).unwrap());
}

pub fn tera_filter_align_right(
    value: &Value,
    args: &HashMap<String, Value>,
) -> tera::Result<Value> {
    let value = try_get_value!("align_right", "value", String, value);

    let width = match args.get("width") {
        Some(width) => try_get_value!("align_right", "width", usize, width),
        None => {
            return Err(tera::Error::msg(
                "filter `align_right` expected an arg called `width`",
            ))
        }
    };

    let r = text::align_right(&value, width);
    return Ok(to_value(r).unwrap());
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
