use crate::http::{ response::{ Response, StatusCode }, Request };

pub fn router(http_request: &Request) -> String {
    let user_agent = http_request.headers.get("user-agent");

    if user_agent.is_none() {
        return Response::from(&StatusCode::BadRequest)
            .text("User-Agent header not found")
            .debug()
            .to_string();
    }

    match http_request.request.method {
        "GET" => { Response::from(&StatusCode::Ok).text(user_agent.unwrap()).to_string() }
        _ => { Response::from(&StatusCode::MethodNotAllowed).to_string() }
    }
}
