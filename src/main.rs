#[allow(dead_code)]
use std::{ io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream }, thread };

mod routes;
pub mod http;
pub mod extensions{
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
    let mut buf_reader = BufReader::new(&mut stream);
    let mut headers = Vec::new();

    for line in buf_reader.by_ref().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        headers.push(line);
    }

    let content_length = get_content_length(&headers);
    let mut body = Vec::with_capacity(content_length);

    let result = buf_reader.take(content_length as u64).read_to_end(&mut body);

    if result.is_err() {
        println!("Error reading body: {:?}", result.err());
    }

    let body_str = String::from_utf8(body).unwrap();

    let http_request = &http::Request::from(&headers, &body_str);

    println!("HTTP request: {:?}", http_request);

    let response = routes::router(http_request);

    println!("Response: {response}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_content_length(headers: &Vec<String>) -> usize {
    let mut content_length = 0;

    for line in headers.iter() {
        if line.starts_with("Content-Length") {
            let parts = line.split(':').collect::<Vec<&str>>();
            content_length = parts[1].trim().parse::<usize>().unwrap();
            break;
        }
    }

    content_length
}
