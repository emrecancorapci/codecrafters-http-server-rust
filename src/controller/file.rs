use std::{ collections::HashMap, env, fs, io::Write, path::PathBuf };

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

    response::ok_body(&file.unwrap(), "application/octet-stream")
}

pub fn post(http_request: &Request) -> String {
    let file_name = http_request.request.path_array[1];
    let env_args: Vec<String> = env::args().collect();

    let directory = {
        if env_args.len() > 1 && env_args[1] == "--directory" {
            env_args[2].clone()
        } else {
            return response::bad_request("directory not found");
        }
    };

    fs::create_dir_all(&directory).unwrap();

    let path = PathBuf::from(&directory).join(file_name);
    let file = fs::File::create(path);

    if file.is_err() {
        return response::internal_server_error(&file.err().unwrap().to_string());
    }

    let result = &file.unwrap().write_all(http_request.body.as_bytes());

    match result {
        Ok(_) => response::created(),
        Err(err) => response::internal_server_error(&err.to_string()),
    }
}
