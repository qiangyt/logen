use logen::app::AppDef;
use handlebars::{Handlebars};

fn main() {
    let mut handlebars = Handlebars::new();
    let app = AppDef::new("hello".to_string(), &mut handlebars);
    app.generate(&handlebars);
}

