use chrono::{DateTime, Utc};

pub mod level;
pub use level::Level;

mod output;
pub use output::{Format as OutputFormat, Output};

pub mod tpl;
pub use tpl::{Engine as TemplateEngine, Template};

mod ts;
pub use ts::Timestamp;

#[derive(Clone)]
pub struct Line {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub text: String,
}
