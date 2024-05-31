use std::collections::HashMap;

use crate::http::{ request::HttpRequest, response::{ HttpResponse, StatusCode } };

type IController = &'static dyn Fn(&HttpRequest, &mut HttpResponse);

#[derive(Eq, Hash, PartialEq)]
pub struct Route {
    path: String,
    method: String,
}
pub struct Router {
    endpoints: HashMap<Route, IController>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            endpoints: HashMap::new(),
        }
    }
    pub fn get(&mut self, path: &str, controller: IController) {
        self.endpoints.insert(
            Route {
                path: path.to_string(),
                method: "GET".to_string(),
            },
            controller
        );
    }

    pub fn post(&mut self, path: &str, controller: IController) {
        self.endpoints.insert(
            Route {
                path: path.to_string(),
                method: "POST".to_string(),
            },
            controller
        );
    }

    pub fn route(&self, request: &HttpRequest, response: &mut HttpResponse) {
        let path = &request.request.path_array;
        let method = &request.request.method;

        match
            self.endpoints.get(
                &(Route {
                    path: path[0].to_string(),
                    method: method.to_string(),
                })
            )
        {
            Some(controller) => controller(request, response),
            None => {
                response.status(StatusCode::NotFound).as_bytes();
            }
        }
    }
}
