use std::{convert::Infallible, fs::read, net::SocketAddr, path::PathBuf};

use hyper::{
    Body, Method, Request, Response, Server, StatusCode,
    body::to_bytes,
    header::{COOKIE, HeaderValue, LOCATION, SET_COOKIE},
    service::{make_service_fn, service_fn},
};
use tokio::fs::read_to_string;

use crate::user::{AppState, Extractable, LoginInfo, User};

mod user;

#[tokio::main]
async fn main() {
    // Hardcoded connection string
    let _db_url = "mysql://root:rootpassword@localhost:3306/mydb";

    let app_state = match AppState::new_without_db().await {
        //AppState::new(db_url).await {
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
        (&Method::PUT, req_path) if req_path.starts_with("/home/") => {
            home_page_put(request, users_list).await
        }
        (&Method::GET, "/login") => page_get("login.html").await,
        (&Method::POST, "/login") => login_page_post(request, users_list).await,

        (&Method::DELETE, "/logout") => logout_page_delete(request, users_list).await,

        (&Method::GET, "/register") => page_get("register.html").await,
        (&Method::POST, "/register") => register_page_post(request, users_list).await,

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

async fn handle_static_file(path: &str) -> Result<Response<Body>, Infallible> {
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
        Err(_) => Ok(bad_request("Failed to load css")),
    }
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

    let (parts, body) = request.into_parts();

    //Checking for already existing session
    let header = parts.headers.get(COOKIE);

    if let Some(cookie_header) = header {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Extract session_id from the cookie string
            if let Some(session_id) = extract_session_id(cookie_str) {
                println!("->> Session ID found: {}", session_id);

                users_list.delete_session(&session_id).await;
                users_list.print_sessions().await;

                let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");

                let response = Response::builder()
                    .status(StatusCode::OK)
                    .header(SET_COOKIE, HeaderValue::from_str(&cookie).unwrap())
                    .body(Body::from("Successfully logged in"))
                    .unwrap();

                return Ok(response);
            } else {
                return Ok(bad_request("No session ID in cookie"));
            }
        } else {
            return Ok(bad_request("Invalid cookie header"));
        }
    }

    return Ok(bad_request("No cookie found"));
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
                    .status(StatusCode::FOUND)
                    .header(SET_COOKIE, HeaderValue::from_str(&session_id).unwrap())
                    .header(LOCATION, "/home")
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

async fn extract_from_request<T: Extractable>(body: Body) -> Result<T, Response<Body>> {
    let body_in_bytes = to_bytes(body).await.map_err(|err| {
        //handle error
        println!("->> Error in parsing request body {}", err);

        bad_request("Could not parse body to bytes")
    })?;

    serde_json::from_slice(&body_in_bytes).map_err(|err| {
        //handle error
        println!("->> Error in parsing json {}", err);
        bad_request("Could not parse bytes to Extractable struct")
    })
}

fn bad_request(msg: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "text/plain")
        .body(Body::from(msg.to_string()))
        .unwrap()
}
