use crate::http::{ response, Request };

mod echo;
mod file;

pub fn router(http_request: &Request) -> String {
    let path = &http_request.request.path_array;

    match path[0] {
        "" => { response::ok("", "") }
        "user-agent" => { response::ok(http_request.user_agent, "text/plain") }
        "echo" => { echo::router(http_request) }
        "files" => { file::router(http_request) }
        _ => { response::not_found() }
    }
}
