use std::error::Error;

use crate::shared::enums::{HtmlBody, HtmlFinal};
use crate::shared::errors::TemplateError;
use crate::shared::structs::HtmlPage;

pub fn attach_bodies_to_template(bodies: &Vec<HtmlBody>, template: &String) -> Result<Vec<HtmlFinal>, Box<dyn Error>> {
    let replacement_str = "!body!";
    if !template.contains(replacement_str) {
        Err(Box::from(TemplateError))
    } else {
        Ok(bodies.into_iter().map(|html_body| {
            let html = match html_body { HtmlBody::Of(html_page) => html_page };
            let content = template.replace("!body!", &html.content);
            HtmlFinal::Of(HtmlPage { name: html.name.clone(), content })
        }).collect())
    }
}


#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::shared::enums::{HtmlBody, HtmlFinal};
    use crate::shared::structs::HtmlPage;
    use crate::template::attach_bodies_to_template;

    #[test]
    pub fn will_replace_string_in_single_page() {
        let body = HtmlPage { name: String::from("test"), content: String::from("<p >This is a test paragraph</p>") };
        let template = String::from("Html !body! template");
        let result = attach_bodies_to_template(&vec![HtmlBody::Of(body)], &template);
        let expected: Result<Vec<HtmlFinal>, Box<dyn Error>> = Ok(vec![
            HtmlFinal::Of(
                HtmlPage {
                    name: String::from("test"),
                    content: String::from("Html <p >This is a test paragraph</p> template"),
                })]);
        assert_eq!(result.unwrap(), expected.unwrap())
    }

    #[test]
    pub fn will_return_err_if_with_bad_template() {
        let body = HtmlPage { name: String::from("test"), content: String::from("<p >This is a test paragraph</p>") };
        let template = String::from("Html template");
        let result = attach_bodies_to_template(&vec![HtmlBody::Of(body)], &template);
        let expected = match result {
            Ok(_) => { "" }
            Err(_err) => { "error" }
        };
        assert_eq!(expected, "error")
    }
}



