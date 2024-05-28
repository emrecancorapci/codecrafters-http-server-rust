use crate::{controller::web::get, http::{response, Request}};

pub fn router(http_request: &Request) -> String {
    match http_request.request.method {
        "GET" => { get() }
        _ => { response::method_not_allowed() }
    }
}
