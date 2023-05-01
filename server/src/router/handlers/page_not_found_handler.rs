use super::handler_trait::Handler;
use http::{request::Request, response::Response};

pub struct PageNotFoundHandler;

impl Handler for PageNotFoundHandler {
    fn handle(_: &Request) -> Response {
        Response::new(404, None, Self::load_file("404.html"))
    }
}
