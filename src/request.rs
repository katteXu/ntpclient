use std::time::Duration;

use anyhow::Result;
use reqwest::header::HeaderMap;

pub async fn request(cf_who: &str) -> Result<String> {
    let client = reqwest::ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .build()?;

    let mut headers = HeaderMap::new();
    headers.insert("cf-who", cf_who.parse()?);
    let response = client
        .get("https://ntp.cloudflarenet.workers.dev/synctime2")
        .headers(headers)
        .send()
        .await?;
    let result = response.text().await?;
    Ok(result)
}

#[tokio::test]
async fn test_request() {
    let result = request("test").await;

    println!("{:?}", result);
}
