use std::collections::HashMap;
use itertools::Itertools;
use std::io::prelude::*;
use flate2::{ write::GzEncoder, Compression };

use crate::http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

pub fn get(http_request: &HttpRequest) -> String {
    let echo_text = http_request.request.path_array[1];

    if echo_text.is_empty() {
        return HttpResponse::from(&StatusCode::BadRequest)
            .text("echo_text is empty")
            .debug()
            .to_string();
    }

    match get_encoding(http_request) {
        Some(encoding_header) => {
            match encode(echo_text) {
                Ok(compressed_string) => {
                    return HttpResponse::from(&StatusCode::Ok)
                        .text(&compressed_string)
                        .headers(encoding_header)
                        .to_string();
                }

                Err(e) => {
                    return HttpResponse::from(&StatusCode::InternalServerError)
                        .text(&e)
                        .debug()
                        .to_string();
                }
            }
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

fn encode(echo_text: &str) -> Result<String, String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    match encoder.write_all(echo_text.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    }

    match encoder.finish() {
        Ok(compressed_bytes) => {
            match String::from_utf8(compressed_bytes) {
                Ok(compressed_string) => {
                    return Ok(compressed_string);
                }
                Err(e) => {
                    return Err(format!("Error: {}", e));
                }
            }
        }
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    }
}
