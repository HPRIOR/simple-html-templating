extern crate core;


use std::path::PathBuf;

use crate::document_parser::parse_documents;
use crate::template::attach_bodies_to_template;

mod document_parser;
pub mod io;
mod template;
pub mod shared;

pub fn lib(dir: PathBuf) -> Result<(), Vec<&'static str>> {
    let result =
        io::get_text_content(dir)
            .map(|blog_posts| parse_documents(&blog_posts))
            .and_then(|html_body| attach_bodies_to_template(&html_body, &String::from("template")));

    panic!("")
}
