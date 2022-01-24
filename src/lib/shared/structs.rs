#[derive(Debug, PartialEq, PartialOrd)]
pub struct HtmlPage {
    pub name: String,
    pub content: String,
}

pub struct HtmlContext {
    pub list_css: String,
    pub title_css: String,
    pub paragraph_css: String,
    pub body_replacement_string: String,
}



