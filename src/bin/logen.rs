use logen::app::{App};
use handlebars::{Handlebars};
use std::fs;

fn main() {
    let config_yaml = fs::read_to_string("logen.default.yaml")
        .expect("failed to open config file: logen.default.yaml");
    let app_def = serde_yaml::from_str(config_yaml.as_str())
        .expect(format!("failed to parse config yaml: {}", config_yaml).as_str());

    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(|s| s.to_string());

    let mut app = App::new(&app_def, &mut handlebars);
    app.generate(&handlebars);
}

