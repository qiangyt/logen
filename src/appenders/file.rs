use anyhow::{Result, Context};
use std::fs::File;
use std::io::Write;

use super::Appender;


pub struct FileAppender {
    file: File,
    path: String,
}

impl Appender for FileAppender {

    fn append(&mut self, line: crate::base::Line) -> Result<()> {
        let data = format!("{}\n", line.text);
        self.file.write_all(data.as_bytes()).with_context(|| format!("failed to write to file: {}", self.path))
    }

}

impl FileAppender {

    pub fn new(path: &str, append: bool) -> Result<FileAppender> {
        Ok(FileAppender {
            file: File::options().append(append).open(path).with_context(|| format!("failed to open file: {}", path))?,
            path: path.to_owned(),
        })
    }

}