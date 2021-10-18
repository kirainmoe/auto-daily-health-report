use xmu_health_report_rust_sdk::{
  create_client, get_system_date, is_today_reported, login, pipeline, report,
};
use xmu_health_report_rust_sdk::{PipelineStage, ReportStage};

use std::collections::HashMap;

use clap::{App, AppSettings, Arg};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
  let matches = App::new("XMU Daily Health Report Rust CLI - 厦门大学每日健康打卡工具")
    .version("1.0")
    .author("kirainmoe <kirainmoe@gmail.com>")
    .subcommand(
      App::new("report")
        .about("Make health report request - 发送健康打卡请求")
        .arg(
          Arg::new("username")
            .about("统一身份认证 ID （学工号）")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::new("password")
            .about("统一身份认证密码)")
            .required(true)
            .index(2),
        ),
    )
    .subcommand(
      App::new("query")
        .about("Query today's health report status - 查询今日打卡状态")
        .arg(
          Arg::new("username")
            .about("统一身份认证 ID （学工号）")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::new("password")
            .about("统一身份认证密码)")
            .required(true)
            .index(2),
        ),
    )
    .subcommand(
      App::new("pipeline")
        .about("Start a health report pipeline - 启动健康打卡流程")
        .arg(
          Arg::new("username")
            .about("统一身份认证 ID （学工号）")
            .required(true)
            .index(1),
        )
        .arg(
          Arg::new("password")
            .about("统一身份认证密码)")
            .required(true)
            .index(2),
        )
        .arg(Arg::new("retry").long("retry").about("重试次数")),
    )
    .setting(AppSettings::ArgRequiredElseHelp)
    .get_matches();

  if let Some(ref matches) = matches.subcommand_matches("report") {
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();

    let client = create_client().await?;
    login(&client, username, password).await?;
    let report_result = report(&client, &HashMap::new()).await?;

    println!(
      "打卡状态: {}",
      if report_result.status_code == ReportStage::ReportSuccess {
        "打卡成功"
      } else {
        "打卡失败"
      }
    );

    if report_result.status_code != ReportStage::ReportSuccess {
      println!("最后完成阶段: {:?}", report_result.status_code);
      println!("错误原因: {:?}", report_result.error_message);
    }
  }

  if let Some(ref matches) = matches.subcommand_matches("query") {
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();

    let client = create_client().await?;
    login(&client, username, password).await?;
    let (today, form_date) = is_today_reported(&client).await?;

    println!("本地日期: {}", get_system_date());
    println!("服务器表单日期: {}", form_date);
    println!("今日打卡状态: {}", if today { "已打卡" } else { "未打卡" });
  }

  if let Some(ref matches) = matches.subcommand_matches("pipeline") {
    let username = matches.value_of("username").unwrap();
    let password = matches.value_of("password").unwrap();
    let retries = matches.value_of("retry").unwrap().parse::<i32>()?;
    let pipeline_result = pipeline(username, password, retries).await?;

    println!(
      "打卡状态: {}",
      if pipeline_result.status == PipelineStage::Success {
        "打卡成功"
      } else {
        "打卡失败"
      }
    );

    if pipeline_result.status != PipelineStage::Success {
      println!("最后完成阶段: {:?}", pipeline_result.status);
    }
  }

  Ok(())
}
