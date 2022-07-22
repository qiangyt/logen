use crate::Line;

use anyhow::Result;
pub mod console;
pub mod file;


pub trait Appender {

    fn append(&mut self, line: Line) -> Result<()>;

}