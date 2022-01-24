extern crate core;


use std::error::Error;
use std::path::PathBuf;

use crate::html_generator::generate_html;
use crate::io::get_template;
use crate::shared::enums::HtmlFinal;

pub mod io;
pub mod shared;
mod html_generator;

pub fn lib(blog_dir: PathBuf, template_dir: PathBuf) -> Result<(), &'static str> {
    let html_files =
        io::get_text_content(blog_dir)
            .and_then(|blogs| {
                let template = get_template(template_dir)?;
                generate_html(&blogs, &template)
            });


    match html_files {
        Ok(_) => Ok(()),
        Err(err) => Err(parse_errors(err))
    }
}

fn parse_errors(err: Box<dyn Error>) -> &'static str {
    ""
}
