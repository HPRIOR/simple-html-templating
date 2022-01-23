use crate::shared::enums::{HtmlBody, HtmlTemplate};
use crate::shared::structs::HtmlPage;

pub fn attach_bodies_to_template(bodies: &Vec<HtmlBody>, template: &String) -> Vec<HtmlTemplate> {
    bodies.into_iter().map(|html_body| {
        let html = match html_body { HtmlBody::Of(html_page) => html_page };
        let content = template.replace("!body!", &html.content);
        HtmlTemplate::Of(HtmlPage { name: html.name.clone(), content })
    }).collect()
}


#[cfg(test)]
mod tests {
    use crate::shared::enums::{HtmlBody, HtmlTemplate};
    use crate::shared::structs::HtmlPage;
    use crate::template::attach_bodies_to_template;

    #[test]
    pub fn will_replace_string_in_single_page() {
        let body = HtmlPage { name: String::from("test"), content: String::from("<p >This is a test paragraph</p>") };
        let template = String::from("Html !body! template");
        let result = attach_bodies_to_template(&vec![HtmlBody::Of(body)], &template);
        let expected = vec![
            HtmlTemplate::Of(
                HtmlPage {
                    name: String::from("test"),
                    content: String::from("Html <p >This is a test paragraph</p> template"),
                })];
        assert_eq!(result, expected)
    }
}



