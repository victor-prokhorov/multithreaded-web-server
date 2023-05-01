use super::handler_trait::Handler;
use http::{
    request::{Request, Resource},
    response::Response,
};
use std::collections::HashMap;

pub struct StaticPageHandler;

impl Handler for StaticPageHandler {
    fn handle(req: &Request) -> Response {
        let Resource::Path(path) = &req.resource;
        let route: Vec<&str> = path.split('/').collect();
        match route[1] {
            "" => Response::new(200, None, Self::load_file("index.html")),
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    match path.split('.').last() {
                        Some("css") => map.insert("Content-Type", "text/css"),
                        Some("js") => map.insert("Content-Type", "text/javascript"),
                        Some("html") => map.insert("Content-Type", "text/javascript"),
                        _ => return Response::new(404, None, Self::load_file("404.html")),
                    };
                    Response::new(200, Some(map), Some(contents))
                }
                None => Response::new(404, None, Self::load_file("404.html")),
            },
        }
    }
}
