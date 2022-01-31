extern crate core;

use std::{env, process};
use std::collections::HashMap;
use std::path::PathBuf;

use html_templating_lib::lib;

fn try_get_args(arg_list: Vec<&str>) -> Result<HashMap<&str, String>, String> {
    let args: HashMap<&str, String> =
        env::args()
            .skip(1)
            .zip(&arg_list)
            .map(|(a, b)| (*b, a))
            .collect();

    let missing_args: Vec<String> =
        arg_list
            .into_iter()
            .filter(|arg| !args.contains_key(*arg))
            .map(|arg| arg.to_owned())
            .collect();

    if missing_args.len() == 0 {
        Ok(args)
    } else {
        Err(missing_args
            .iter()
            .enumerate()
            .fold(
                String::from(""),
                |a, b| format!("{}\n{}. {}", a, b.0 + 1, b.1.replace("_", " ")))
        )
    }
}

fn main() {
    let arg_result = match try_get_args(vec!["content_dir", "template_file", "output_dir", "ctx_file"]) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Missing args: {}", e);
            process::exit(1);
        }
    };

    let result = lib(
        PathBuf::from(&arg_result["content_dir"]),
        PathBuf::from(&arg_result["template_file"]),
        PathBuf::from(&arg_result["output_dir"]),
        PathBuf::from(&arg_result["ctx_file"]),
    );
    match result {
        Ok(_) => {
            println!("finished!");
            process::exit(0)
        }

        Err(err) => {
            eprintln!("{}", err);
            process::exit(1)
        }
    };
}
