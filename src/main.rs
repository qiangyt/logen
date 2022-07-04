use std::fs;

use logen::def::AppDef;
use logen::ctx::App;


fn main() {
    let config_yaml = fs::read_to_string("logen.default.yaml")
        .expect("failed to open config file: logen.default.yaml");

    let def: AppDef = serde_yaml::from_str(&config_yaml)
        .expect(&format!("failed to parse config yaml: {}", config_yaml));

    let mut app = App::new(&def).unwrap();
    app.generate().unwrap();
}
