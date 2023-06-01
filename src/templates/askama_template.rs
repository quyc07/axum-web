use crate::school::{Gender, Student};
use askama::Template;
use std::fmt::{Debug, Display, Formatter, Write};

#[derive(Template)]
#[template(path = "twitter.html")]
pub struct TwitterTemplate {}

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    // data to be passed to the template
    pub(crate) students: Vec<Student>,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name().as_str())
    }
}
