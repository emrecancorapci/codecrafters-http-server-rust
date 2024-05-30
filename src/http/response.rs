use std::collections::HashMap;

use crate::extensions::hash_map::HashMapExt;

pub enum StatusCode {
    Ok,
    Created,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    IAmATeapot,
    InternalServerError,
}

impl StatusCode {
    pub fn to_string(&self) -> &str {
        match self {
            StatusCode::Ok => "200",
            StatusCode::Created => "201",
            StatusCode::BadRequest => "400",
            StatusCode::NotFound => "404",
            StatusCode::MethodNotAllowed => "405",
            StatusCode::IAmATeapot => "418",
            StatusCode::InternalServerError => "500",
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::IAmATeapot => "I'm a teapot",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

pub struct Response<'a> {
    http_version: &'a str,
    status_code: &'a StatusCode,
    headers: HashMap<String, String>,
    body: &'a str,
}

impl Response<'_> {
    pub fn from<'a>(status_code: &'a StatusCode) -> Response<'a> {
        Response {
            http_version: "HTTP/1.1",
            status_code,
            headers: HashMap::new(),
            body: "",
        }
    }

    pub fn body<'a>(&'a self, body: &'a str, content_type: &'a str) -> Response {
        let mut headers = self.headers.clone();
        let length = body.len().to_string();

        headers.insert(String::from("Content-Length"), length);
        headers.insert(String::from("Content-Type"), content_type.to_string());

        Response {
            http_version: self.http_version,
            status_code: self.status_code,
            headers: headers.clone(),
            body,
        }
    }

    pub fn text<'a>(&'a self, body: &'a str) -> Response {
        self.body(body, "text/plain")
    }

    pub fn headers<'a>(&'a self, _headers: HashMap<String, String>) -> Response {
        let mut headers = self.headers.clone();
        headers.extend(_headers);

        Response {
            http_version: self.http_version,
            status_code: self.status_code,
            headers,
            body: self.body,
        }
    }

    pub fn to_string(&self) -> String {
        let mut headers = String::new();

        if self.headers.len() > 0 {
            for (key, value) in &self.headers {
                headers.push_str(&format!("{key}: {value}\r\n"));
            }
        }

        if self.body != "" {
            if !headers.contains("Content-Length") {
                println!("Content-Length not found even though body is present");
                headers.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
            }
            if !headers.contains("Content-Type") {
                println!("Content-Type not found even though body is present");
                headers.push_str("Content-Type: text/plain\r\n");
            }
        }

        format!(
            "{http_version} {status_code} {status_msg}\r\n{headers}\r\n{body}",
            http_version = self.http_version,
            status_code = self.status_code.to_string(),
            status_msg = self.status_code.get_message(),
            body = self.body
        )
    }

    pub fn debug(&self) -> &Response {
        println!("HTTP version: {}", self.http_version);
        println!("Status code: {}", self.status_code.to_string());
        println!("Status message: {}", self.status_code.get_message());
        println!("Headers: {}", &self.headers.to_string());
        println!("Body: {}", self.body);

        self
    }
}
