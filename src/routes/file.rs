use crate::{ controller::file::{get, post}, http::{ response::{HttpResponse, StatusCode}, request::HttpRequest } };

pub fn router(http_request: &HttpRequest) -> String {
    match http_request.request.method {
        "GET" => { get(http_request) }
        "POST" => { post(http_request) }
        _ => { HttpResponse::from(&StatusCode::MethodNotAllowed).to_string() }
    }
}
