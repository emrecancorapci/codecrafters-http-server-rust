use crate::http::{ response, Request };

pub fn get(http_request: &Request) -> String {
    let echo_text = http_request.request.path_array.get(1);

    match echo_text {
        None => { response::bad_request("echo_text not found") }
        Some(echo_text) => {
            if echo_text.is_empty() {
                return response::bad_request("echo_text is empty");
            } else {
                return response::ok_text(echo_text);
            }
        }
    }
}
