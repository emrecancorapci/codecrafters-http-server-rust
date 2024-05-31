use http::server::Server;
use router::router;

pub mod http {
    pub mod server;
    pub mod router;
    pub mod parser;
    pub mod request;
    pub mod response;
    pub mod extensions;
}

pub mod router;
pub mod helpers;
pub mod controller {
    pub mod default;
    pub mod echo;
    pub mod file;
    pub mod teapot;
    pub mod user_agent;
    pub mod web;
}

fn main() {
    let mut server = Server::create_server(4221);
    let router = router();

    server.use_router(router);
    server.listen()
}
