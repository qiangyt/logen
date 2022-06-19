use logen::app::AppDef;
use serde_json::json;

fn main() {
    let mut app = AppDef::new("hello".to_string());
    app.generate();
}

