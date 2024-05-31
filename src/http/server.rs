use std::{io::Write, net::{TcpListener, TcpStream}};

use super::{parser::parse_stream, request::HttpRequest, response::{HttpResponse, StatusCode}, router::Router};

pub struct Server {
    router: Router,
    listener: TcpListener,
}

impl Server {
    pub fn create_server(port: u16) -> Server {
        let addr = format!("127.0.0.1:{port}");
        Server {
            router: Router::new(),
            listener: TcpListener::bind(addr).unwrap(),
        }
    }

    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(_stream) => {
                    Server::handle_connection(&self.router, _stream);
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }

    pub fn use_router(&mut self, router: Router) {
        self.router = router;
    }

    fn handle_connection(router: &Router, mut stream: TcpStream) {
        let parsing = parse_stream(&stream);
        let mut response = HttpResponse::new();

        match parsing {
            Ok((headers, body)) => {
                let request = &HttpRequest::from(&headers, &body);

                response.debug_on();

                router.route(request, &mut response);
            }
            Err(error) => {
                response.status(StatusCode::BadRequest).debug_msg(&error);
            }
        }

        stream.write_all(&response.as_bytes()[..]).unwrap();
    }
}
