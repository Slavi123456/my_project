use anyhow::{Context, Result};
use hyper::{Body, Client, Method, Request, Response, Uri, body::to_bytes, header::CONTENT_TYPE};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = Client::new();

    //Get request for the home page
    let home_uri = make_uri("/home")?;
    let home_resp_get = client
        .get(home_uri)
        .await
        .context("Failed to send GET request")?;

    parse_response(home_resp_get).await?;

    //Post request for the home page
    let data = r#"
        {
            "username": "John",
            "password": "Doe"
        }"#;

    let home_uri = make_uri("/home")?;
    let home_post_req = Request::builder()
        .method(&Method::POST)
        .uri(home_uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(data))
        .unwrap();

    let home_resp_post = client
        .request(home_post_req)
        .await
        .context("Failed to send GET request")?;

    parse_response(home_resp_post).await?;

    Ok(())
}

fn make_uri(path: &str) -> Result<Uri, anyhow::Error> {
    let base_path = "http://127.0.0.1:3000";
    let whole_path = format!("{}{}", base_path, path);
    Ok(whole_path.parse::<Uri>()?)
}

async fn parse_response(response: Response<Body>) -> Result<()> {
    let resp_statuc = response.status();
    let resp_body = to_bytes(response.into_body())
        .await
        .context("Failed to read response body")?;
    let body_to_str = String::from_utf8_lossy(&resp_body);

    println!("=== HTTP Response Report ===");
    println!("Statuc code {}", resp_statuc);
    println!("===Response body=== \n");
    println!("{}", &body_to_str);

    Ok(())
}
