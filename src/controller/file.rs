use crate::{ request::{ bad_request, not_found, send_content }, HttpRequest };
use std::{ env, fs };

pub fn get(http_request: &HttpRequest) -> String {
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
