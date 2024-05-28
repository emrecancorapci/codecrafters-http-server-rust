#[allow(dead_code)]
use std::{ io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream }, thread };
use itertools::Itertools;

mod routes;
pub mod controller {
    pub mod echo;
    pub mod file;
}
pub mod request;

#[derive(Debug)]
pub struct HttpRequest<'a> {
    method: &'a str,
    path: &'a str,
    http_version: &'a str,
    host: &'a str,
    user_agent: &'a str,
    path_array: Vec<&'a str>,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                thread::spawn(move || handle_connection(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let raw_http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("HTTP request: {:?}", raw_http_request);

    let http_request = &format_request(&raw_http_request);

    println!("Formatted HTTP request: {:?}", http_request);

    let response = routes::router(http_request);

    println!("Response: {response}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn format_request(http_request: &Vec<String>) -> HttpRequest {
    let request_line = {
        let request_line = http_request.get(0);

        if request_line.is_none() {
            panic!("request_line not found")
        } else {
            request_line.unwrap().split(' ').collect_vec()
        }
    };

    let (method, path, http_version) = {
        if request_line.len() < 3 {
            panic!("request_line does not have 3 parts")
        } else {
            (request_line[0], request_line[1], request_line[2])
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

    HttpRequest {
        method,
        path,
        http_version,
        user_agent,
        host,
        path_array: request_line[1][1..].split('/').collect_vec(),
    }
}
