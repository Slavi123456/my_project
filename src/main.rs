use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    Body, Request, Response, Server,
    service::{make_service_fn, service_fn},
};
use tokio::fs::read_to_string;

#[tokio::main]
async fn main() {
    //Set up the addres for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING on http://{addr}");

    //Creating a service which will be our handler for requests
    let make_service =
        make_service_fn(|socket| async move { Ok::<_, Infallible>(service_fn(main_service)) });

    //Starting the server
    Server::bind(&addr).serve(make_service).await.unwrap();
}

async fn main_service(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Ok(Response::new(Body::from("Hello world!")))
    if request.uri().path() == "/home" {
        home_page().await
    } else {
        Ok(Response::new(Body::from("404 Not Found")))
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
