use chrono::{DateTime, Local};
use std::time::{SystemTime, UNIX_EPOCH};

#[macro_export]
macro_rules! print_on_debug_env {
  () => ({
    match std::env::var("DEBUG") {
      Ok(_) => println!("\n"),
      Err(_) => {}
    }
  });
  ($($arg:tt)*) => ({
    match std::env::var("DEBUG") {
      Ok(_) => println!($($arg)*),
      Err(_) => {}
    }
  })
}

/// 获取当前系统日期，格式为 YYYY-MM-DD
pub fn get_system_date() -> String {
  let now: DateTime<Local> = Local::now();
  let result = format!("{}", now.format("%Y-%m-%d"));

  result
}

/// 获取时间戳
pub fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}