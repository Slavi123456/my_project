use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    Body, Method, Request, Response, Server,
    service::{make_service_fn, service_fn},
};

use crate::{
    handlers::{
        login_out::{login_page_post, logout_page_delete},
        page::{main_page_get, page_get},
        profile::profile_page_put,
        register::register_page_post,
    },
    structs::app_state::AppState,
    utils::{handle_static_file, load_user_data},
};

mod handlers;
mod structs;
mod utils;

#[tokio::main]
async fn main() {
    // Hardcoded connection string
    let _db_url = "mysql://root:rootpassword@localhost:3306/mydb";

    //There is also AppState::new_without_db() for trying withot the database saving
    let app_state = match AppState::new(_db_url).await {
        Ok(app_state) => app_state,
        Err(error) => {
            println!("->> Error building the AppState error {}", error);
            return;
        }
    };

    //Set up the addres for the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("->> LISTENING on http://{addr}");

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
        (&Method::GET, "/") => main_page_get().await,
        (&Method::GET, "/home") => page_get("home.html").await,

        (&Method::GET, "/login") => page_get("login.html").await,
        (&Method::POST, "/login") => login_page_post(request, users_list).await,

        (&Method::DELETE, "/logout") => logout_page_delete(request, users_list).await,

        (&Method::GET, "/register") => page_get("register.html").await,
        (&Method::POST, "/register") => register_page_post(request, users_list).await,

        (&Method::GET, "/profile") => page_get("profile.html").await,
        (&Method::PUT, "/profile") => profile_page_put(request, users_list).await,

        (&Method::GET, "/profile/user") => load_user_data(request, users_list).await,
        (&Method::GET, "/loginPageStyle.css") => handle_static_file("loginPageStyle.css").await,

        _ => Ok(Response::new(Body::from("404 Not Found"))),
    }
}
