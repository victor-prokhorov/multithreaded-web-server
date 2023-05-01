use http::{request::Request, response::Response};
use std::{env, fs};

pub trait Handler {
    fn handle(req: &Request) -> Response;
    fn load_file(filename: &str) -> Option<String> {
        // evaluated at compile-time
        let public_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        // evaluated at runtime
        let public_path = env::var("PUBLIC_PATH").unwrap_or(public_path);
        let filename = format!("{public_path}/{filename}");
        let contents = fs::read_to_string(filename);
        contents.ok()
    }
}
