use crate::http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

pub fn get(_: &HttpRequest, response: &mut HttpResponse) {
    response.status(StatusCode::IAmATeapot).as_bytes();
}