use std::{ env, fs, io::Write, path::PathBuf };

use crate::http::{ response::{ Response, StatusCode }, Request };

pub fn get(http_request: &Request) -> String {
    let file_name = http_request.request.path_array[1];
    let env_args: Vec<String> = env::args().collect();

    let mut directory = {
        if env_args.len() > 1 && env_args[1] == "--directory" {
            env_args[2].clone()
        } else {
            return Response::from(&StatusCode::BadRequest)
                .text("directory not found")
                .debug()
                .to_string();
        }
    };

    println!("directory: {directory}");
    println!("file_name: {file_name}");

    directory.push_str(&file_name);

    let file = fs::read_to_string(directory);

    if file.is_err() {
        return Response::from(&StatusCode::NotFound)
            .text(&file.err().unwrap().to_string())
            .debug()
            .to_string();
    }

    Response::from(&StatusCode::Ok).body(&file.unwrap(), "application/octet-stream").to_string()
}

pub fn post(http_request: &Request) -> String {
    let file_name = http_request.request.path_array[1];
    let env_args: Vec<String> = env::args().collect();

    let directory = {
        if env_args.len() > 1 && env_args[1] == "--directory" {
            env_args[2].clone()
        } else {
            return Response::from(&StatusCode::BadRequest)
                .text("directory not found")
                .debug()
                .to_string();
        }
    };

    fs::create_dir_all(&directory).unwrap();

    let path = PathBuf::from(&directory).join(file_name);
    let file = fs::File::create(path);

    if file.is_err() {
        return Response::from(&StatusCode::BadRequest)
            .text(&file.err().unwrap().to_string())
            .debug()
            .to_string();
    }

    let result = &file.unwrap().write_all(http_request.body.as_bytes());

    match result {
        Ok(_) => Response::from(&StatusCode::Created).to_string(),
        Err(err) =>
            Response::from(&StatusCode::InternalServerError)
                .text(&err.to_string())
                .debug()
                .to_string(),
    }
}
