use crate::http::{response, Request};

pub fn router(http_request: &Request) -> String {
    let user_agent = http_request.headers.get("User-Agent").unwrap();

    match http_request.request.method {
        "GET" => { response::ok(user_agent, "text/plain") }
        _ => { response::method_not_allowed() }
    }
}
