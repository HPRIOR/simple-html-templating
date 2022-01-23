extern crate core;


use std::error::Error;
use std::path::PathBuf;

use crate::document_parser::parse_documents;
use crate::io::get_template;
use crate::shared::enums::HtmlFinal;
use crate::template::attach_bodies_to_template;

mod document_parser;
pub mod io;
mod template;
pub mod shared;

pub fn lib(blog_dir: PathBuf, template_dir: PathBuf) -> Result<(), &'static str> {
    let blog_posts =
        io::get_text_content(blog_dir)
            .map(|blog_posts| parse_documents(&blog_posts))
            .and_then(|html_body| {
                match get_template(template_dir) {
                    Ok(template) => attach_bodies_to_template(&html_body, &template),
                    Err(e) => Err(e)
                }
            });

    // todo
    // index page with hrefs to each file -- use blog posts to generate hyperlinks - these can be used
    // as files names and to embed into index file
    // about page
    // save files to desired directory - create directory if not exists - overwrite files in directory
    // template file

    match blog_posts {
        Ok(_) => Ok(()),
        Err(err) => Err(parse_errors(err))
    }
}

fn parse_errors(err: Box<dyn Error>) -> &'static str {
    ""
}
