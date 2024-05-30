use crate::http::{ request::HttpRequest, response::{HttpResponse, StatusCode} };

mod echo;
mod file;
mod user_agent;
mod web;

pub fn router(request: &HttpRequest, response: &mut HttpResponse) {
    let path = &request.request.path_array;

    match path[0] {
        "" => { response.status(StatusCode::Ok).as_bytes(); }
        "user-agent" => { user_agent::router(request, response) }
        "echo" => { echo::router(request, response) }
        "files" => { file::router(request, response) }
        "web" => { web::router(request, response) }
        "teapot" => { response.status(StatusCode::IAmATeapot).as_bytes(); }
        _ => { response.status(StatusCode::NotFound).as_bytes(); }
    }
}
