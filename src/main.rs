use logen::cfg;

fn main() {
    if let Some(app) = cfg::with_cli_args().unwrap() {
        app.generate().unwrap();
    }
}
