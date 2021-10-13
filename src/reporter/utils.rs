use chrono::{DateTime, Utc};

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
  let now: DateTime<Utc> = Utc::now();
  let result = format!("{}", now.format("%Y-%m-%d"));

  result
}
