use chrono::{DateTime, Utc};

pub mod level;
pub use level::Level;

mod output;
pub use output::{Output, Format as OutputFormat};

pub mod tpl;
pub use tpl::{Template, Engine as TemplateEngine};

mod ts;
pub use ts::Timestamp;


pub struct Line {
    pub name: String,
    pub timestamp: DateTime<Utc>,
    pub text: String,
}
