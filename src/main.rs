use std::{ io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream }, thread };

pub mod endpoint;
pub mod request;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        println!("connection accepted. Stream: {:?}", stream);

        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("HTTP request: {:?}", http_request);

    let response = generate_reponse(&http_request);
    println!("Response: {response}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn generate_reponse(http_request: &Vec<String>) -> String {
    let request_line = {
        let request_line = http_request.get(0);

        if request_line.is_none() {
            return request::bad_request();
        } else {
            request_line.unwrap().split(' ').collect::<Vec<&str>>()
        }
    };

    let path = request_line[1][1..].split('/').collect::<Vec<&str>>();

    router(http_request.clone(), path[0])
}

fn router(http_request: Vec<String>, path: &str, ) -> String {
    match path {
        "" => {
            return request::ok("");
        }
        "echo" => {
            return endpoint::echo(http_request);
        }
        "user-agent" => {
            return endpoint::user_agent(http_request);
        }
        "files" => {
            return endpoint::files(http_request);
        }
        _ => {
            return request::not_found();
        }
    }
}
