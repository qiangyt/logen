use handlebars::Handlebars;
use logen::app::App;
use std::fs;

fn main() {
    let config_yaml = fs::read_to_string("logen.default.yaml")
        .expect("failed to open config file: logen.default.yaml");
    let app_def = serde_yaml::from_str(config_yaml.as_str())
        .expect(format!("failed to parse config yaml: {}", config_yaml).as_str());

    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(|s| s.to_string());
    handlebars.register_helper("align_left", Box::new(handlebars_helper_align_left));
    handlebars.register_helper("align_right", Box::new(handlebars_helper_align_right));
    handlebars.register_helper("to_uppercase", Box::new(handlebars_helper_to_uppercase));
    handlebars.register_helper("to_lowercase", Box::new(handlebars_helper_to_lowercase));

    let mut app = App::new(&app_def, &mut handlebars);
    app.generate(&handlebars);
}

fn handlebars_helper_align_left(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> Result<(), handlebars::RenderError> {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(handlebars::RenderError::new(
        "align_left(): invalid parameter 0 'value'",
        ))?;

    let width = h
        .param(1)
        .and_then(|v| v.value().as_u64())
        .ok_or(handlebars::RenderError::new(
            "align_left(): invalid parameter 1 'width'",
        ))? as u64;

    let mut rendered = String::from(value);
    let mut len = 0;
    for _ in value.chars() {
        len = len + 1;
    }
    while len < width {
        rendered.push(' ');
        len = len + 1;
    }

    out.write(rendered.as_ref())?;

    Ok(())
}

fn handlebars_helper_align_right(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> Result<(), handlebars::RenderError> {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(handlebars::RenderError::new(
        "align_right(): invalid parameter 0 'value'",
        ))?;

    let width = h
        .param(1)
        .and_then(|v| v.value().as_u64())
        .ok_or(handlebars::RenderError::new(
            "align_right(): invalid parameter 1 'width'",
        ))? as u64;

    let mut rendered = String::from(value);
    let mut len = 0;
    for _ in value.chars() {
        len = len + 1;
    }
    while len < width {
        rendered.insert(0, ' ');
        len = len + 1;
    }

    out.write(rendered.as_ref())?;

    Ok(())
}


fn handlebars_helper_to_uppercase (
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> Result<(), handlebars::RenderError> {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(handlebars::RenderError::new(
        "to_uppercase(): invalid parameter 0 'value'",
        ))?;

    out.write(value.to_uppercase().as_ref())?;

    Ok(())
}


fn handlebars_helper_to_lowercase (
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> Result<(), handlebars::RenderError> {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or(handlebars::RenderError::new(
        "to_lowercase(): invalid parameter 0 'value'",
        ))?;

    out.write(value.to_lowercase().as_ref())?;

    Ok(())
}
