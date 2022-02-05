use std::path::PathBuf;

use html_templating_lib::io;

#[test]
pub fn will_return_typed_json_context() {
    let context = io::get_context(PathBuf::from("tests/resources/context.json")).unwrap();
    assert_eq!(context.li_css, "li_css");
    assert_eq!(context.ul_css, "ul_css");
    assert_eq!(context.title_css, "title_css");
    assert_eq!(context.paragraph_css, "paragraph_css");
}

