use anyhow::{Context, Result};
use hyper::{
    Body, Client, Method, Request, Response, Uri,
    body::to_bytes,
    client::HttpConnector,
    header::{CONTENT_TYPE, COOKIE, HeaderValue},
};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = Client::new();
    //Get request for the home page
    // send_get("/home", &client).await?;

    //Post request for the home page
    let data = r#"
        {
            "first_name": "John",
            "last_name": "Doe",
            "email": "john@doe.com",
            "password": "johnDoe123"
        }"#;

    send_post("/home", &client, data, None).await?;
    // send_post("/home", &client, data).await?;

    //Put request for the home page
    let put_data = r#"
        {
            "first_name": "Johny",
            "last_name": "Doeee",
            "email": "john@doe.com",
            "password": "johnDoe12345"
        }"#;

    send_put("/home/0", &client, put_data).await?;

    //Send post request for login
    // let login_data = r#"
    // {
    //     "email": "john@doe.com",
    //     "password": "johnDoe123"
    // }"#;
    // send_post("/login", &client, login_data, None).await?;

    //Checking if the cookie works
    // send_post("/login", &client, login_data, Some("0")).await?;

    //Sending delete request
    // send_delete("/logout", &client, Some("0")).await?;

    Ok(())
}
async fn send_delete(
    path: &str,
    client: &Client<HttpConnector>,
    session_id: Option<&str>,
) -> Result<()> {
    let uri = make_uri(path)?;

    //Adding the optional session_id
    let cookie = match session_id {
        Some(id) => HeaderValue::from_str(&format!("session_id={};", id)).unwrap(),
        None => HeaderValue::from_str("No cookies here").unwrap(),
    };

    let req = Request::builder()
        .method(&Method::DELETE)
        .uri(uri)
        .header(COOKIE, cookie)
        .body(Body::from("Loggin out"))
        .unwrap();

    let resp = client
        .request(req)
        .await
        .context("Failed to send POST request")?;

    parse_response(resp).await
}

async fn send_put(path: &str, client: &Client<HttpConnector>, data: &str) -> Result<()> {
    let uri = make_uri(path)?;
    let req = Request::builder()
        .method(&Method::PUT)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(data.to_owned()))
        .unwrap();

    let resp = client
        .request(req)
        .await
        .context("Failed to send PUT request")?;

    parse_response(resp).await
}

async fn send_post(
    path: &str,
    client: &Client<HttpConnector>,
    data: &str,
    session_id: Option<&str>,
) -> Result<()> {
    let uri = make_uri(path)?;

    //Adding the optional session_id
    let cookie = match session_id {
        Some(id) => HeaderValue::from_str(&format!("session_id={};", id)).unwrap(),
        None => HeaderValue::from_str("No cookies here").unwrap(),
    };

    let req = Request::builder()
        .method(&Method::POST)
        .uri(uri)
        .header(CONTENT_TYPE, "application/json")
        .header(COOKIE, cookie)
        .body(Body::from(data.to_owned()))
        .unwrap();

    let resp = client
        .request(req)
        .await
        .context("Failed to send POST request")?;

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
    let (parts, body) = response.into_parts();

    let resp_statuc = parts.status;
    let resp_header = parts.headers;
    let resp_body = to_bytes(body)
        .await
        .context("Failed to read response body")?;
    let body_to_str = String::from_utf8_lossy(&resp_body);

    println!("=== HTTP Response Report ===");
    println!("Statuc code {}", resp_statuc);
    println!("Headers: ");
    for (key, value) in resp_header.iter() {
        println!("{}, {}", key, value.to_str().unwrap_or("invalid"));
    }
    println!("===Response body=== \n");
    println!("{}", &body_to_str);

    Ok(())
}
