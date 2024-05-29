use crate::http::{ response::{ Response, StatusCode }, Request };

pub fn get(http_request: &Request) -> String {
    let echo_text = http_request.request.path_array.get(1);

    match echo_text {
        None => {
            return Response::from(&StatusCode::BadRequest)
                .text("echo_text is missing")
                .debug()
                .to_string();
        }
        Some(echo_text) => {
            if echo_text.is_empty() {
                return Response::from(&StatusCode::BadRequest)
                    .text("echo_text is empty")
                    .debug()
                    .to_string();
            } else {
                return Response::from(&StatusCode::Ok).text(echo_text).to_string();
            }
        }
    }
}
