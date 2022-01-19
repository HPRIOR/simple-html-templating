pub struct HtmlDoc {
    name: String,
    content: String,
}

impl HtmlDoc {
    fn parse_content(content: &String) -> String {
        String::from("hello")
    }

    pub fn new(name: String, content: String) -> Self {
        let parsed_content = HtmlDoc::parse_content(&content);
        Self {
            name,
            content: parsed_content,
        }
    }
}

fn get_paragraphs_from(content: &String) -> Vec<String> {
    panic!("not impl")
}

fn wrap(paras: &Vec<String>) -> String {
    panic!("not impl")
}

fn inject_css() -> String {
    panic!("not impl")
}

#[cfg(test)]
mod tests {
    mod get_paragraphs_from_tests{
        #[test]
        fn sanity(){
            assert!(true)
        }
    }
}

