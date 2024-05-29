use crate::{ controller::file::{get, post}, http::{ response::{Response, StatusCode}, Request } };

pub fn router(http_request: &Request) -> String {
    match http_request.request.method {
        "GET" => { get(http_request) }
        "POST" => { post(http_request) }
        _ => { Response::from(&StatusCode::MethodNotAllowed).to_string() }
    }
}
