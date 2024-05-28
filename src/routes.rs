use crate::{ controller, http::{ response, Request } };

mod echo;

pub fn router(http_request: &Request) -> String {
    match http_request.request.path_array[0] {
        "" => { response::ok("", "") }
        "user-agent" => { response::ok(http_request.user_agent, "text/plain") }
        "echo" => { echo::router(http_request) }
        "files" => { controller::file::get(http_request) }
        _ => { response::not_found() }
    }
}
