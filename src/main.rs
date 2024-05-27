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

    // let method = http_request.get(0).unwrap().split(' ').nth(0).unwrap();

    let response = generate_reponse(http_request);
    stream.write_all(response.as_bytes()).unwrap();
}

fn generate_reponse(http_request: Vec<String>) -> String {
    let header = http_request.get(0);

    if header.is_none() {
        return request::bad_request();
    }

    let target_array = header.unwrap().split(' ').nth(1);

    if target_array.is_none() {
        return request::bad_request();
    }

    let target = target_array.unwrap().split('/').collect::<Vec<&str>>();

    println!("target_array: {:?}", target_array);

    match target[1] {
        "" => {
            return request::ok("");
        }
        "echo" => {
            return endpoint::echo(http_request);
        }
        "user-agent" => {
            return endpoint::user_agent(http_request);
        }
        _ => {
            return request::not_found();
        }
    }
}
