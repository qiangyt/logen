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
    handlebars.register_helper("align_left", Box::new(handlebars_helper__align_left));


    let mut app = App::new(&app_def, &mut handlebars);
    app.generate(&handlebars);
}


fn handlebars_helper__align_left(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> Result<(), handlebars::RenderError> {

    let value = h.param(0)
        .ok_or(handlebars::RenderError::new("align_left(): invalid parameter 0 'value'"))?;
    
    let width = h
        .param(1)
        .and_then(|v| v.value().as_u64())
        .ok_or(handlebars::RenderError::new("align_left(): invalid parameter 1 'width'"))? as u64;
    
    let rendered = format!("{} xx", value.render());
    out.write(rendered.as_ref())?;

    Ok(())
}