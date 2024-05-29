use std::collections::HashMap;

use itertools::Itertools;

pub mod response;

#[derive(Debug)]
pub struct Request<'a> {
    pub request: RequestLine<'a>,
    pub headers: HashMap<String, String>,
    pub body: &'a str,
}

#[derive(Debug)]
pub struct RequestLine<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
    pub path_array: Vec<&'a str>,
}

impl<'a> Request<'a> {
    pub fn from(http_request: &'a Vec<String>, body: &'a str) -> Request<'a> {
        let mut headers: HashMap<String, String> = HashMap::new();

        let request_line = {
            let request_line = http_request.get(0);

            if request_line.is_none() {
                panic!("request_line not found")
            } else {
                request_line.unwrap().split(' ').collect_vec()
            }
        };

        let request = {
            if request_line.len() < 3 {
                panic!("request_line does not have 3 parts")
            } else {
                RequestLine::from(&request_line[0], &request_line[1], &request_line[2])
            }
        };

        for line in http_request.iter().skip(1) {
            let header = line.split(':').collect_vec();

            if header.len() < 2 {
                continue;
            }

            let key: String = header[0].trim().to_lowercase().to_string();

            headers.insert(key, header[1].trim().to_string());
        }

        // Check if delete is needed here
        if !headers.contains_key("User-Agent") {
            headers.insert("User-Agent".to_string(), "".to_string());
        }

        Request {
            request,
            headers,
            body,
        }
    }
}

impl<'a> RequestLine<'a> {
    pub fn is_empty(&self) -> bool {
        self.method.is_empty() && self.path.is_empty() && self.version.is_empty()
    }
    pub fn from(method: &'a str, path: &'a str, version: &'a str) -> RequestLine<'a> {
        RequestLine {
            method,
            path,
            version,
            path_array: path[1..].split('/').collect_vec(),
        }
    }
}
