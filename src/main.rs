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

  let response = generate_reponse(request_target);
  stream.write_all(response.as_bytes()).unwrap();
}

fn generate_reponse(target: &str) -> String {
  let http_version = "HTTP/1.1";

  let target_array = target.split('/').collect::<Vec<&str>>();
  println!("target_array: {:?}", target_array);

  match target_array[1] {
    "" => {
      format!("{http_version} 200 OK\r\n\r\n")
    }
    "echo" => {
      let echo_text = target_array[2];
      let echo_text_len = echo_text.len();
      let response_string = format!("Content-Type: text/plain\r\nContent-Length: {echo_text_len}\r\n\r\n{echo_text}");
      format!("{http_version} 200 OK\r\n{response_string}")
    }
    _ => {
      format!("{http_version} 404 Not Found\r\n\r\n")
    }
  }
}
