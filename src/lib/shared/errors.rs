use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct TemplateParseError;

impl std::error::Error for TemplateParseError {}

impl Display for TemplateParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error create template - could not find template replacement string")
    }
}

