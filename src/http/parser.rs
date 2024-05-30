use std::{ io::{BufReader, Read, BufRead}, net::TcpStream };

pub fn parse_stream(mut stream: &TcpStream) -> (Vec<String>, String) {
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

    if content_length == 0 {
        return (headers, String::new());
    }

    let mut body = Vec::with_capacity(content_length);
    let result = buf_reader.take(content_length as u64).read_to_end(&mut body);

    if result.is_err() {
        println!("Error reading body: {:?}", result.err());
    }

    let body_str = String::from_utf8(body).unwrap();

    (headers, body_str)
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
