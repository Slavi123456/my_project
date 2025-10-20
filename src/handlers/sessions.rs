use std::convert::Infallible;

use hyper::{Body, Response};

use crate::{
    structs::{Routes, app_state::AppState},
    utils::response::redirect_with_cookie,
};

pub async fn handle_existing_session_in_login(
    app_state: &AppState,
    session_id: &str,
) -> Result<Response<Body>, Infallible> {
    //Create respond depending on the validation of the session
    let response = match app_state.is_session_valid(&session_id).await {
        true => {
            let cookie = format!("session_id={}; HttpOnly; Path=/; Max-Age=0", session_id);
            redirect_with_cookie(&cookie, Routes::HOME, "Already logged in")
        }
        false => {
            let cookie = format!("session_id=; HttpOnly; Path=/; Max-Age=0");
            redirect_with_cookie(&cookie, Routes::LOGIN, "Invalid session")
        }
    };
    app_state.print_sessions().await;

    Ok(response)
}
