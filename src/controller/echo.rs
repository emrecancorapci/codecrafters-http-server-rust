use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    extensions::hash_map::HashMapExt,
    http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } },
};

pub fn get(http_request: &HttpRequest) -> String {
    let echo_text = http_request.request.path_array[1];

    if echo_text.is_empty() {
        return HttpResponse::from(&StatusCode::BadRequest)
            .text("echo_text is empty")
            .debug()
            .to_string();
    }

    match get_encoding(http_request) {
        Some(encoders) => {
            println!("Encodings : {}", encoders.to_string());
            return HttpResponse::from(&StatusCode::Ok)
                .text(echo_text)
                .headers(encoders)
                .to_string();
        }
        None => {
            return HttpResponse::from(&StatusCode::Ok).text(echo_text).to_string();
        }
    }
}

fn get_encoding(http_request: &HttpRequest) -> Option<HashMap<String, String>> {
    let mut headers: HashMap<String, String> = HashMap::new();
    let accept_encoding = http_request.headers.get("accept-encoding");

    if accept_encoding.is_some() {
        let encodings = accept_encoding.unwrap().split(',').collect_vec();

        for encoding in encodings {
            if encoding.trim() == "gzip" {
                headers.insert(String::from("Content-Encoding"), String::from("gzip"));
                break;
            }
        }
        return Some(headers);
    }

    None
}
