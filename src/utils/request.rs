use hyper::{Body, Response, StatusCode, body::to_bytes};

use crate::structs::traits::Extractable;

pub async fn extract_from_request<T: Extractable>(body: Body) -> Result<T, Response<Body>> {
    let body_in_bytes = to_bytes(body).await.map_err(|err| {
        //handle error
        println!("->> Error in parsing request body {}", err);

        bad_request("Could not parse body to bytes")
    })?;

    serde_json::from_slice(&body_in_bytes).map_err(|err| {
        //handle error
        println!("->> Error in parsing json {}", err);
        bad_request("Could not parse bytes to Extractable struct")
    })
}

pub fn bad_request(msg: &str) -> Response<Body> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .header("Content-Type", "text/plain")
        .body(Body::from(msg.to_string()))
        .unwrap()
}
