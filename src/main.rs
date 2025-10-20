use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    Body, Method, Request, Response, Server,
    service::{make_service_fn, service_fn},
};

use crate::{
    handlers::{
        login_out::{handle_delete_logout, handle_post_login},
        page::{handle_get_request, handle_get_root},
        profile::handle_put_profile,
        register::handle_post_register,
    },
    structs::{Pages, Routes, app_state::AppState},
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

    //Creating a service which will serve as a requests dispatcher
    let make_service = make_service_fn(move |_socket| {
        let app_state = app_state.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |request: Request<Body>| {
                request_dispatcher(request, app_state.clone())
            }))
        }
    });

    //Starting the server
    let server = Server::bind(&addr).serve(make_service).await;

    //Handle server error
    if let Err(err) = server {
        println!("->>Server couldn't start error {}", err);
    }
}

async fn request_dispatcher(
    request: Request<Body>,
    app_state: AppState,
) -> Result<Response<Body>, Infallible> {
    let req_method = request.method();
    let req_path = request.uri().path();

    match (req_method, req_path) {
        (&Method::GET, Routes::ROOT) => handle_get_root().await,
        (&Method::GET, Routes::HOME) => handle_get_request(Pages::HOME).await,

        (&Method::GET, Routes::LOGIN) => handle_get_request(Pages::LOGIN).await,
        (&Method::POST, Routes::LOGIN) => handle_post_login(request, app_state).await,

        (&Method::DELETE, Routes::LOGOUT) => handle_delete_logout(request, app_state).await,

        (&Method::GET, Routes::REGISTER) => handle_get_request(Pages::REGISTER).await,
        (&Method::POST, Routes::REGISTER) => handle_post_register(request, app_state).await,

        (&Method::GET, Routes::PROFILE) => handle_get_request(Pages::PROFILE).await,
        (&Method::PUT, Routes::PROFILE) => handle_put_profile(request, app_state).await,

        (&Method::GET, Routes::USER_PROFILE) => load_user_data(request, app_state).await,
        (&Method::GET, Routes::PAGE_CSS_FILE) => handle_static_file(Pages::CSS_FILE).await,

        //this should be changed to other error
        _ => Ok(Response::new(Body::from("404 Not Found"))),
    }
}
