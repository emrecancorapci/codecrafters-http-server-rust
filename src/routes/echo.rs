use crate::{
    controller::echo::get,
    http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } },
};

pub fn router(request: &HttpRequest, response: &mut HttpResponse) {
    match request.request.method {
        "GET" => { get(request, response) }
        _ => {
            response.status(StatusCode::MethodNotAllowed);
        }
    }
}
