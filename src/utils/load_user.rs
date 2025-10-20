use std::convert::Infallible;

use hyper::{Body, Request, Response};

use crate::{
    structs::{Routes, app_state::AppState, user::UserProfile},
    utils::{
        extract_session_id_from_header,
        response::{redirect_with_cookie, response_with_json},
        response_bad_request,
    },
};

pub async fn load_user_data(
    request: Request<Body>,
    app_state: AppState,
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
    if !app_state.is_session_valid(&session_id).await {
        let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");
        let response = redirect_with_cookie(&cookie, Routes::LOGIN, "Invalid session");
        return Ok(response);
    }

    //get User profile from session id
    let user_profile: UserProfile = match app_state
        .get_user_profile_from_session_id(&session_id)
        .await
    {
        Ok(profile) => profile,
        Err(err) => return Ok(response_bad_request(&err)),
    };

    //Make json
    let profile_json = match serde_json::to_string(&user_profile) {
        Ok(json) => json,
        Err(err) => {
            println!("Error in parsing UserProfile to json");
            return Ok(response_bad_request(&err.to_string()));
        }
    };

    let response = response_with_json(profile_json);

    Ok(response)
}
