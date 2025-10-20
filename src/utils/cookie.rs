use hyper::{Body, HeaderMap, Response, header};

use crate::{structs::Constants, utils::response_bad_request};

pub fn extract_session_id_from_header(header: &HeaderMap) -> Result<String, Response<Body>> {
    let Some(cookie_header) = header.get(header::COOKIE) else {
        return Err(response_bad_request("No cookie found"));
    };

    let Ok(cookie_str) = cookie_header.to_str() else {
        return Err(response_bad_request("Invalid cookie header"));
    };

    let Some(session_id) = extract_session_id_from_cookie(cookie_str) else {
        return Err(response_bad_request("No session ID in cookie"));
    };
    Ok(session_id)
}

fn extract_session_id_from_cookie(cookie_str: &str) -> Option<String> {
    cookie_str
        .split(';')
        .map(str::trim)
        .find_map(|el| el.strip_prefix(Constants::SESSION_ID_KEY).map(String::from))
}
