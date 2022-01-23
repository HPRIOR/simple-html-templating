use crate::shared::structs::HtmlPage;

// make illegal states unrepresentable
#[derive(Debug, PartialEq, PartialOrd)]
pub enum HtmlInit { Of(HtmlPage) }

pub enum HtmlBody { Of(HtmlPage) }

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HtmlTemplate { Of(HtmlPage) }

pub enum HtmlCombined { Of(HtmlPage) }
