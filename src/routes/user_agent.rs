use crate::http::{ response::{ Response, StatusCode }, Request };

pub fn router(http_request: &Request) -> String {
    let user_agent = http_request.headers.get("User-Agent").unwrap();

    match http_request.request.method {
        "GET" => { Response::from(&StatusCode::Ok).text(user_agent).to_string() }
        _ => { Response::from(&StatusCode::MethodNotAllowed).to_string() }
    }
}
