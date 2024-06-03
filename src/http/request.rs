use std::collections::HashMap;
use itertools::Itertools;
use super::extensions::HashMapExt;

pub struct HttpRequest<'a> {
    pub request: RequestLine<'a>,
    pub headers: HashMap<String, String>,
    pub body: &'a str,
}

pub struct RequestLine<'a> {
    pub method: &'a str,
    pub path: &'a str,
    pub version: &'a str,
    pub path_array: Vec<&'a str>,
}

impl<'a> HttpRequest<'a> {
    pub fn from(http_request: &'a Vec<String>, body: &'a str) -> HttpRequest<'a> {
        let request = RequestLine::from_string_option(http_request.first());

        if request.is_err() {
            panic!("RequestLine not found: {:?}", request.err().unwrap());
        }

        let headers = parse_headers(http_request);

        HttpRequest {
            request: request.unwrap(),
            headers,
            body,
        }
    }
    pub fn debug(&self) {
        println!(
            "HttpRequest Line: {}\r\n Headers: {}\r\n Body: {}",
            self.request.to_string(),
            self.headers.to_string(),
            self.body
        );
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
    pub fn from_string_option(
        request_option: Option<&String>
    ) -> Result<RequestLine, &'static str> {
        if request_option.is_none() {
            return Err("request_line not found");
        }

        let request_line = request_option.unwrap().split(' ').collect_vec();
        if request_line.len() < 3 {
            return Err("request_line does not have 3 parts");
        }

        Ok(RequestLine::from(&request_line[0], &request_line[1], &request_line[2]))
    }
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.method, self.path, self.version)
    }
}

fn parse_headers(http_request: &Vec<String>) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();
    for line in http_request.iter().skip(1) {
        let header = line.split(':').collect_vec();

        if header.len() < 2 {
            continue;
        }

        let key: String = header[0].trim().to_lowercase().to_string();
        let value: String = header[1].trim().to_string();

        headers.insert(key, value);
    }
    headers
}
