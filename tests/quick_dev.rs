use anyhow::Result;
use hyper::{Client, Uri, body::HttpBody};

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let client = Client::new();
    let uri = "http://127.0.0.1:3000/home".parse::<Uri>()?;

    let mut response = client.get(uri).await?;
    let data = response.body_mut().data().await.unwrap().unwrap();

    println!("=== HTTP Response Report ===");
    println!("Statuc code {}", response.status());
    println!("{:?}", data);

    Ok(())
}
