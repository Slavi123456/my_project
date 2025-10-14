use std::convert::Infallible;

use hyper::{
    Body, Request, Response, StatusCode,
    header::{HeaderValue, LOCATION, SET_COOKIE},
};

use crate::{
    structs::{app_state::AppState, login::LoginInfo},
    utils::{bad_request, extract_from_request, extract_session_id_from_header},
};

///////////////////////////////////////////////////////////////////////////

pub async fn logout_page_delete(
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

///////////////////////////////////////////////////////////////////////////

pub async fn login_page_post(
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
