use crate::{ controller::echo::get, http::{ response, Request } };

pub fn router(http_request: &Request) -> String {
    match http_request.request.method {
        "GET" => { get(http_request) }
        _ => { response::method_not_allowed() }
    }
}
