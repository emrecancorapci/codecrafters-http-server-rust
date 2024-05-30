use std::{ fs, io::Write, path::PathBuf };

use crate::{
    helpers::get_env_arg,
    http::{ response::{ HttpResponse, StatusCode }, request::HttpRequest },
};

pub fn get(http_request: &HttpRequest) -> String {
    let file_name = http_request.request.path_array[1];
    let directory = get_env_arg("--directory");

    let path = PathBuf::from(&directory).join(file_name);
    let file = fs::read_to_string(path);

    match file {
        Ok(file) => {
            return HttpResponse::from(&StatusCode::Ok)
                .body(&file, "application/octet-stream")
                .to_string();
        }
        Err(err) => {
            return HttpResponse::from(&StatusCode::NotFound)
                .text(&err.to_string())
                .debug()
                .to_string();
        }
    }
}

pub fn post(http_request: &HttpRequest) -> String {
    let file_name = http_request.request.path_array[1];
    let directory = get_env_arg("--directory");

    fs::create_dir_all(&directory).unwrap();

    let path = PathBuf::from(&directory).join(file_name);
    let file = fs::File::create(path);

    if file.is_err() {
        return HttpResponse::from(&StatusCode::BadRequest)
            .text(&file.err().unwrap().to_string())
            .debug()
            .to_string();
    }

    let result = &file.unwrap().write_all(http_request.body.as_bytes());

    match result {
        Ok(_) => HttpResponse::from(&StatusCode::Created).to_string(),
        Err(err) =>
            HttpResponse::from(&StatusCode::InternalServerError)
                .text(&err.to_string())
                .debug()
                .to_string(),
    }
}
