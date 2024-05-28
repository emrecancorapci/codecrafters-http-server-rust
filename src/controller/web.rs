use crate::http::response;

pub fn get() -> String {
    let index = std::fs::read_to_string("./public/index.html");

    if index.is_err() {
        return response::not_found();
    }

    response::ok_text(&index.unwrap())
}
