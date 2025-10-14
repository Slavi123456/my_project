use hyper::{Body, HeaderMap, Response, header::COOKIE};

use crate::utils::bad_request;

pub fn extract_session_id_from_header(header: &HeaderMap) -> Result<String, Response<Body>> {
    if let Some(cookie_header) = header.get(COOKIE) {
        if let Ok(cookie_str) = cookie_header.to_str() {
            // Extract session_id from the cookie string
            if let Some(session_id) = extract_session_id_from_cookie(cookie_str) {
                println!("->> Session ID found: {}", session_id);

                Ok(session_id)
            } else {
                return Err(bad_request("No session ID in cookie"));
            }
        } else {
            return Err(bad_request("Invalid cookie header"));
        }
    } else {
        return Err(bad_request("No cookie found"));
    }
}

fn extract_session_id_from_cookie(cookie_str: &str) -> Option<String> {
    for part in cookie_str.split(';') {
        let trimmed = part.trim();
        if let Some(session_id) = trimmed.strip_prefix("session_id=") {
            return Some(session_id.to_string());
        }
    }
    None
}
