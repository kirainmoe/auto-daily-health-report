use reqwest::{Client};
use anyhow::anyhow;

use crate::print_on_debug_env;
use super::encrypt::encrypt_aes_cbc;

use regex::Regex;

const OAUTH_URL: &str = "https://ids.xmu.edu.cn/authserver/login?service=https://xmuxg.xmu.edu.cn/login/cas/xmu";


/// simple regex to match input with specific attribute
/// since rust scraper is not compatible with async
async fn select_input_value_with_attr<'a>
  (document: &'a str, attr: &'a str, value: &'a str) -> Result<String, anyhow::Error> {
  let re = Regex::new(&format!(r##"<input.*{}=['"]{}['"].*value=['"](.*)['"].*[/]>"##, attr, value))?;
  for cap in re.captures_iter(document) {
    return Ok(String::from(&cap[1]));
  }
  Err(anyhow!("match failed!"))
}

/// 学工系统统一身份认证登录
/// 
/// ```
/// let client = create_client().await?;
/// login(client, username, password).await?;
/// ```
pub async fn login(client: &Client, username: &str, password: &str) -> Result<bool, anyhow::Error> {
  let login_page_resp = client
    .get(OAUTH_URL)
    .send()
    .await?
    .text()
    .await?;

  let lt = select_input_value_with_attr(&login_page_resp, "name", "lt").await?;
  let execution = select_input_value_with_attr(&login_page_resp, "name", "execution").await?;
  let salt = select_input_value_with_attr(&login_page_resp, "id", "pwdDefaultEncryptSalt").await?;
  let password = encrypt_aes_cbc(&password, &salt);

  print_on_debug_env!("Session Info:\nlt = {}\nexecution = {}\nsalt={}\n", &lt, &execution, &salt);

  let post_form = [
    ("username", username),
    ("password", &password),
    ("lt", &lt),
    ("dllt", "userNamePasswordLogin"),
    ("execution", &execution),
    ("_eventId", "submit"),
    ("rmShown", "1"),
  ];

  let login_resp = client
    .post(OAUTH_URL)
    .form(&post_form)
    .send()
    .await?;
  
  let response_url = login_resp.url();

  if response_url.host_str() != Some("xmuxg.xmu.edu.cn") {
    return Err(anyhow!("Login failed!"));
  }

  Ok(true)
}