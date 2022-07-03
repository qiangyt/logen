use crate::Template;
use super::App;

pub struct Line<'a> {
    pub app: &'a App<'a>,
    pub index: u64,
    pub data: tera::Context,
    pub template: &'a Template,
}

impl<'a> Line<'a> {

}
