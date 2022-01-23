use std::error::Error;
use std::fs;
use std::fs::DirEntry;
use std::io::Read;
use std::path::PathBuf;

use crate::shared::enums::HtmlInit;
use crate::shared::structs::HtmlPage;

fn get_file_name_from_entry(entry: &DirEntry) -> String {
    String::from(
        entry
            .file_name()
            .to_str()
            .unwrap()
    ).split(".").take(1).collect()
}

pub fn get_text_content(dir: PathBuf) -> Result<Vec<HtmlInit>, Box<dyn Error>> {
    let entries = fs::read_dir(dir)?;
    let mut result: Vec<HtmlInit> = Vec::new();
    for entry in entries {
        let entry = entry?;
        let file_name = get_file_name_from_entry(&entry);
        let file_path = entry.path();
        let blog_post = HtmlPage {
            name: file_name,
            content: fs::read_to_string(file_path)?.to_owned(),
        };
        result.push(HtmlInit::Of(blog_post))
    };
    Ok(result)
}


pub fn get_template(dir: PathBuf) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(dir)?)
}

