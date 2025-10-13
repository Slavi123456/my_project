use std::{convert::Infallible, net::SocketAddr};

use hyper::{
    Body, Method, Request, Response, Server, StatusCode,
    body::to_bytes,
    header::{COOKIE, HeaderValue, SET_COOKIE},
    service::{make_service_fn, service_fn},
};
use tokio::fs::read_to_string;

use crate::user::{AppState, Extractable, LoginInfo, User};

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
        (&Method::GET, "/home") => home_page_get().await,
        (&Method::POST, "/home") => home_page_post(request, users_list).await,
        (&Method::PUT, req_path) if req_path.starts_with("/home/") => {
            home_page_put(request, users_list).await
        }
        (&Method::POST, "/login") => login_page_post(request, users_list).await,
        (&Method::POST, "/logout") => logout_page_post(request, users_list).await,
        _ => Ok(Response::new(Body::from("404 Not Found"))),
    }
}
async fn logout_page_post(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - login_page_post");

    //parse the request 
    //get the session_id
    //delete it if valid

    return Ok(Response::new(Body::from("Successfuly logged out")));
}
async fn login_page_post(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - login_page_post");

    let (parts, body) = request.into_parts();

    //Checking for already existing session
    let header = parts.headers.get(COOKIE);

    if let Some(cookie_header) = header {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Extract session_id from the cookie string
            if let Some(session_id) = extract_session_id(cookie_str) {
                println!("->> Session ID found: {}", session_id);

                //validate the session_id

                // return Ok(Response::new(Body::from("Already logged in")));
                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(SET_COOKIE, HeaderValue::from_str(&session_id).unwrap())
                    .body(Body::from("Already logged in"))
                    .unwrap();

                users_list.print_sessions().await;

                return Ok(response);
            } else {
                //return Ok(bad_request("No session ID in cookie"));
            }
        } else {
            return Ok(bad_request("Invalid cookie header"));
        }
    }

    //Extracting loginInfo
    let login: LoginInfo = match extract_from_request(body).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Create a session for succesful login
    match users_list.find_user(login).await {
        Ok(user_id) => match users_list.add_session(user_id).await {
            Ok(session) => {
                let session_id = session.session_id();
                let cookie = format!("session_id={}; HttpOnly; Path=/", session_id);

                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
                    .body(Body::from("Successfully logged in"))
                    .unwrap();

                users_list.print_sessions().await;

                return Ok(response);
            }
            Err(err_msg) => return Ok(bad_request(&err_msg)),
        },
        Err(err_msg) => return Ok(bad_request(&err_msg)),
    }

    // Ok(Response::new(Body::from("Successfully logged in")))
}
fn extract_session_id(cookie_str: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        let trimmed = part.trim();
        if let Some(session_id) = trimmed.strip_prefix("session_id=") {
            return Some(session_id.to_string());
        }
    }
    None
}

async fn home_page_put(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - home_page_put");

    let (path, body) = request.into_parts();
    let path_segments = path.uri.path().split("/").collect::<Vec<&str>>();
    //println!("{:?}", path_segments);

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
    if let Err(err_msg) = users_list
        .update_user(user, path_segments[2].parse::<usize>().unwrap())
        .await
    {
        return Ok(bad_request(&err_msg));
    }

    users_list.print_users().await;

    Ok(Response::new(Body::from("Succesful updated user")))
}

async fn home_page_get() -> Result<Response<Body>, Infallible> {
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

    let user: User = match extract_from_request(request.into_body()).await {
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

async fn extract_from_request<T: Extractable>(body: Body) -> Result<T, Response<Body>> {
    let body_in_bytes = to_bytes(body).await.map_err(|err| {
        //handle error
        println!("->> Error in parsing request body {}", err);

        bad_request("Could not parse request body")
    })?;

    serde_json::from_slice(&body_in_bytes).map_err(|err| {
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
