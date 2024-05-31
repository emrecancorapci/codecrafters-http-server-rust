use crate::http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

pub fn get(request: &HttpRequest, response: &mut HttpResponse) {
    match request.headers.get("user-agent") {
        Some(user_agent) => {
            response.status(StatusCode::Ok).body(user_agent.to_string().into_bytes(), "text/plain");
        }
        None => {
            response
                .status(StatusCode::BadRequest)
                .body("User-Agent header not found".to_string().into_bytes(), "text/plain");
        }
    }
}
