use crate::{ controller::{ default, echo, file, user_agent, web }, http::router::Router };

pub fn router() -> Router {
    let mut router = Router::new();

    router.get("", &default::get);
    router.get("user-agent", &user_agent::get);
    router.get("echo", &echo::get);
    router.get("files", &file::get);
    router.post("files", &file::post);
    router.get("index.html", &web::get);

    router
}
