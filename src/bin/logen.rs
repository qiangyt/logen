use logen::app::AppDef;
use handlebars::{Handlebars};

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(|s| s.to_string());
    let app = AppDef::new("hello".to_string(), &mut handlebars);
    app.generate(&handlebars);
}

