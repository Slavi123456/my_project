use hyper::{
    Body, Response,
    body::{Bytes, to_bytes},
};

use crate::{structs::traits::Extractable, utils::response_bad_request};

pub async fn deserialize_json_body<T: Extractable>(body: Body) -> Result<T, Response<Body>> {
    let body_in_bytes = to_bytes(body).await.map_err(|err| {
        println!("->> Error in parsing request body {}", err);

        response_bad_request("Could not parse request body to bytes")
    })?;

    parse_json_struct(body_in_bytes)
}

fn parse_json_struct<T: Extractable>(bytes: Bytes) -> Result<T, Response<Body>> {
    serde_json::from_slice(&bytes).map_err(|err| {
        println!("->> Error in parsing json {}", err);

        response_bad_request("Could not parse body's bytes to an Extractable struct")
    })
}
