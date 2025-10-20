use std::{convert::Infallible, fs::read, path::PathBuf};

use hyper::{Body, Response, StatusCode};

use crate::utils::response_bad_request;

pub async fn handle_static_file(path: &str) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_static_file");

    let mut file_path = PathBuf::from("./pages");
    file_path.push(path);

    match read(&file_path) {
        Ok(content) => {
            let mime_type = if path.ends_with(".css") {
                "text/css"
            } else {
                "text/plain"
            };

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", mime_type)
                .body(Body::from(content))
                .unwrap())
        }
        Err(_) => Ok(response_bad_request("Failed to load css")),
    }
}
