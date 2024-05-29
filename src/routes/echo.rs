use crate::{ controller::echo::get, http::{ response::{Response, StatusCode}, Request } };

pub fn router(http_request: &Request) -> String {
    match http_request.request.method {
        "GET" => { get(http_request) }
        _ => { Response::from(&StatusCode::MethodNotAllowed).to_string() }
    }
}
