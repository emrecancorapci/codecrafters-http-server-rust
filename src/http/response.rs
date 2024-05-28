// 200

pub fn ok(text: &str, content_type: &str) -> String {
    let content_header = {
        if text == "" {
            String::from("")
        } else if content_type == "" {
            format!("Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{text}", text.len())
        } else {
            format!("Content-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{text}", text.len())
        }
    };
    send_response("200 OK", &content_header)
}

// 400
pub fn bad_request(server_message: &str) -> String {
    println!("{server_message}");
    send_response("400 Bad Request", "")
}

pub fn not_found() -> String {
    send_response("404 Not Found", "")
}

pub fn method_not_allowed() -> String {
    send_response("405 Method Not Allowed", "")
}

fn send_response(request: &str, content: &str) -> String {
    let http_version = "HTTP/1.1";
    if content == "" {
        return format!("{http_version} {request}\r\n\r\n");
    }
    format!("{http_version} {request}\r\n{content}")
}
