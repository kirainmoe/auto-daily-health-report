use reqwest::header;
use reqwest::Client;

use std::error::Error;

use super::constant::{USER_AGENT, REFERER};

pub async fn create_client() -> Result<Client, Box<dyn Error>> {
  let mut headers = header::HeaderMap::new();
  headers.insert("User-Agent", header::HeaderValue::from_static(USER_AGENT));
  headers.insert("Referer", header::HeaderValue::from_static(REFERER));

  let client = reqwest::Client::builder()
    .cookie_store(true)
    .default_headers(headers)
    .build()?;
  
  Ok(client)
}

pub async fn get(client: &Client, url: &str) -> Result<String, Box<dyn Error>> {
  let text = client
    .get(url)
    .send()
    .await?
    .text()
    .await?;

  Ok(text)
}