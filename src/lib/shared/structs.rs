use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct HtmlPage {
    pub name: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
pub struct Context {
    pub list_css: String,
    pub title_css: String,
    pub paragraph_css: String,
}






