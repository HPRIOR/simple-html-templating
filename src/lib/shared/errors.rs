use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct TemplateError;

impl std::error::Error for TemplateError {}

impl Display for TemplateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error create template - could not find template replacement string")
    }
}
