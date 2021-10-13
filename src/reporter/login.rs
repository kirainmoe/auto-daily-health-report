use reqwest::{Client};
use scraper::{Html, Selector};
use std::error::Error;

use super::encrypt::encrypt_aes_cbc;

const OAUTH_URL: &str = "https://ids.xmu.edu.cn/authserver/login?service=https://xmuxg.xmu.edu.cn/login/cas/xmu";

async fn select_input_value<'a>(document: &'a Html, select_target: &str) -> Result<&'a str, Box<dyn Error>> {
  let selector = Selector::parse(select_target)
    .or(Err(format!("Create selector {} failed!", select_target)))?;

  let target = document
    .select(&selector)
    .next()
    .ok_or(format!("Cannot find {}!", select_target))?;
  
  let value = target
    .value()
    .attr("value")
    .ok_or(format!("Cannot get input value!"))?;

  Ok(value)
}

/// 学工系统统一身份认证登录
/// 
/// ```
/// let client = create_client().await?;
/// login(client, username, password).await?;
/// ```
pub async fn login(client: &Client, username: &str, password: &str) -> Result<bool, Box<dyn Error>> {
  let login_page_resp = client
    .get(OAUTH_URL)
    .send()
    .await?
    .text()
    .await?;

  let document = Html::parse_document(&login_page_resp);

  let lt = select_input_value(&document, "input[name='lt']").await?;
  let dllt = select_input_value(&document, "input[name='dllt']").await?;
  let execution = select_input_value(&document, "input[name='execution']").await?;
  let salt = select_input_value(&document, "input#pwdDefaultEncryptSalt").await?;
  let password = encrypt_aes_cbc(&password, &salt);

  let post_form = [
    ("username", username),
    ("password", &password),
    ("lt", lt),
    ("dllt", dllt),
    ("execution", execution),
    ("_eventId", "submit"),
    ("rmShown", "1"),
  ];

  let login_resp = client
    .post(OAUTH_URL)
    .form(&post_form)
    .send()
    .await?;
  
  let response_url = login_resp.url();
  assert_eq!(response_url.host_str(), Some("xmuxg.xmu.edu.cn"));

  Ok(true)
}