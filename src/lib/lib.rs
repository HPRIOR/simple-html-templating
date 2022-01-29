extern crate core;


use std::error::Error;
use std::path::PathBuf;

use crate::html_generator::generate_html;
use crate::io::get_template;
use crate::shared::enums::HtmlFinal;

pub mod io;
pub mod shared;
mod html_generator;

pub fn lib(blog_dir: PathBuf, template_dir: PathBuf, save_dir: PathBuf) -> Result<(), String> {
    let html_files =
        io::get_text_content(blog_dir)
            .and_then(|blogs| {
                let template = get_template(template_dir)?;
                generate_html(&blogs, &template)
            });

    if let Ok(html_files) = html_files {
        let save_result = io::save_html(save_dir, &html_files);
        if let Ok(has_saved) = save_result {
            Ok(has_saved)
        } else {
            //let err = save_result.err().unwrap();
            //let error_msg = format!("failed to save content files: {}", err).to_string();
            Err("error occurred saving html files")
        }
    } else {
        match html_files {
            Ok(_) => Ok(()),
            Err(err) => Err(parse_errors(err))
        }
    }
}

fn parse_errors(err: Box<dyn Error>) -> String {
    format!("An error has occurred: {}", err)
}
