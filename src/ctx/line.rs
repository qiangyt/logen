use serde::Serialize;
use serde_json::to_value;

use crate::Template;

pub struct Line<'a> {
    data: &'a mut tera::Context,
    template: &'a Template,
}

impl<'a> Line<'a> {

    pub fn new(index: u64, data: &'a mut tera::Context, template: &'a Template) -> Line<'a> {
        let mut r = Line {data, template};
        r.var("index", &index);
        r
    }

    pub fn var<T: Serialize + ?Sized, S: Into<String>>(&mut self, key: S, val: &T) {
        self.data.insert(key.into(), &to_value(val).unwrap());
    }    

    pub fn render(&self, template_name: &str) -> String {
        self.template.render(template_name, &self.data)
    }

}
