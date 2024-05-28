use std::{ env, fs };

use crate::http::{ response, Request };

pub fn get(http_request: &Request) -> String {
    let file_name = http_request.request.path_array[1];
    let env_args: Vec<String> = env::args().collect();

    let mut directory = {
        if env_args.len() > 1 && env_args[1] == "--directory" {
            env_args[2].clone()
        } else {
            return response::bad_request("directory not found");
        }
    };

    println!("directory: {directory}");
    println!("file_name: {file_name}");

    directory.push_str(&file_name);

    let file = fs::read_to_string(directory);

    if file.is_err() {
        return response::not_found();
    }

    response::ok(&file.unwrap(), "application/octet-stream")
}
