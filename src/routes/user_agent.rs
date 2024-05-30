use crate::http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

pub fn router(request: &HttpRequest, response: &mut HttpResponse) {
    match request.headers.get("user-agent") {
        Some(user_agent) => {
            match request.request.method {
                "GET" => {
                    response
                        .status(StatusCode::Ok)
                        .body(user_agent.to_string().into_bytes(), "text/plain");
                }
                _ => {
                    response.status(StatusCode::MethodNotAllowed);
                }
            }
        }
        None => {
            response
                .status(StatusCode::BadRequest)
                .body("User-Agent header not found".to_string().into_bytes(), "text/plain");
        }
    }
}
