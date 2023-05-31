use sailfish::runtime::{Buffer, escape, Render};
use sailfish::{RenderError, TemplateOnce};
use crate::school::{Gender, Student};

#[derive(TemplateOnce)]  // automatically implement `TemplateOnce` trait
#[template(path = "hello.stpl")]  // specify the path to template
pub struct HelloTemplate {
    // data to be passed to the template
    pub(crate) students: Vec<Student>,
}

impl Render for Gender {
    fn render(&self, b: &mut Buffer) -> Result<(), RenderError> {
        b.push_str(self.name().as_str());
        Ok(())
    }

    fn render_escaped(&self, b: &mut Buffer) -> Result<(), RenderError> {
        escape::escape_to_buf(self.name().as_str(), b);
        Ok(())
    }
}

#[derive(TemplateOnce)]
#[template(path = "china.stpl")]
pub struct ChinaTemplate {}