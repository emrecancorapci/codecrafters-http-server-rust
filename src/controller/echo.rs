use std::collections::HashMap;

use crate::http::{ response::{ HttpResponse, StatusCode }, request::HttpRequest };

pub fn get(http_request: &HttpRequest) -> String {
    let echo_text = http_request.request.path_array[1];

    if echo_text.is_empty() {
        return HttpResponse::from(&StatusCode::BadRequest)
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
        return HttpResponse::from(&StatusCode::Ok).headers(headers).text(echo_text).to_string();
    } else {
        return HttpResponse::from(&StatusCode::Ok).text(echo_text).to_string();
    }
}
