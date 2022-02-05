extern crate core;

use std::path::PathBuf;

use html_templating_lib::io::*;
use html_templating_lib::shared::enums::HtmlInit;
use html_templating_lib::shared::structs::HtmlPage;

#[test]
pub fn will_return_text_content_struct() {
    let result = get_text_content(PathBuf::from("tests/resources/test_content"));
    let content_one = HtmlPage { name: String::from("text_content_one"), content: String::from("test content one") };
    let content_two = HtmlPage { name: String::from("text_content_two"), content: String::from("test content two") };
    let expected = vec![HtmlInit::Of(content_one), HtmlInit::Of(content_two)];
    assert_eq!(result.unwrap(), expected)
}


#[test]
pub fn will_return_error_if_wrong_path() {
    let result = match get_t
    ext_content(PathBuf::from(""))
    {
        Ok(_) => "_",
        Err(_) => "error"
    };
    assert_eq!(result, "error");
}
