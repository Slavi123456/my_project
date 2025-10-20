use hyper::{Body, Response, body::to_bytes};

use crate::{structs::traits::Extractable, utils::response_bad_request};

pub async fn extract_from_request<T: Extractable>(body: Body) -> Result<T, Response<Body>> {
    let body_in_bytes = to_bytes(body).await.map_err(|err| {
        //handle error
        println!("->> Error in parsing request body {}", err);

        response_bad_request("Could not parse body to bytes")
    })?;

    serde_json::from_slice(&body_in_bytes).map_err(|err| {
        //handle error
        println!("->> Error in parsing json {}", err);
        response_bad_request("Could not parse bytes to Extractable struct")
    })
}
