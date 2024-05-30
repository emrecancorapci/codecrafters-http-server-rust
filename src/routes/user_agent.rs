use crate::http::{ response::{ HttpResponse, StatusCode }, request::HttpRequest };

pub fn router(http_request: &HttpRequest) -> String {
    let user_agent = http_request.headers.get("user-agent");

    if user_agent.is_none() {
        return HttpResponse::from(&StatusCode::BadRequest)
            .text("User-Agent header not found")
            .debug()
            .to_string();
    }

    match http_request.request.method {
        "GET" => { HttpResponse::from(&StatusCode::Ok).text(user_agent.unwrap()).to_string() }
        _ => { HttpResponse::from(&StatusCode::MethodNotAllowed).to_string() }
    }
}
