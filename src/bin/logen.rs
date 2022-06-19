use std::str::FromStr;

use logen::app::AppDef;

fn main() {
    let mut app = AppDef::new("hello".to_string());
    app.generate();
}

