use std::error::Error;

use crate::HtmlFinal;
use crate::shared::errors::TemplateParseError;
use crate::shared::structs::{Context, HtmlPage};

pub fn get_index_page(template: &String, html_pages: &Vec<HtmlFinal>, ctx: &Context) -> Result<HtmlFinal, Box<dyn Error>> {
    let href_list = get_list_of_href(html_pages, ctx).join("");
    let template_replacement_str = "!body!";
    if !template.contains(template_replacement_str) {
        return Err(Box::from(TemplateParseError));
    };
    Ok(
        HtmlFinal::Of(HtmlPage {
            name: String::from("index"),
            content: template.replace(
                template_replacement_str,
                format!("<ul {}>{}</ul>", ctx.list_css, href_list).as_str(),
            ),
        })
    )
}

fn get_list_of_href(html_pages: &Vec<HtmlFinal>, ctx: &Context) -> Vec<String> {
    html_pages.into_iter().map(|html| {
        let html_pg = match html { HtmlFinal::Of(html_pg) => html_pg };
        format!("<li {}><a href=\"{}.html\" >{}</a></li>", ctx.list_css, html_pg.name.clone(), html_pg.name.clone().replace('_', " "))
    }).collect()
}


#[cfg(test)]
mod tests {
    use crate::HtmlFinal;
    use crate::shared::structs::{Context, HtmlPage};

    fn get_html_input() -> Vec<HtmlFinal> {
        let html_page_one = HtmlFinal::Of(
            HtmlPage {
                name: String::from("html_page_one"),
                content: String::from("content of html_page_one"),
            });
        let html_page_two = HtmlFinal::Of(
            HtmlPage {
                name: String::from("html_page_two"),
                content: String::from("content of html_page_two"),
            });

        vec![html_page_one, html_page_two]
    }

    fn get_ctx() -> Context {
        Context {
            list_css: "".to_string(),
            title_css: "".to_string(),
            paragraph_css: "".to_string(),
        }
    }

    mod get_list_of_href {
        use crate::html_generator::index_html::get_list_of_href;
        use crate::html_generator::index_html::tests::{get_ctx, get_html_input};

        #[test]
        fn should_return_list_of_html_with_embedded_html_name() {
            let input = get_html_input();
            let result = get_list_of_href(&input, &get_ctx());
            let expected = vec![
                String::from("<li ><a href=\"html_page_one.html\" >html page one</a></li>"),
                String::from("<li ><a href=\"html_page_two.html\" >html page two</a></li>"),
            ];

            assert_eq!(result, expected)
        }
    }

    mod get_index_page {
        use crate::html_generator::index_html::get_index_page;
        use crate::html_generator::index_html::tests::{get_ctx, get_html_input};
        use crate::HtmlFinal;
        use crate::shared::structs::HtmlPage;

        #[test]
        fn should_return_correct_html_page() {
            let html_input = get_html_input();
            let template = String::from("html !body! template");
            let result = match get_index_page(&template, &html_input, &get_ctx()) {
                Ok(result) => result,
                Err(_) => panic!()
            };

            let expected = HtmlFinal::Of(
                HtmlPage {
                    name: String::from("index"),
                    content: String::from(
                        "html <ul >\
                        <li ><a href=\"html_page_one.html\" >html page one</a></li><li ><a href=\"html_page_two.html\" >html page two</a></li>\
                        </ul> template"
                    ),
                });
            assert_eq!(result, expected)
        }

        #[test]
        fn should_replace_content_placeholder_with_no_html() {
            let html_input: Vec<HtmlFinal> = vec![];
            let template = String::from("html !body! template");
            let result = match get_index_page(&template, &html_input, &get_ctx()) {
                Ok(result) => result,
                Err(_) => panic!()
            };

            let expected = HtmlFinal::Of(
                HtmlPage {
                    name: String::from("index"),
                    content: String::from(
                        "html <ul >\
                        </ul> template"
                    ),
                });
            assert_eq!(result, expected)
        }

        #[test]
        fn should_return_err_if_no_template() {
            let html_input: Vec<HtmlFinal> = vec![];
            let template = String::from("html template");
            let result = match get_index_page(&template, &html_input, &get_ctx()) {
                Ok(_) => "ok",
                Err(_) => "error"
            };

            assert_eq!(result, "error")
        }
    }
}
