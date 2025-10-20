use std::convert::Infallible;

use hyper::{Body, Request, Response, StatusCode, header::LOCATION};

use crate::{
    structs::{app_state::AppState, user::User},
    utils::{extract_from_request, response_bad_request},
};

pub async fn handle_post_register(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - handle_post_register");

    let user: User = match extract_from_request(request.into_body()).await {
        Ok(u) => u,
        Err(err) => return Ok(err),
    };

    //Validation
    if let Err(err_msg) = user.validate() {
        return Ok(response_bad_request(&err_msg));
    }
    //Saving the user information
    if let Err(err) = users_list.add_user(user).await {
        return Ok(response_bad_request(&format!("{}", err)));
    }
    users_list.print_users().await;

    //Transfer to the login page
    let respone = Response::builder()
        .status(StatusCode::FOUND)
        .header(LOCATION, "/login")
        .body(Body::empty())
        .unwrap();
    Ok(respone)
}
