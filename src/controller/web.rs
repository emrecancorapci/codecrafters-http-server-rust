use crate::http::{request::HttpRequest, response::{ HttpResponse, StatusCode }};

pub fn get(request: &HttpRequest, response: &mut HttpResponse) {
    match std::fs::read_to_string("./public/index.html") {
        Ok(index) => {
            response.status(StatusCode::Ok).body(index.into_bytes(), "text/html");
        }
        Err(err) => {
            response.status(StatusCode::NotFound).debug_msg(&err.to_string()[..]);
        }
    }
}
