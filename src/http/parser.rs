use std::{ io::{ BufRead, BufReader, Read }, net::TcpStream };

pub fn parse_stream(mut stream: &TcpStream) -> Result<(Vec<String>, String), String> {
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

    match content_length {
        Some(content_length) => {
            let mut body = Vec::with_capacity(content_length);
            let result = buf_reader.take(content_length as u64).read_to_end(&mut body);

            match result {
                Ok(_) => { Ok((headers, String::from_utf8(body).unwrap())) }
                Err(error) => { Err(format!("Error reading body: {}", error.to_string())) }
            }
        }
        None => { Ok((headers, String::from(""))) }
    }
}

fn get_content_length(headers: &Vec<String>) -> Option<usize> {
    let mut content_length = 0;

    for line in headers.iter() {
        if line.starts_with("Content-Length") {
            let parts = line.split(':').collect::<Vec<&str>>();
            content_length = parts[1].trim().parse::<usize>().unwrap();
            break;
        }
    }

    if content_length == 0 {
        return None;
    }

    Some(content_length)
}
