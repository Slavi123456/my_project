use hyper::{
    Body, Response, StatusCode,
    header::{self, HeaderValue},
};

pub fn response_bad_request(msg: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "text/plain")
        .body(Body::from(msg.to_string()))
        .unwrap()
}

pub fn redirect_with_cookie(cookie: &str, route: &str, body_text: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::SET_COOKIE, HeaderValue::from_str(cookie).unwrap())
        .header(header::LOCATION, route)
        .body(Body::from(body_text.to_string()))
        .unwrap()
}
