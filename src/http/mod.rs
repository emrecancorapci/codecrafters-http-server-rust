use itertools::Itertools;

pub mod response;

#[derive(Debug)]
pub struct Request<'a> {
    pub request: RequestLine<'a>,
    pub host: &'a str,
    pub user_agent: &'a str,
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
    pub fn from(http_request: &Vec<String>) -> Request {
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

        let user_agent = {
            let user_agent = http_request.iter().find(|line| line.starts_with("User-Agent: "));

            if user_agent.is_none() {
                ""
            } else {
                &user_agent.unwrap()[12..].trim()
            }
        };

        let host = {
            let host = http_request.iter().find(|line| line.starts_with("Host: "));

            if host.is_none() {
                ""
            } else {
                &host.unwrap()[6..].trim()
            }
        };

        // let body = {
        //     let body = http_request;

        // };

        Request {
            request,
            user_agent,
            host,
            body: "",
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
