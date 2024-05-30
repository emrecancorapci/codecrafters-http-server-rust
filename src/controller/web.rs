use crate::http::response::{ HttpResponse, StatusCode };

pub fn get(response: &mut HttpResponse) {
    match std::fs::read_to_string("./public/index.html") {
        Ok(index) => {
            response.status(StatusCode::Ok).body(index.into_bytes(), "text/html");
        }
        Err(err) => {
            response.status(StatusCode::NotFound).debug_msg(&err.to_string()[..]);
        }
    }
}
