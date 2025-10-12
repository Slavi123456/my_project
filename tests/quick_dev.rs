use anyhow::{Context, Result};
use hyper::{
    Body, Client, Method, Request, Response, Uri,
    body::to_bytes,
    client::{self, HttpConnector},
    header::CONTENT_TYPE,
};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = Client::new();
    send_get("/home", &client).await?;

    //Post request for the home page
    let data = r#"
        {
            "first_name": "John",
            "last_name": "Doe",
            "email": "john@doe.com",
            "password": "johnDoe123"
        }"#;

    send_post("/home", &client, data).await?;
    send_post("/home", &client, data).await?;

    Ok(())
}
async fn send_post(path: &str, client: &Client<HttpConnector>, data: &str) -> Result<()> {
    let uri = make_uri(path)?;
    let req = Request::builder()
        .method(&Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(data.to_owned()))
        .unwrap();

    let resp = client
        .request(req)
        .await
        .context("Failed to send GET request")?;

    parse_response(resp).await
}

async fn send_get(path: &str, client: &Client<HttpConnector>) -> Result<()> {
    //Get request for the home page
    let uri = make_uri(path)?;
    let resp = client
        .get(uri)
        .await
        .context("Failed to send GET request")?;

    parse_response(resp).await
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
