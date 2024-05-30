use crate::{
    controller::file::{ get, post },
    http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } },
};

pub fn router(request: &HttpRequest, response: &mut HttpResponse) {
    match request.request.method {
        "GET" => { get(request, response) }
        "POST" => { post(request, response) }
        _ => {
            response.status(StatusCode::MethodNotAllowed);
        }
    }
}
