mod handlers;
use crate::router::handlers::{Handler, StaticPageHandler};
use handlers::PageNotFoundHandler;
use handlers::WebServiceHandler;
use http::request::{Method, Request, Resource};
use std::io::prelude::*;
use tracing::{instrument, trace};

pub struct Router;

impl Router {
    #[instrument(skip(req, stream))]
    pub fn route(req: Request, stream: &mut impl Write) {
        trace!(method = ?req.method, path = ?req.resource);
        match req.method {
            Method::Get => match &req.resource {
                Resource::Path(path) => {
                    let route: Vec<&str> = path.split('/').collect();
                    match route[1] {
                        "api" => WebServiceHandler::handle(&req)
                            .send_response(stream)
                            .unwrap(),

                        _ => StaticPageHandler::handle(&req)
                            .send_response(stream)
                            .unwrap(),
                    }
                }
            },
            _ => PageNotFoundHandler::handle(&req)
                .send_response(stream)
                .unwrap(),
        }
    }
}
