mod handler_trait;
pub use handler_trait::Handler;
mod page_not_found_handler;
pub use page_not_found_handler::PageNotFoundHandler;
mod static_page_handler;
pub use static_page_handler::StaticPageHandler;
mod web_service_handler;
pub use web_service_handler::WebServiceHandler;
