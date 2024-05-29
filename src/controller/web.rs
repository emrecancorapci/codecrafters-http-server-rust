use crate::http::response::{ Response, StatusCode };

pub fn get() -> String {
    let index = std::fs::read_to_string("./public/index.html");

    if index.is_err() {
        return Response::from(&StatusCode::NotFound)
            .text(&index.err().unwrap().to_string())
            .debug()
            .to_string();
    }

    Response::from(&StatusCode::Ok).body(&index.unwrap(), "text/html").to_string()
}
