#[derive(Debug, PartialOrd, PartialEq)]
pub struct HtmlBody {
    name: String,
    content: String,
}

impl HtmlBody {
    pub fn new(name: String, content: &String) -> Self {
        Self {
            name,
            content: parse_content(&content),
        }
    }
    pub fn get_content(&self) -> &String {
        &self.content
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}

fn parse_content(content: &String) -> String {
    let paragraphs = get_paragraphs_from(&content);
    wrap(&paragraphs, inject_css).join("")
}

fn get_paragraphs_from(content: &String) -> Vec<String> {
    content
        .split("\n\n")
        .map(|line| line.trim_matches('\n'))
        .filter(|line| line.chars().count() > 0)
        .map(|s| s.to_string())
        .collect()
}

fn wrap(paras: &Vec<String>, with: fn() -> String) -> Vec<String> {
    paras
        .iter()
        .map(|s| format!("<p {}>{}</p>", with(), s))
        .collect()
}

fn inject_css() -> String {
    "".to_owned()
}

#[cfg(test)]
mod tests {
    mod get_paragraphs_from {
        use crate::document_parser::html_fragment::get_paragraphs_from;

        #[test]
        fn will_split_by_double_line_break() {
            let content = String::from("hello my name is \nHarry \n\n this is a new paragraph");
            let result = get_paragraphs_from(&content);
            let expected = vec!["hello my name is \nHarry ", " this is a new paragraph"];
            assert_eq!(result, expected)
        }

        #[test]
        fn will_ignore_multiple_line_paragraph_gap() {
            let content =
                String::from("hello my name is \nHarry \n\n\n\n\n this is a new paragraph");
            let result = get_paragraphs_from(&content);
            let expected = vec!["hello my name is \nHarry ", " this is a new paragraph"];
            assert_eq!(result, expected)
        }

        #[test]
        fn will_ignore_multiple_paras_with_no_spaces() {
            let content =
                String::from("line_one\nline_two\n\npara_two\nline_two\n\npara_three");
            let result = get_paragraphs_from(&content);
            let expected = vec!["line_one\nline_two", "para_two\nline_two", "para_three"];
            assert_eq!(result, expected)
        }
    }

    mod wrap {
        use crate::document_parser::html_fragment::wrap;

        #[test]
        fn will_wrap_simple_paragraphs() {
            let arg: Vec<String> = vec![String::from("1"), String::from("2")];
            let result = wrap(&arg, || String::from(""));
            let expected = vec![String::from("<p >1</p>"), String::from("<p >2</p>")];
            assert_eq!(result, expected)
        }

        #[test]
        fn will_wrap_more_complex_paragraphs() {
            let arg: Vec<String> = vec![
                String::from("line_one\nline_two"),
                String::from("para_two\nline_two"),
                String::from("para_three")];
            let result = wrap(&arg, || String::from(""));
            let expected = vec![
                String::from("<p >line_one\nline_two</p>"),
                String::from("<p >para_two\nline_two</p>"),
                String::from("<p >para_three</p>")];
            assert_eq!(result, expected)
        }

        #[test]
        fn will_inject_node_() {
            let arg: Vec<String> = vec![String::from("1"), String::from("2")];
            let result = wrap(&arg, || String::from("att"));
            let expected = vec![String::from("<p att>1</p>"), String::from("<p att>2</p>")];
            assert_eq!(result, expected)
        }
    }

    mod html_body {
        use crate::document_parser::html_fragment::HtmlBody;

        #[test]
        fn will_be_constructed_from_document() {
            let content =
                String::from("this is a string \nthis is another line \n\nthis is a new paragraph");
            let html_obj = HtmlBody::new("test".to_owned(), &content);
            let expected = String::from(
                "<p >this is a string \nthis is another line </p><p >this is a new paragraph</p>",
            );
            assert_eq!(html_obj.get_content(), &expected)
        }
    }
}
