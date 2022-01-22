use std::path::PathBuf;

use html_templating_lib::io::*;
use html_templating_lib::io::blog_post::BlogPost;


#[test]
pub fn get_text_content_will_return_text_content_struct(){
    let result = get_text_content(PathBuf::from("/Users/harryprior/Code/simple-html-templating/tests/resources/test_content"));
    let content_one = BlogPost{name: String::from("text_content_one"), content:String::from("test content one")};
    let content_two = BlogPost{name: String::from("text_content_two"), content:String::from("test content two")};
    let expected = vec![content_one, content_two];
    assert_eq!(result.unwrap(), expected)
}
