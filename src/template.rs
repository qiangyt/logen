use tera::Tera;
use std::collections::HashMap;

pub struct Template {
    tera: Tera,
}


impl Template {

    pub fn new() -> Template {
        let mut tera = Tera::default();
        
        // disable autoescaping completely
        tera.autoescape_on(vec![]);

        tera.register_filter("align_left", Box::new(Template::tera_filter_align_left));
        tera.register_filter("align_right", Box::new(Template::tera_filter_align_right));

        Template {tera}
    }

        
    pub fn tera_filter_align_left(value: &tera::Value, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let mut value = tera::try_get_value!("align_left", "value", String, value);

        let width = match args.get("width") {
            Some(width) => tera::try_get_value!("align_left", "width", i32, width),
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

        Ok(tera::to_value(value).unwrap())
    }

    pub fn tera_filter_align_right(value: &tera::Value, args: &HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
        let mut value = tera::try_get_value!("align_right", "value", String, value);

        let width = match args.get("width") {
            Some(width) => tera::try_get_value!("align_right", "width", i32, width),
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

        Ok(tera::to_value(value).unwrap())
    }

    pub fn add_raw_template(&mut self, name: &str, content: &str) {
        self.tera.add_raw_template(name, content)
             .expect(&format!("failed to register template {}: {}", name, content));
    }

    pub fn render(&self, template_name: &str, data: &tera::Context) -> String {
        self.tera.render(template_name, data).unwrap()
    }

}