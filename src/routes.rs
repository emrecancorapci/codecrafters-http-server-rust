use crate::http::{ response::{ HttpResponse, StatusCode }, request::HttpRequest };

mod echo;
mod file;
mod user_agent;
mod web;

pub fn router(http_request: &HttpRequest) -> String {
    let path = &http_request.request.path_array;

    match path[0] {
        "" => { HttpResponse::from(&StatusCode::Ok).to_string() }
        "user-agent" => { user_agent::router(http_request) }
        "echo" => { echo::router(http_request) }
        "files" => { file::router(http_request) }
        "web" => { web::router(http_request) }
        "teapot" => { HttpResponse::from(&StatusCode::IAmATeapot).to_string() }
        _ => { HttpResponse::from(&StatusCode::NotFound).to_string() }
    }
}
