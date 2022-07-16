use logen::config;
use logen::ctx::app::App;
use logen::template::TemplateEngine;

fn main() {
    if let Some(mut def) = config::with_cli_args().unwrap() {
        let mut template_engine = TemplateEngine::new();
        def.post_init(&mut template_engine).unwrap();
        let mut app = App::new(&def, &mut template_engine).unwrap();
        app.generate().unwrap();
    }
}
