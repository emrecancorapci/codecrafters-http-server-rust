use crate::{ request, HttpRequest, controller::echo::get };

pub fn router(http_request: &HttpRequest) -> String {
    match http_request.request.method {
        "GET" => { get(http_request) }
        _ => { request::method_not_allowed() }
    }
}
