use crate::http::{ response, Request };

mod echo;
mod file;
mod user_agent;
mod web;

pub fn router(http_request: &Request) -> String {
    let path = &http_request.request.path_array;
    

    match path[0] {
        "" => { response::ok() }
        "user-agent" => { user_agent::router(http_request) }
        "echo" => { echo::router(http_request) }
        "files" => { file::router(http_request) }
        "web" => { web::router(http_request)}
        "teapot" => { response::i_am_a_teapot()}
        _ => { response::not_found() }
    }
}
