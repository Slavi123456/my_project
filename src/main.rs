use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    Body, Method, Request, Response, Server, StatusCode,
    body::to_bytes,
    service::{make_service_fn, service_fn},
};
use tokio::fs::read_to_string;

use crate::user::{AppState, User};

mod user;

#[tokio::main]
async fn main() {
    //Set up the addres for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING on http://{addr}");

    let app_state = AppState::new().await;

    //Creating a service which will be our handler for requests
    let make_service = make_service_fn(move |_socket| {
        let app_state = app_state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |request: Request<Body>| {
                main_service(request, app_state.clone())
            }))
        }
    });

    //Starting the server
    Server::bind(&addr).serve(make_service).await.unwrap();
}

async fn main_service(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    let req_method = request.method();
    let req_path = request.uri().path();

    match (req_method, req_path) {
        (&Method::GET, "/home") => home_page().await,
        (&Method::POST, "/home") => home_page_post(request, users_list).await,
        _ => Ok(Response::new(Body::from("404 Not Found"))),
    }
}

async fn home_page() -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - home_page");
    let home_page = match read_to_string("./pages/home.html").await {
        Ok(content) => content,
        Err(err) => {
            //Handle the error
            println!("->> Error in home_page {}", err);

            "<html><body>base</body></html>".to_string()
        }
    };

    Ok(Response::new(Body::from(home_page)))
}

async fn home_page_post(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - home_page_post");

    let user = match extract_user_from_request(request).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Validation
    if let Err(err_msg) = user.validate() {
        return Ok(bad_request(&err_msg));
    }
    //Saving the user information
    users_list.add_user(user).await;
    users_list.print_users().await;

    Ok(Response::new(Body::from(
        "Successfully parsed post request",
    )))
}

async fn extract_user_from_request(request: Request<Body>) -> Result<User, Response<Body>> {
    let req_body = to_bytes(request.into_body()).await.map_err(|err| {
        //handle error
        println!("->> Error in parsing request body {}", err);

        bad_request("Could not parse request body")
    })?;

    serde_json::from_slice(&req_body).map_err(|err| {
        //handle error
        println!("->> Error in parsing json {}", err);
        bad_request("Could not parse request body")
    })
}

fn bad_request(msg: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "text/plain")
        .body(Body::from(msg.to_string()))
        .unwrap()
}
