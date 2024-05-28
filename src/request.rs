pub fn send_content(text: &str, content_type: &str) -> String {
    let text_len = text.len();
    let content = format!("Content-Type: {content_type}\r\nContent-Length: {text_len}\r\n\r\n{text}");
    ok(&content)
}

pub fn ok(content: &str) -> String {
    send_request("200 OK", content)
}

pub fn bad_request(server_message: &str) -> String {
    println!("{server_message}");
    send_request("400 Bad Request", "")
}

pub fn not_found() -> String {
    send_request("404 Not Found", "")
}

fn send_request(request: &str, content: &str) -> String {
    let http_version = "HTTP/1.1";
    format!("{http_version} {request}\r\n\r\n{content}")
}
