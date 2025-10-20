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

pub fn redirect_without_cookie(route: &str, body_text: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::FOUND)
        .header(header::LOCATION, route)
        .body(Body::from(body_text.to_string()))
        .unwrap()
}

pub fn response_with_json(json: String) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json))
        .unwrap()
}

pub fn response_ok_with_content(content: Vec<u8>, content_type: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", content_type)
        .body(Body::from(content))
        .unwrap()
}
