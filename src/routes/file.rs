use crate::{ controller::file::{get, post}, http::{ response, Request } };

pub fn router(http_request: &Request) -> String {
    match http_request.request.method {
        "GET" => { get(http_request) }
        "POST" => { post(http_request) }
        _ => { response::method_not_allowed() }
    }
}
