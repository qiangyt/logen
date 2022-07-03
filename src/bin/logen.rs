use logen::app::App;
use logen::logger::template::Template;
use serde_json::value::Value;
use tera::Tera;
use std::fs;
use std::collections::HashMap;

fn main() {
    let config_yaml = fs::read_to_string("logen.default.yaml")
        .expect("failed to open config file: logen.default.yaml");
    let app_def = serde_yaml::from_str(config_yaml.as_str())
        .expect(format!("failed to parse config yaml: {}", config_yaml).as_str());

    let mut tmpl = Template::new();
    
   // tera.register_filter("to_upper", filter);

    let mut app = App::new(&app_def, &mut tmpl);
    app.generate(&tmpl);
}
