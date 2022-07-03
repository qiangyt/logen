use logen::ctx::App;
use logen::def::AppDef;
use logen::Template;
use std::fs;

fn main() {
    let config_yaml = fs::read_to_string("logen.default.yaml")
        .expect("failed to open config file: logen.default.yaml");
    let app_def: AppDef = serde_yaml::from_str(config_yaml.as_str())
        .expect(format!("failed to parse config yaml: {}", config_yaml).as_str());

    let mut template = Template::new();
    app_def.post_init(&mut template);

    let mut app = App::new(&app_def, &template);
    app.generate();
}
