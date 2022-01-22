extern crate core;

use std::path::PathBuf;

use crate::document_parser::parse_documents;

mod document_parser;
pub mod io;
mod template;
pub mod shared;

pub fn lib(dir: PathBuf) -> Result<(), Vec<&'static str>> {
    let mut err_list: Vec<&'static str> = vec![];

    if let Ok(blog_posts) = io::get_text_content(dir) {
        let html_bodies = parse_documents(&blog_posts);
    } else {
        err_list.push("Could not find html content")
    };

    if err_list.len() > 0 {
        Err(err_list)
    } else {
        Ok(())
    }
}
