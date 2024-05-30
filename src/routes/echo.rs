use crate::{ controller::echo::get, http::{ response::{HttpResponse, StatusCode}, request::HttpRequest } };

pub fn router(http_request: &HttpRequest) -> String {
    match http_request.request.method {
        "GET" => { get(http_request) }
        _ => { HttpResponse::from(&StatusCode::MethodNotAllowed).to_string() }
    }
}
