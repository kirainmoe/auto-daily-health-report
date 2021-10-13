use std::error::Error;

use super::client::create_client;
use super::login::login;
use super::query::is_today_reported;
use super::report::{report, ReportStage};

use serde_json::Value;

use crate::print_on_debug_env;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum PipelineStage {
  BeforePipeline,
  LoginSuccess,
  AlreadyReportedToday,
  Prechecked,
  ReportRequestedSuccess,
  ReportRequestedFailed,
  Success,
  Failed,
}

trait UpdateStage {
  fn update(&mut self, target: PipelineStage);
}

impl UpdateStage for PipelineStage {
  fn update(&mut self, target: PipelineStage) {
    if *self < target {
      *self = target;
    }
  }
}

pub struct PipelineResult {
  pub status: PipelineStage,
  pub post_data: Option<Value>,
}

pub async fn pipeline(username: &str, password: &str, retries: i32) -> Result<PipelineResult, Box<dyn Error>> {
  let client = create_client().await?;
  let mut _stage = PipelineStage::BeforePipeline;
  let mut _post_data: Option<Value> = None;
  let mut tries = 0;

  print_on_debug_env!("[Debug] Running health-report pipeline for user: {}", username);

  while tries < retries {
    tries += 1;

    if _stage != PipelineStage::ReportRequestedSuccess {
      _stage = PipelineStage::BeforePipeline;
    }

    let pipeline_result: Result<bool, Box<dyn Error>> = {
      print_on_debug_env!("[{}/{}] Stage 1: Performing login()...", tries, retries);
      login(&client, username, password).await?;
      _stage.update(PipelineStage::LoginSuccess);

      print_on_debug_env!("[{}/{}] Stage 2: Reading current report status...", tries, retries);
      let (today_report_status, _) = is_today_reported(&client).await?;
      if today_report_status {
        match _stage {
          PipelineStage::ReportRequestedSuccess => _stage.update(PipelineStage::Success),
          _ => _stage.update(PipelineStage::AlreadyReportedToday),
        };

        Ok(true)
      } else {
          print_on_debug_env!("[{}/{}] Stage 3: Making health report request...", tries, retries);
          let report_result = report(&client).await?;
          if report_result.status_code == ReportStage::ReportSuccess {
            print_on_debug_env!("[Debug] health report request successfully end.");
            _stage.update(PipelineStage::ReportRequestedSuccess);
            _post_data = report_result.post_data;
          } else {
            print_on_debug_env!("[Debug] health report request failed, err: {}", report_result.error_message.unwrap());
            _stage.update(PipelineStage::ReportRequestedFailed);
          }

          if _stage == PipelineStage::ReportRequestedSuccess {
            print_on_debug_env!("[{}/{}] Stage 4: Re-check health report status to ensure...", tries, retries);
            let (today_report_status, _) = is_today_reported(&client).await?;
            if today_report_status {
              _stage.update(PipelineStage::Success);
            } else {
              _stage.update(PipelineStage::Failed);
            }
          }
    
          Ok(true)
      }
    };

    if pipeline_result? {
      break;
    }
  }

  Ok(PipelineResult {
    status: _stage,
    post_data: _post_data,
  })
}