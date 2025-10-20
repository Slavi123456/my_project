use std::convert::Infallible;

use hyper::{Body, Request, Response};

use crate::{
    structs::{Routes, app_state::AppState, user::User},
    utils::{deserialize_json_body, response::redirect_without_cookie, response_bad_request},
};

pub async fn handle_post_register(
    request: Request<Body>,
    app_state: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_post_register");

    //Checking for already existing session

    //Extract user
    let user: User = match deserialize_json_body(request.into_body()).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Validation
    if let Err(err_msg) = user.validate() {
        return Ok(response_bad_request(&err_msg));
    }
    //Saving the user information
    if let Err(err) = app_state.add_user(user).await {
        return Ok(response_bad_request(&format!("{}", err)));
    }
    app_state.print_users().await;

    //Transfer to the login page
    let response = redirect_without_cookie(Routes::LOGIN, "Succesfully registered!");

    Ok(response)
}
