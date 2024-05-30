use crate::http::response::{ HttpResponse, StatusCode };

pub fn get() -> String {
    let index = std::fs::read_to_string("./public/index.html");

    if index.is_err() {
        return HttpResponse::from(&StatusCode::NotFound)
            .text(&index.err().unwrap().to_string())
            .debug()
            .to_string();
    }

    HttpResponse::from(&StatusCode::Ok).body(&index.unwrap(), "text/html").to_string()
}
