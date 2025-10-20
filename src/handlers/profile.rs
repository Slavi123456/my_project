use std::convert::Infallible;

use hyper::{Body, Request, Response};

use crate::{
    structs::{Routes, app_state::AppState, user::User},
    utils::{
        deserialize_json_body, extract_session_id_from_header,
        response::{redirect_with_cookie, redirect_without_cookie},
        response_bad_request,
    },
};

pub async fn handle_put_profile(
    request: Request<Body>,
    app_state: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - home_page_put");

    let (parts, body) = request.into_parts();

    //Checking for existing session
    let session_id = match extract_session_id_from_header(&parts.headers) {
        Ok(id) => id,
        Err(error) => return Ok(error),
    };

    //Validate the session if not return to the login page
    if !app_state.is_session_valid(&session_id).await {
        let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");
        let response = redirect_with_cookie(&cookie, Routes::LOGIN, "Invalid session");
        return Ok(response);
    }

    //Get the user id from the session
    let user_id = match app_state.get_user_id_from_session(&session_id).await {
        Ok(id) => id,
        Err(error) => return Ok(response_bad_request(&error)),
    };

    //Reading the request body
    let user: User = match deserialize_json_body(body).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Validation
    if let Err(err_msg) = user.validate() {
        return Ok(response_bad_request(&err_msg));
    }

    //Updating the user
    if let Err(err_msg) = app_state.update_user(user, user_id).await {
        return Ok(response_bad_request(&err_msg));
    }

    app_state.print_users().await;

    //transfer to the home page
    let response = redirect_without_cookie(Routes::LOGIN, "Succesfully updated user");

    Ok(response)
}
