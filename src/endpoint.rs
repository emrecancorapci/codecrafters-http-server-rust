use crate::request::{ bad_request, send_content };
use std::{ env, fs };

pub fn user_agent(http_request: Vec<String>) -> String {
    let user_agent_str = http_request.get(2);

    if user_agent_str.is_none() {
        return bad_request("user_agent_str not found");
    }

    let user_agent = user_agent_str.unwrap().split(' ').nth(1);

    if user_agent.is_none() {
        return bad_request("user_agent not found");
    }

    return send_content(user_agent.unwrap(), "text/plain");
}

pub fn echo(http_request: Vec<String>) -> String {
    let echo_text_str = http_request.get(0);

    if echo_text_str.is_none() {
        return bad_request("echo_text_str not found");
    }

    let echo_text: Option<&str> = echo_text_str.unwrap().split(' ').nth(2);

    if echo_text.is_none() {
        return bad_request("echo_text not found");
    }

    return send_content(echo_text.unwrap(), "text/plain");
}

pub fn files(http_request: Vec<String>) -> String {
    let file_name_str = http_request.get(0);
    let env_args: Vec<String> = env::args().collect();

    let mut directory = {
        if env_args.len() > 1 && env_args[1] == "--directory" {
            env_args[2].clone()
        } else {
            return bad_request("directory not found");
        }
    };

    println!("directory: {directory}");

    if file_name_str.is_none() {
        return bad_request("file_name_str not found");
    }

    let file_name = {
        let file_name = file_name_str.unwrap().split(' ').nth(2);

        if file_name.is_none() {
            return bad_request("--directory is not set");
        } else {
            file_name.unwrap()
        }
    };

    println!("file_name: {file_name}");

    directory.push_str(&file_name);

    let file = fs::read_to_string(directory);

    if file.is_err() {
        return bad_request("file not found");
    }

    return send_content(&file.unwrap(), "application/octet-stream");
}
