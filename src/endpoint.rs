use crate::request::{bad_request, send_content};

pub fn user_agent(http_request: Vec<String>) -> String {
    let user_agent_str = http_request.get(2);

    if user_agent_str.is_none() {
        println!("user_agent not found");
        return bad_request();
    }

    let user_agent = user_agent_str.unwrap().split(' ').nth(1);

    if user_agent.is_none() {
        println!("user_agent not found");
        return bad_request();
    }

    return send_content(user_agent.unwrap());
}

pub fn echo(http_request: Vec<String>) -> String {
    let echo_text_str = http_request.get(0); //.unwrap().split(' ').nth(2).unwrap();

    if echo_text_str.is_none() {
        return bad_request();
    }

    let echo_text: Option<&str> = echo_text_str.unwrap().split(' ').nth(2);

    if echo_text.is_none() {
        return bad_request();
    }

    return send_content(echo_text.unwrap());
}
