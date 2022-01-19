use std::fs;

mod document_parser;

fn replace_template_body(body_str: &str, template_str: &str) -> String {
    template_str.replace("#body#", body_str)
}

fn main() {
    let template_str: String = fs::read_to_string("resources/template.html").unwrap();
    let body_str: String = fs::read_to_string("resources/body.html").unwrap();
    let result = replace_template_body(&body_str, &template_str);
    println!("{}", template_str);
    println!("{}", result);
}
