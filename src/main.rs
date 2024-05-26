use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

  for stream in listener.incoming() {
    println!("connection accepted. Stream: {:?}", stream);

    match stream {
      Ok(_stream) => {
        println!("accepted new connection");
        handle_connection(_stream);
      }
      Err(e) => {
        println!("error: {}", e);
      }
    }
  }
}

fn handle_connection(mut stream: TcpStream) {
  let buf_reader = BufReader::new(&mut stream);
  let http_request: Vec<_> = buf_reader
    .lines()
    .map(|result| result.unwrap())
    .take_while(|line| !line.is_empty())
    .collect();
  println!("HTTP request: {:?}", http_request);

  let request_line = http_request.get(0).unwrap();
  let request_target = request_line.split(' ').nth(1).unwrap();

  let http_version = "HTTP/1.1";
  let status_code = if request_target == "/" { 200 } else { 404 };
  let status_message = if request_target == "/" { "OK" } else { "Not Found" };
  let response = format!("{http_version} {status_code} {status_message}\r\n\r\n");

  stream.write_all(response.as_bytes()).unwrap();
}
