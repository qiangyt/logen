use logen::config;
use logen::template::TemplateEngine;

fn main() {
    if let Some(mut app) = config::with_cli_args().unwrap() {
        let template_engine = &mut TemplateEngine::new();
        app.init(template_engine).unwrap();
        app.generate(template_engine).unwrap();
    }
}
