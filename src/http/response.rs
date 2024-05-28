use std::collections::HashMap;

trait HashMapExt {
    fn to_string(&self) -> String;
}

impl HashMapExt for HashMap<&str, &str> {
    fn to_string(&self) -> String {
        let mut result = String::new();

        for (key, value) in self {
            result.push_str(&format!("{key}: {value}\r\n"));
        }

        result
    }
}

enum StatusCode {
    Ok,
    Created,
    BadRequest,
    NotFound,
    MethodNotAllowed,
    IAmATeapot,
    InternalServerError,
}

impl StatusCode {
    fn to_string(&self) -> &str {
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

    fn get_message(&self) -> &str {
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

struct Response<'a> {
    http_version: &'a str,
    status_code: &'a StatusCode,
    headers: HashMap<&'a str, &'a str>,
    body: &'a str,
}

impl Response<'_> {
    fn from<'a>(status_code: &'a StatusCode) -> Response<'a> {
        Response {
            http_version: "HTTP/1.1",
            status_code,
            headers: HashMap::new(),
            body: "",
        }
    }

    fn body<'a>(&'a self, body: &'a str, content_type: &'a str) -> Response {
        let mut headers = self.headers.clone();
        let length = body.len().to_string();

        headers.insert("Content-Length", &length);
        headers.insert("Content-Type", content_type);

        Response {
            http_version: self.http_version,
            status_code: self.status_code,
            headers: self.headers.clone(),
            body,
        }
    }

    fn text<'a>(&'a self, body: &'a str) -> Response {
        self.body(body, "text/plain")
    }

    fn headers<'a>(&'a self, _headers: HashMap<&'a str, &'a str>) -> Response {
        let mut headers = self.headers.clone();
        headers.extend(_headers);

        Response {
            http_version: self.http_version,
            status_code: self.status_code,
            headers,
            body: self.body,
        }
    }

    fn to_string(&self) -> String {
        let mut headers = String::new();

        if self.headers.len() > 0 {
            for (key, value) in &self.headers {
                headers.push_str(&format!("{key}: {value}\r\n"));
            }
        }

        if self.body != "" {
            headers.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
            if !headers.contains("Content-Type") {
                headers.push_str("Content-Type: text/plain\r\n");
            }
        }

        format!(
            "{} {} {}\r\n{headers}\r\n{}",
            self.http_version,
            self.status_code.to_string(),
            self.status_code.get_message(),
            self.body
        )
    }

    fn debug(&self) -> &Response {
        println!("HTTP version: {}", self.http_version);
        println!("Status code: {}", self.status_code.to_string());
        println!("Status message: {}", self.status_code.get_message());
        println!("Headers: {}", &self.headers.to_string());
        println!("Body: {}", self.body);

        self
    }
}

pub fn ok() -> String {
    Response::from(&StatusCode::Ok).to_string()
}

pub fn ok_text(text: &str) -> String {
    Response::from(&StatusCode::Ok).text(text).to_string()
}

pub fn ok_body(body: &str, content_type: &str) -> String {
    Response::from(&StatusCode::Ok).body(body, content_type).to_string()
}

pub fn ok_headers(headers: HashMap<&str, &str>) -> String {
    Response::from(&StatusCode::Ok).headers(headers).to_string()
}

pub fn ok_body_headers(body: &str, content_type: &str, headers: HashMap<&str, &str>) -> String {
    Response::from(&StatusCode::Ok)
        .body(body, content_type)
        .headers(headers)
        .to_string()
}

pub fn created() -> String {
    Response::from(&StatusCode::Created).to_string()
}

// 400
pub fn bad_request(server_message: &str) -> String {
    Response::from(&StatusCode::BadRequest).text(server_message).debug().to_string()
}

pub fn not_found() -> String {
    Response::from(&StatusCode::NotFound).debug().to_string()
}

pub fn method_not_allowed() -> String {
    Response::from(&StatusCode::MethodNotAllowed).debug().to_string()
}

// 500

pub fn internal_server_error(err: &str) -> String {
    Response::from(&StatusCode::InternalServerError).text(err).debug().to_string()
}
