use logen::assets::Asset;
use logen::def::AppDef;
use logen::ctx::App;

use std::fs;
use std::str;

fn main() {
    let default_config_file = Asset::get("logen.config.default.yaml");
    let config_yaml = String::from_utf8(default_config_file.unwrap().data.as_ref().to_vec()).unwrap();

    //let config_yaml = fs::read_to_string("logen.default.yaml")
    //    .expect("failed to open config file: logen.default.yaml");

    let def: AppDef = serde_yaml::from_str(&config_yaml)
        .expect(&format!("failed to parse config yaml: {}", config_yaml));

    let mut app = App::new(&def).unwrap();
    app.generate().unwrap();
}
