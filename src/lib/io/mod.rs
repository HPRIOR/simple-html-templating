use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::error::Error;
use std::fs::DirEntry;

use self::blog_post::BlogPost;

pub mod blog_post;

fn get_file_name_from_entry(entry: &DirEntry) -> String{
    String::from(
        entry
            .file_name()
            .to_str()
            .unwrap()
    ).split(".").take(1).collect()
}

pub fn get_text_content(dir: PathBuf) -> Result<Vec<BlogPost>, Box<dyn Error>> {
    let entries = fs::read_dir(dir)?;
    let mut result: Vec<BlogPost> = Vec::new();
    for entry in entries {
        let entry = entry?;
        let file_name = get_file_name_from_entry(&entry);
        let file_path = entry.path();
        let blog_post = BlogPost {
            name: file_name,
            content: fs::read_to_string(file_path)?.to_owned(),
        };
        result.push(blog_post)
    };
    Ok(result)
}

