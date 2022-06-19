use logen::app::AppDef;
use handlebars::Handlebars;
use serde_json::json;

fn main() {
    let mut reg = Handlebars::new();
    // render without register
    println!("{}", reg.render_template("Hello {{name}}", &json!({"name": "foo"})).unwrap());

    // register template using given name
    reg.register_template_string("tpl_1", "Good afternoon, {{name}}").unwrap();
    println!("{}", reg.render("tpl_1", &json!({"name": "foo"})).unwrap());

    
    let mut app = AppDef::new("hello".to_string());
    app.generate();
}

