use crate::document_parser::html_body::parse_content;
use crate::shared::enums::{HtmlBody, HtmlInit};

pub mod html_body;


pub fn parse_documents(documents: &Vec<HtmlInit>) -> Vec<HtmlBody> {
    documents
        .iter()
        .map(|init| match init { HtmlInit::Of(h) => { h } })
        .map(|bp| {
            parse_content(bp.name.clone(), &bp.content)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::document_parser::parse_documents;
    use crate::shared::enums::HtmlBody::Of;
    use crate::shared::enums::HtmlInit;
    use crate::shared::structs::HtmlPage;

    #[test]
    fn parse_documents_will_produce_correct_html_bodies() {
        let blog_post_one = HtmlPage {
            name: String::from("blog_one"),
            content: String::from("line_one\nline_two\n\npara_two"),
        };
        let blog_post_two = HtmlPage {
            name: String::from("blog_two"),
            content: String::from("line_one\nline_two\n\npara_two\nline_two\n\npara_three"),
        };
        let input = vec![HtmlInit::Of(blog_post_one), HtmlInit::Of(blog_post_two)];
        let result: Vec<String> =
            parse_documents(&input)
                .into_iter()
                .map(|html| {
                    match html { Of(html_page) => html_page.content }
                })
                .collect();

        let html_body_one =
            String::from("<p >line_one\nline_two</p><p >para_two</p>");
        let html_body_two =
            String::from("<p >line_one\nline_two</p><p >para_two\nline_two</p><p >para_three</p>");

        let expected = vec![html_body_one, html_body_two];

        assert_eq!(result, expected)
    }

    #[test]
    fn parse_documents_will_produce_correct_name() {
        let blog_post_one = HtmlPage {
            name: String::from("blog_one"),
            content: String::from("line_one\nline_two\n\npara_two"),
        };
        let blog_post_two = HtmlPage {
            name: String::from("blog_two"),
            content: String::from("line_one\nline_two\n\npara_two\nline_two\n\npara_three"),
        };
        let input = vec![HtmlInit::Of(blog_post_one), HtmlInit::Of(blog_post_two)];
        let result: Vec<String> =
            parse_documents(&input)
                .into_iter()
                .map(|html| {
                    match html { Of(html_page) => html_page.name }
                })
                .collect();

        let html_name_one =
            String::from("blog_one");
        let html_name_two =
            String::from("blog_two");

        let expected = vec![html_name_one, html_name_two];

        assert_eq!(result, expected)
    }
}

