use crate::document_parser::html_fragment::HtmlBody;
use crate::io::blog_post::BlogPost;

mod html_fragment;


pub fn parse_documents(documents: &Vec<BlogPost>) -> Vec<HtmlBody> {
    documents
        .iter()
        .map(|bp| {
            HtmlBody::new(bp.name.clone(), &bp.content)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::document_parser::parse_documents;
    use crate::io::blog_post::BlogPost;

    #[test]
    fn parse_documents_will_produce_correct_html_bodies() {
        let blog_post_one = BlogPost {
            name: String::from("blog_one"),
            content: String::from("line_one\nline_two\n\npara_two"),
        };
        let blog_post_two = BlogPost {
            name: String::from("blog_two"),
            content: String::from("line_one\nline_two\n\npara_two\nline_two\n\npara_three"),
        };
        let input = vec![blog_post_one, blog_post_two];
        let result: Vec<String> =
            parse_documents(&input)
                .into_iter()
                .map(|html| html.get_content().clone())
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
        let blog_post_one = BlogPost {
            name: String::from("blog_one"),
            content: String::from("line_one\nline_two\n\npara_two"),
        };
        let blog_post_two = BlogPost {
            name: String::from("blog_two"),
            content: String::from("line_one\nline_two\n\npara_two\nline_two\n\npara_three"),
        };
        let input = vec![blog_post_one, blog_post_two];
        let result: Vec<String> =
            parse_documents(&input)
                .into_iter()
                .map(|html| html.get_name().clone())
                .collect();

        let html_name_one =
            String::from("blog_one");
        let html_name_two =
            String::from("blog_two");

        let expected = vec![html_name_one, html_name_two];

        assert_eq!(result, expected)
    }
}

