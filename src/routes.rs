use crate::{ request, HttpRequest, controller };

mod echo;

pub fn router(http_request: &HttpRequest) -> String {
    match http_request.request.path_array[0] {
        "" => {
            return request::ok("");
        }
        "echo" => {
            echo::router(http_request)
        }
        "user-agent" => {
            return request::send_content(http_request.user_agent, "text/plain");
        }
        "files" => {

            return controller::file::get(http_request);
        }
        _ => {
            return request::not_found();
        }
    }
}
