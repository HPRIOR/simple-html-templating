extern crate core;


use std::error::Error;
use std::path::PathBuf;

use crate::html_generator::generate_html;
use crate::shared::enums::HtmlFinal;

pub mod io;
pub mod shared;
mod html_generator;

pub fn lib(blog_dir: PathBuf, template_dir: PathBuf, save_dir: PathBuf, ctx_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let ctx = io::get_context(ctx_path)?;

    let template = io::get_template(template_dir)?;
    let html_content = io::get_text_content(blog_dir)?;

    let html = generate_html(&html_content, &template, &ctx)?;


    Ok(io::save_html(save_dir, &html)?)
}

