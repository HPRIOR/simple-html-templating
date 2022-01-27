use std::path::PathBuf;

use html_templating_lib::lib;

fn replace_template_body(body_str: &str, template_str: &str) -> String {
    template_str.replace("#body#", body_str)
}

fn main() {
    lib(PathBuf::from("resources/content"), PathBuf::from("resources/template"), PathBuf::from("resources/results"));
}
