use crate::http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

pub fn get(request: &HttpRequest, response: &mut HttpResponse) {
    response.status(StatusCode::Ok).as_bytes();
}