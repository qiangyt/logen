use logen::app::App;
use tera::Tera;
use std::fs;

fn main() {
    let config_yaml = fs::read_to_string("logen.default.yaml")
        .expect("failed to open config file: logen.default.yaml");
    let app_def = serde_yaml::from_str(config_yaml.as_str())
        .expect(format!("failed to parse config yaml: {}", config_yaml).as_str());

    let mut tera = Tera::default();
    // disable autoescaping completely
    tera.autoescape_on(vec![]);

    //handlebars.register_helper("align_left", Box::new(handlebars_helper_align_left));
    //handlebars.register_helper("align_right", Box::new(handlebars_helper_align_right));
    //handlebars.register_helper("to_uppercase", Box::new(handlebars_helper_to_uppercase));
    //handlebars.register_helper("to_lowercase", Box::new(handlebars_helper_to_lowercase));

    let mut app = App::new(&app_def, &mut tera);
    app.generate(&tera);
}
/*
    let mut rendered = String::from(value);
    let mut len = 0;
    for _ in value.chars() {
        len = len + 1;
    }
    while len < width {
        rendered.push(' ');
        len = len + 1;
    }
*/
/*
    let mut rendered = String::from(value);
    let mut len = 0;
    for _ in value.chars() {
        len = len + 1;
    }
    while len < width {
        rendered.insert(0, ' ');
        len = len + 1;
    }
*/

