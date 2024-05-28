use crate::{request::{ bad_request, send_content }, HttpRequest };

pub fn get(http_request: &HttpRequest) -> String {
    let echo_text = http_request.request.path_array.get(1);

    match echo_text {
        None => return bad_request("echo_text not found"),
        Some(echo_text) => {
            if echo_text.is_empty() {
                return bad_request("echo_text is empty");
            }
            return send_content(echo_text, "text/plain");
        }
    }
}