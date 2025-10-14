use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    Body, HeaderMap, Method, Request, Response, Server, StatusCode,
    body::to_bytes,
    header::{CONTENT_TYPE, COOKIE, HeaderValue, LOCATION, SET_COOKIE},
    service::{make_service_fn, service_fn},
};
use tokio::fs::read_to_string;

use crate::{
    structs::{
        app_state::AppState,
        login::LoginInfo,
        traits::Extractable,
        user::{User, UserProfile},
    },
    utils::{bad_request, extract_from_request, handle_static_file, load_user_data},
};

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
        // (&Method::PUT, req_path) if req_path.starts_with("/home/") => {
        //     home_page_put(request, users_list).await
        // }
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

async fn page_get(page: &str) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - get_page - {}", page);

    let page = match read_to_string(format!("./pages/{}", page)).await {
        Ok(content) => content,
        Err(err) => {
            //Handle the error
            println!("->> Error in home_page {}", err);

            "<html><body>base</body></html>".to_string()
        }
    };

    Ok(Response::new(Body::from(page)))
}

async fn main_page_get() -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - main_page_get");

    let respone = Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/login")
        .body(Body::empty())
        .unwrap();
    Ok(respone)
}

async fn logout_page_delete(
    request: Request<Body>,
    mut users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - logout_page_post");

    let (parts, _body) = request.into_parts();

    //Checking for already existing session
    let session_id = match extract_session_id_from_header(&parts.headers) {
        Ok(id) => id,
        Err(error) => return Ok(error),
    };

    users_list.delete_session(&session_id).await;
    users_list.print_sessions().await;

    //Transfer to the login page with expired cookie
    let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");

    let resp = Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/login")
        .header(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
        .body(Body::from("Successfully logged out"))
        .unwrap();

    Ok(resp)
}

async fn login_page_post(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - login_page_post");

    let (parts, body) = request.into_parts();

    //Checking for already existing session
    if let Ok(session_id) = extract_session_id_from_header(&parts.headers) {
        println!("->> Session ID found: {}", session_id);

        //Validate the session if not return to the login page
        if !users_list.is_session_valid(&session_id).await {
            let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");
            let response = Response::builder()
                .status(StatusCode::FOUND)
                .header(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
                .header(LOCATION, "/login")
                .body(Body::from("Invalid session"))
                .unwrap();

            return Ok(response);
        }

        //Transfer the user to he home page
        let response = Response::builder()
            .status(StatusCode::FOUND)
            .header(SET_COOKIE, HeaderValue::from_str(&session_id).unwrap())
            .header(LOCATION, "/home")
            .body(Body::from("Already logged in"))
            .unwrap();

        users_list.print_sessions().await;

        return Ok(response);
    } else {
        //Some kind of an error
    }

    //Extracting loginInfo
    let login: LoginInfo = match extract_from_request(body).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Create a session for succesful login
    let cookie = match users_list.find_user(login).await {
        Ok(user_id) => match users_list.add_session(user_id).await {
            Ok(session) => {
                let session_id = session.session_id();
                users_list.print_sessions().await;
                &format!("session_id={}; HttpOnly; Path=/", session_id)
            }
            Err(err_msg) => return Ok(bad_request(&err_msg)),
        },
        Err(err_msg) => return Ok(bad_request(&err_msg)),
    };

    //Create response with the cookie and the redirecting to the home page
    let response = Response::builder()
        .status(StatusCode::FOUND)
        .header(SET_COOKIE, HeaderValue::from_str(cookie).unwrap())
        .header(LOCATION, "/home")
        .body(Body::from("Successfully logged in"))
        .unwrap();

    Ok(response)
    // Ok(Response::new(Body::from("Successfully logged in")))
}

fn extract_session_id_from_header(header: &HeaderMap) -> Result<String, Response<Body>> {
    if let Some(cookie_header) = header.get(COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Extract session_id from the cookie string
            if let Some(session_id) = extract_session_id_from_cookie(cookie_str) {
                println!("->> Session ID found: {}", session_id);

                Ok(session_id)
            } else {
                return Err(bad_request("No session ID in cookie"));
            }
        } else {
            return Err(bad_request("Invalid cookie header"));
        }
    } else {
        return Err(bad_request("No cookie found"));
    }
}
fn extract_session_id_from_cookie(cookie_str: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        let trimmed = part.trim();
        if let Some(session_id) = trimmed.strip_prefix("session_id=") {
            return Some(session_id.to_string());
        }
    }
    None
}

async fn profile_page_put(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - home_page_put");

    let (parts, body) = request.into_parts();
    // let path_segments = path.uri.path().split("/").collect::<Vec<&str>>();
    //println!("{:?}", path_segments);

    //Checking for existing session
    let session_id = match extract_session_id_from_header(&parts.headers) {
        Ok(id) => id,
        Err(error) => return Ok(error),
    };
    //Validate the session if not return to the login page
    if !users_list.is_session_valid(&session_id).await {
        let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");
        let response = Response::builder()
            .status(StatusCode::FOUND)
            .header(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
            .header(LOCATION, "/login")
            .body(Body::from("Invalid session"))
            .unwrap();

        return Ok(response);
    }

    //Get the user id from the session
    let user_id = match users_list.get_user_id_from_session(&session_id).await {
        Ok(id) => id,
        Err(error) => return Ok(bad_request(&error)),
    };

    //Reading the request body
    let user: User = match extract_from_request(body).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Validation
    if let Err(err_msg) = user.validate() {
        return Ok(bad_request(&err_msg));
    }

    //Updating the user
    if let Err(err_msg) = users_list.update_user(user, user_id).await {
        return Ok(bad_request(&err_msg));
    }

    users_list.print_users().await;

    //transfer to the home page
    let respone = Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/login")
        .body(Body::from("Succesful updated user"))
        .unwrap();

    Ok(respone)
    // Ok(Response::new(Body::from("Succesful updated user")))
}

async fn register_page_post(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - home_page_post");

    let user: User = match extract_from_request(request.into_body()).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Validation
    if let Err(err_msg) = user.validate() {
        return Ok(bad_request(&err_msg));
    }
    //Saving the user information
    if let Err(err) = users_list.add_user(user).await {
        return Ok(bad_request(&format!("{}", err)));
    }
    users_list.print_users().await;

    // Ok(Response::new(Body::from("Successfully registered")))

    //Transfer to the login page
    let respone = Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/login")
        .body(Body::empty())
        .unwrap();
    Ok(respone)
}
