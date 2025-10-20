use std::{convert::Infallible, fs::read_to_string};

use hyper::{Body, Response, StatusCode, header};

use crate::structs::Routes;

pub async fn handle_get_root() -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_get_root");

    //Should check for session_id because it login two times and have two sessions
    //Should transfer to the home page if there is session

    //Transfer to the login page
    let respone = Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, Routes::LOGIN)
        .body(Body::empty())
        .unwrap();
    Ok(respone)
}

pub async fn handle_get_request(page: &str) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_get_request - {}", page);

    let page = match read_to_string(format!("./pages/{}", page)) {
        Ok(content) => content,
        Err(err) => {
            //Handle the error
            println!("->> Error in home_page {}", err);

            //I should have something like basic html for error
            "<html><body>base</body></html>".to_string()
        }
    };

    //Should check for session id, becuase it's not appropriate to access some pages without session

    Ok(Response::new(Body::from(page)))
}
