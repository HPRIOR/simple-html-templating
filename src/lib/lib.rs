use std::error::Error;
use std::io::{stderr, Write};
use std::path::PathBuf;

mod document_parser;
pub mod io;

pub fn lib(dir: PathBuf) -> Result<(), Vec<&'static str>> {
    let mut err_list: Vec<&'static str> = vec![];

    if let Ok(text_content) = io::get_text_content(dir) {

    } else {
        err_list.push("Could not find html content")
    };

    if err_list.len() > 0 {
        Err(err_list)
    } else {
        Ok(())
    }
}
