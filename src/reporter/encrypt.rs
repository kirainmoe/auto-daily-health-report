use aes::Aes128;
use base64::encode;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use regex::Regex;

type AesCbc = Cbc<Aes128, Pkcs7>;

const CHARSET: &str = "ABCDEFGHJKMNPQRSTWXYZabcdefhijkmnprstwxyz2345678";

pub fn rand_str(len: i32) -> String {
  let mut res = String::new();
  let charset: Vec<char> = CHARSET.chars().collect();
  let charset_length = charset.len();
  for _i in 0..len {
    let index = rand::thread_rng().gen_range(0..charset_length);
    res += &charset[index as usize].to_string();
  }
  res
}

pub fn gas<'a>(data: &'a str, key: &'a str, iv: &'a str) -> String {
  let reg = Regex::new(r"(^\s+)|(\s+$)").unwrap();
  let trimed_key = reg.replace_all(key, "");

  let cipher = AesCbc::new_from_slices(trimed_key.as_bytes(), iv.as_bytes()).unwrap();
  let cipher_text = cipher.encrypt_vec(data.as_bytes());
  let result = encode(cipher_text);

  result
}

/// 统一身份认证 AES-CBC 加密函数
pub fn encrypt_aes_cbc<'a>(data: &'a str, p1: &'a str) -> String {
  if p1.len() == 0 {
    return data.to_owned();
  }
  let data = rand_str(64) + data;
  let iv = rand_str(16);
  let result = gas(&data, p1, &iv);

  result
}
