pub fn send_content(text: &str, content_type: &str) -> String {
    let content = format!("Content-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{text}", text.len());
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

pub fn method_not_allowed() -> String {
    send_request("405 Method Not Allowed", "")
}

fn send_request(request: &str, content: &str) -> String {
    let http_version = "HTTP/1.1";
    if content == "" {
        return format!("{http_version} {request}\r\n\r\n")
    }
    format!("{http_version} {request}\r\n{content}")
}