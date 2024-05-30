#[allow(dead_code)]
use std::{ io::prelude::*, net::{ TcpListener, TcpStream }, thread };

use http::{ parser::parse_stream, request::HttpRequest, response::{ HttpResponse, StatusCode } };

mod routes;
pub mod http {
    pub mod request;
    pub mod response;
    pub mod parser;
}
pub mod helpers;
pub mod extensions {
    pub mod hash_map;
}
pub mod controller {
    pub mod echo;
    pub mod file;
    pub mod web;
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
    let request = parse_stream(&stream);

    match request {
        Ok((headers, body)) => {
            let http_request = &HttpRequest::from(&headers, &body);

            let response = routes::router(http_request);
            println!("HttpResponse:\r\n{response}");

            stream.write_all(response.as_bytes()).unwrap();
        }
        Err(error) => {
            stream
                .write_all(
                    HttpResponse::from(&StatusCode::BadRequest).text(&error).to_string().as_bytes()
                )
                .unwrap();
            return;
        }
    }
}
