use std::convert::Infallible;

use hyper::{
    Body, Request, Response, StatusCode,
    header::{CONTENT_TYPE, HeaderValue, LOCATION, SET_COOKIE},
};

use crate::{
    extract_session_id_from_header,
    structs::{app_state::AppState, user::UserProfile},
    utils::bad_request,
};

pub async fn load_user_data(
    request: Request<Body>,
    users_list: AppState,
) -> Result<Response<Body>, Infallible> {
    println!("->> HANDLER - load_user_data");

    let (parts, _body) = request.into_parts();

    let session_id = match extract_session_id_from_header(&parts.headers) {
        Ok(id) => id,
        Err(err) => {
            return Ok(err);
        }
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

    //get User profile from session id
    let user_profile: UserProfile = match users_list
        .get_user_profile_from_session_id(&session_id)
        .await
    {
        Ok(profile) => profile,
        Err(err) => return Ok(bad_request(&err)),
    };

    //Make json
    let profile_json = match serde_json::to_string(&user_profile) {
        Ok(json) => json,
        Err(err) => {
            println!("Error in parsing UserProfile to json");
            return Ok(bad_request(&err.to_string()));
        }
    };

    let response = Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(profile_json))
        .unwrap();

    Ok(response)
}
