use std::{convert::Infallible, fs::read, path::Path};

use hyper::{Body, Response};

use crate::utils::{response::response_ok_with_content, response_bad_request};

pub fn handle_static_file(path: &str) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_static_file");

    let file_path = Path::new("pages").join(path);

    match read(&file_path) {
        Ok(content) => {
            let header_type = if path.ends_with(".css") {
                "text/css"
            } else {
                "text/plain"
            };

            Ok(response_ok_with_content(content, header_type))
        }
        Err(_) => Ok(response_bad_request("Failed to load css")),
    }
}
