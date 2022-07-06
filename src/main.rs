use logen::ctx::App;
use logen::config;


fn main() {
    if let Some(def) = config::with_cli_args().unwrap() {
        let mut app = App::new(&def).unwrap();
        app.generate().unwrap();
    }    
}
