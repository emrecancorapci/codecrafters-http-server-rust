use crate::{request::{ bad_request, not_found, send_content }, HttpRequest};
use std::{ env, fs };

pub fn echo(http_request: &HttpRequest) -> String {
    let echo_text = http_request.path_array.get(1);

    match echo_text {
        None => return bad_request("echo_text not found"),
        Some(echo_text) => {
            if echo_text.is_empty() {
                return bad_request("echo_text is empty");
            }
            return send_content(echo_text, "text/plain");
        }
    }
}

pub fn files(http_request: &HttpRequest) -> String {
    let file_name = http_request.path_array[1];
    let env_args: Vec<String> = env::args().collect();

    let mut directory = {
        if env_args.len() > 1 && env_args[1] == "--directory" {
            env_args[2].clone()
        } else {
            return bad_request("directory not found");
        }
    };

    println!("directory: {directory}");

    println!("file_name: {file_name}");

    directory.push_str(&file_name);

    let file = fs::read_to_string(directory);

    if file.is_err() {
        return not_found();
    }

    return send_content(&file.unwrap(), "application/octet-stream");
}
