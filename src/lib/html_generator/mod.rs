use std::error::Error;

use crate::html_generator::document_parser::parse_documents;
use crate::html_generator::index_html::get_index_page;
use crate::html_generator::template::attach_bodies_to_template;
use crate::HtmlFinal;
use crate::shared::enums::HtmlInit;

mod index_html;
mod template;
mod document_parser;

pub fn generate_html(html_content: &Vec<HtmlInit>, template: &String) -> Result<Vec<HtmlFinal>, Box<dyn Error>> {
    let html_bodies = parse_documents(&html_content);
    let mut html_pages: Vec<HtmlFinal> = attach_bodies_to_template(&html_bodies, template)?;
    let index_page = get_index_page(&template, &html_pages)?;
    html_pages.push(index_page);
    return Ok(html_pages)
}
