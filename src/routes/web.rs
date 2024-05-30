use crate::{controller::web::get, http::{response::{HttpResponse, StatusCode}, request::HttpRequest}};

pub fn router(http_request: &HttpRequest) -> String {
    match http_request.request.method {
        "GET" => { get() }
        _ => { HttpResponse::from(&StatusCode::MethodNotAllowed).to_string() }
    }
}
