use std::collections::HashMap;
use itertools::Itertools;
use std::io::prelude::*;
use flate2::{ write::GzEncoder, Compression };

use crate::http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

pub fn get(request: &HttpRequest, response: &mut HttpResponse) {
    let echo_text = request.request.path_array[1];

    if echo_text.is_empty() {
        response.status(StatusCode::BadRequest).debug_msg("echo_text is empty");
        return;
    }

    match get_encoding(request) {
        Some(encoding_header) => {
            match encode(echo_text) {
                Ok(compressed_string) => {
                    response
                        .status(StatusCode::Ok)
                        .body(compressed_string, "text/plain")
                        .headers(encoding_header);
                }

                Err(e) => {
                    response.status(StatusCode::InternalServerError).debug_msg(&e);
                }
            }
        }
        None => {
            response.status(StatusCode::Ok).body(echo_text.as_bytes().to_vec(), "text/plain");
        }
    }
}

fn get_encoding(request: &HttpRequest) -> Option<HashMap<String, String>> {
    let mut headers: HashMap<String, String> = HashMap::new();
    let accept_encoding = request.headers.get("accept-encoding");

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

fn encode(echo_text: &str) -> Result<Vec<u8>, String> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    match encoder.write_all(echo_text.as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    }

    match encoder.finish() {
        Ok(compressed_bytes) => {
            return Ok(compressed_bytes);
        }
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    }
}
