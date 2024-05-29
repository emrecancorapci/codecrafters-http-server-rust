use crate::http::{ response::{ Response, StatusCode }, Request };

mod echo;
mod file;
mod user_agent;
mod web;

pub fn router(http_request: &Request) -> String {
    let path = &http_request.request.path_array;

    match path[0] {
        "" => { Response::from(&StatusCode::Ok).to_string() }
        "user-agent" => { user_agent::router(http_request) }
        "echo" => { echo::router(http_request) }
        "files" => { file::router(http_request) }
        "web" => { web::router(http_request) }
        "teapot" => { Response::from(&StatusCode::IAmATeapot).to_string() }
        _ => { Response::from(&StatusCode::NotFound).to_string() }
    }
}
