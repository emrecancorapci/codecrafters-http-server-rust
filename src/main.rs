#[allow(dead_code)]
use std::{ io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream }, thread };

mod routes;
pub mod http;
pub mod controller {
    pub mod echo;
    pub mod file;
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
    let buffer = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Buffer: {:?}", buffer);

    let http_request = &http::Request::from(&buffer);

    println!("HTTP request: {:?}", http_request);

    let response = routes::router(http_request);

    println!("Response: {response}");

    stream.write_all(response.as_bytes()).unwrap();
}
