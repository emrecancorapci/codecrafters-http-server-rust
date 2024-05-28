#[allow(dead_code)]

use std::{ io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream }, thread };
use itertools::Itertools;

pub mod endpoint;
pub mod request;

#[derive(Debug)]
pub struct HttpRequest<'a>{
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

    let response = router(http_request);

    println!("Response: {response}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn router(http_request: &HttpRequest) -> String {
    match http_request.path_array[0] {
        "" => {
            return request::ok("");
        }
        "echo" => {
            return endpoint::echo(http_request);
        }
        "user-agent" => {
            return request::send_content(http_request.user_agent, "text/plain");
        }
        "files" => {
            return endpoint::files(http_request);
        }
        _ => {
            return request::not_found();
        }
    }
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

    println!("request_line: {:?}", request_line);

    HttpRequest {
        method: request_line[0],
        path: request_line[1],
        http_version: "HTTP/1.1", // "HTTP/1.1" is hardcoded for now
        user_agent: http_request.get(2).unwrap().split(' ').nth(1).unwrap(),
        host: http_request.get(1).unwrap().split(' ').nth(1).unwrap(),
        path_array: request_line[1][1..].split('/').collect_vec(),
    }
}