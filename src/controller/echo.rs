use std::collections::HashMap;

use crate::http::{ response::{ Response, StatusCode }, Request };

pub fn get(http_request: &Request) -> String {
    let echo_text = http_request.request.path_array[1];

    if echo_text.is_empty() {
        return Response::from(&StatusCode::BadRequest)
            .text("echo_text is empty")
            .debug()
            .to_string();
    }

    let headers: HashMap<String, String> = {
        let mut headers = HashMap::new();
        let accept_encoding = http_request.headers.get("accept-encoding");

        match accept_encoding {
            None => headers,
            Some(encoding) => {
                if encoding != "invalid-encoding" {
                    headers.insert(String::from("Content-Encoding"), encoding.to_string());
                }
                headers
            }
        }
    };

    if headers.len() > 0 {
        return Response::from(&StatusCode::Ok).headers(headers).text(echo_text).to_string();
    } else {
        return Response::from(&StatusCode::Ok).text(echo_text).to_string();
    }
}
