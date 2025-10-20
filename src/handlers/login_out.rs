use std::convert::Infallible;

use hyper::{Body, Request, Response};

use crate::{
    handlers::sessions::handle_existing_session_in_login,
    structs::{Routes, app_state::AppState, login::LoginInfo},
    utils::{
        deserialize_json_body, extract_session_id_from_header, response::redirect_with_cookie,
        response_bad_request,
    },
};

///////////////////////////////////////////////////////////////////////////

pub async fn handle_delete_logout(
    request: Request<Body>,
    mut app_state: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_delete_logout");

    let (parts, _body) = request.into_parts();

    //Checking for already existing session
    let session_id = match extract_session_id_from_header(&parts.headers) {
        Ok(id) => id,
        Err(error) => return Ok(error),
    };

    //Update App state
    app_state.delete_session(&session_id).await;
    app_state.print_sessions().await;

    //Transfer to the login page with expired cookie
    let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");
    let response = redirect_with_cookie(&cookie, Routes::LOGIN, "Successfully logged out");

    Ok(response)
}

///////////////////////////////////////////////////////////////////////////

pub async fn handle_post_login(
    request: Request<Body>,
    app_state: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_post_login");

    let (parts, body) = request.into_parts();

    //Checking for already existing session
    let session_id = match extract_session_id_from_header(&parts.headers) {
        Ok(id) => {
            println!("->> Session ID found: {}", id);

            return handle_existing_session_in_login(&app_state, &id).await;
        }
        Err(err) => {
            //Handle error
        }
    };

    //Extracting loginInfo
    let login: LoginInfo = match deserialize_json_body(body).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Check for valid user
    let user_id = match app_state.find_user(login).await {
        Ok(id) => id,
        Err(err_msg) => return Ok(response_bad_request(&err_msg)),
    };
    //Create session
    let session = match app_state.add_session(user_id).await {
        Ok(session) => session,
        Err(err_msg) => return Ok(response_bad_request(&err_msg)),
    };
    app_state.print_sessions().await;

    //Create response with the cookie and the redirecting to the home page
    let cookie = format!("session_id={}; HttpOnly; Path=/", session.session_id());
    let response = redirect_with_cookie(&cookie, Routes::HOME, "Successfully logged in");

    Ok(response)
}
