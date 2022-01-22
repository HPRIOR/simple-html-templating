extern crate core;

use std::path::PathBuf;

use html_templating_lib::io::*;
use html_templating_lib::io::blog_post::BlogPost;

#[test]
pub fn will_return_text_content_struct() {
    let result = get_text_content(PathBuf::from("tests/resources/test_content"));
    let content_one = BlogPost { name: String::from("text_content_one"), content: String::from("test content one") };
    let content_two = BlogPost { name: String::from("text_content_two"), content: String::from("test content two") };
    let expected = vec![content_one, content_two];
    assert_eq!(result.unwrap(), expected)
}

#[test]
pub fn will_return_empty_list_with_empty_dir() {
    let result = get_text_content(PathBuf::from("tests/resources/empty_test_content"));
    let expected: Vec<BlogPost> = vec![];
    assert_eq!(result.unwrap(), expected)
}

#[test]
pub fn will_return_error_if_wrong_path() {
    let result = match get_text_content(PathBuf::from("")) {
        Ok(_) => "_",
        Err(_) => "error"
    };
    assert_eq!(result, "error");
}
