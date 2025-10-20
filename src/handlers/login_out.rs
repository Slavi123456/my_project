use std::convert::Infallible;

use hyper::{
    Body, Request, Response, StatusCode,
    header::{HeaderValue, LOCATION, SET_COOKIE},
};

use crate::{
    structs::{Routes, app_state::AppState, login::LoginInfo},
    utils::{
        extract_from_request, extract_session_id_from_header, response::redirect_with_cookie,
        response_bad_request,
    },
};

///////////////////////////////////////////////////////////////////////////

pub async fn handle_delete_logout(
    request: Request<Body>,
    mut users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_delete_logout");

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

pub async fn handle_post_login(
    request: Request<Body>,
    app_state: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_post_login");

    let (parts, body) = request.into_parts();

    //Checking for already existing session
    if let Ok(session_id) = extract_session_id_from_header(&parts.headers) {
        println!("->> Session ID found: {}", session_id);

        //Validate the session if not return to the login page
        if !app_state.is_session_valid(&session_id).await {
            //If not expire the cookie and transfer to the login page
            let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");
            let response = redirect_with_cookie(&cookie, Routes::LOGIN, "Invalid session");

            return Ok(response);
        }

        //Transfer the user to he home page
        let cookie = format!("session_id={}; HttpOnly; Path=/; Max-Age=0", session_id);
        let response = redirect_with_cookie(&cookie, Routes::HOME, "Already logged in");
        app_state.print_sessions().await;

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
    let cookie = match app_state.find_user(login).await {
        Ok(user_id) => match app_state.add_session(user_id).await {
            Ok(session) => {
                let session_id = session.session_id();
                app_state.print_sessions().await;
                &format!("session_id={}; HttpOnly; Path=/", session_id)
            }
            Err(err_msg) => return Ok(response_bad_request(&err_msg)),
        },
        Err(err_msg) => return Ok(response_bad_request(&err_msg)),
    };

    //Create response with the cookie and the redirecting to the home page
    let response = redirect_with_cookie(cookie, Routes::HOME, "Successfully logged in");

    Ok(response)
}
