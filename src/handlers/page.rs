use std::{convert::Infallible, fs::read_to_string};

use hyper::{Body, Response, StatusCode, header::LOCATION};

pub async fn page_get(page: &str) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - get_page - {}", page);

    let page = match read_to_string(format!("./pages/{}", page)) {
        Ok(content) => content,
        Err(err) => {
            //Handle the error
            println!("->> Error in home_page {}", err);

            "<html><body>base</body></html>".to_string()
        }
    };

    Ok(Response::new(Body::from(page)))
}

pub async fn main_page_get() -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - main_page_get");

    let respone = Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/login")
        .body(Body::empty())
        .unwrap();
    Ok(respone)
}
