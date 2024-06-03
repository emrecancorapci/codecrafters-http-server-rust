use std::{ fs, io::Write, path::PathBuf };

use crate::{
    helpers::get_env_arg,
    http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } },
};

pub fn get(request: &HttpRequest, response: &mut HttpResponse) {
    let file_name = request.request.path_array
        .get(1)
        .expect("No path entered. This error shouldn't even be seen.");

    match get_env_arg("--directory") {
        Ok(dir) => {
            let path = PathBuf::from(&dir).join(file_name);

            match fs::read_to_string(path) {
                Ok(file) => {
                    response
                        .status(StatusCode::Ok)
                        .body(file.into_bytes(), "application/octet-stream");
                }
                Err(err) => {
                    response.status(StatusCode::NotFound).debug_msg(&err.to_string()[..]);
                }
            }
        }
        Err(err) => {
            response.status(StatusCode::InternalServerError).debug_msg(&err.to_string());
        }
    }
}

pub fn post(request: &HttpRequest, response: &mut HttpResponse) {
    let file_name = request.request.path_array
        .get(1)
        .expect("No path entered. This error shouldn't even be seen.");

    match get_env_arg("--directory") {
        Ok(dir) => {
            fs::create_dir_all(&dir).unwrap();

            let path = PathBuf::from(&dir).join(file_name);

            match fs::File::create(path) {
                Ok(mut file) => {
                    match file.write_all(request.body.as_bytes()) {
                        Ok(_) => {
                            response.status(StatusCode::Created);
                        }
                        Err(err) => {
                            response
                                .status(StatusCode::InternalServerError)
                                .debug_msg(&err.to_string()[..]);
                        }
                    }
                }
                Err(err) => {
                    response.status(StatusCode::BadRequest).debug_msg(&err.to_string()[..]);
                }
            }
        }
        Err(err) => {
            response.status(StatusCode::InternalServerError).debug_msg(&err.to_string());
        }
    }
}
