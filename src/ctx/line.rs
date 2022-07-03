use crate::template::Template;
use crate::ctx::app::App;

pub struct Line<'a> {
    pub app: &'a App<'a>,
    pub index: u64,
    pub data: tera::Context,
    pub template: &'a Template,
}

impl<'a> Line<'a> {

}
