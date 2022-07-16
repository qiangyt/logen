use logen::cfg;
use logen::tpl::TemplateEngine;

fn main() {
    if let Some(mut app) = cfg::with_cli_args().unwrap() {
        let template_engine = &mut TemplateEngine::new();
        app.init(template_engine).unwrap();
        app.generate(template_engine).unwrap();
    }
}
