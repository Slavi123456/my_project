use std::convert::Infallible;

use hyper::{
    Body, Request, Response, StatusCode,
    header::{HeaderValue, LOCATION, SET_COOKIE},
};

use crate::{
    structs::{app_state::AppState, user::User},
    utils::{bad_request, extract_from_request, extract_session_id_from_header},
};

pub async fn profile_page_put(
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
