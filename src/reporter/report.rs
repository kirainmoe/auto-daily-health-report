use reqwest::Client;

use serde_json::{from_str, Value};

use std::error::Error;
use std::str::FromStr;

use serde_json::json;
use serde_json::map::Map;

use super::client::get;
use super::constant::{
  CURRENT_FORM_URL, 
  FORM_RENDER_DATA_URL, 
  MY_FORM_INSTANCE_URL, 
  POST_CHANGE_URL
};
use crate::print_on_debug_env;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReportStage {
  BeforeReport,
  GetFormIdDone,
  GetFormInstanceDone,
  GetOwnerModificationRecordDone,
  ChangeFormContentDone,
  ConstructPostDataDone,
  ReportSuccess,
  ReportFailed,
}

pub struct ReportResult {
  pub status_code: ReportStage,
  pub error_message: Option<Box<dyn Error>>,
  pub post_data: Option<Value>,
}


/// 发送打卡请求
/// 
/// @param `client: &Client`  已登录过的 reqwest 会话
/// 
/// @return  `Result<ReportResult, Box<dyn Error>>`
pub async fn report(client: &Client) -> Result<ReportResult, Box<dyn Error>> {
  let mut _stage = ReportStage::BeforeReport;

  let report_result: Result<Value, Box<dyn Error>> = {
    // 获取每日表单的 form ID
    let resp = get(&client, CURRENT_FORM_URL).await?;
    let form_business_json: Value = from_str(&resp)?;
    let form_id = form_business_json["data"][0]["business"]["id"].to_string();

    print_on_debug_env!("[Debug] Form business ID of today: {}", &form_id);

    _stage = ReportStage::GetFormIdDone;

    // 获取表单数据列
    let form_render_data_url = FORM_RENDER_DATA_URL.replace("$1", &form_id[..]);
    let resp = get(&client, &form_render_data_url).await?;
    let form_instance_json: Value = from_str(&resp)?;
    let form_components = form_instance_json["data"]["components"]
      .as_array()
      .ok_or("Cannot destruct form_components")?;

    _stage = ReportStage::GetFormInstanceDone;

    // 获取表单所有者修改记录信息
    let my_form_instance_url = MY_FORM_INSTANCE_URL.replace("$1", &form_id[..]);
    let resp = get(&client, &my_form_instance_url).await?;
    let mut my_form_instance_json: Value = from_str(&resp)?;

    let form_json = &mut my_form_instance_json["data"];
    let id_value = form_json["id"].clone();
    let instance_id = id_value.as_str().ok_or("Cannot destruct intance_id!")?;

    print_on_debug_env!("[Debug] Form instance ID of today: {}", instance_id);

    _stage = ReportStage::GetOwnerModificationRecordDone;

    // 修改表单内容
    let form_data = form_json["formData"]
      .as_array_mut()
      .ok_or("Cannot destruct formData")?;
    let mut post_value: Map<String, Value> = Map::new();
    for item in form_data.iter() {
      let name = item["name"].as_str().ok_or("Cannot get name of field")?;

      let title = item["title"].as_str().ok_or("Cannot get title of field")?;

      let new_value: serde_json::Value = {
        // 勾选本人填写
        if String::from_str(title)?.contains("学生本人是否填写") {
          json!({
            "stringValue": "是",
          })
        } else if String::from_str(title)?.contains("Can you hereby declare that") {
          json!({
            "stringValue": "是 Yes",
          })
        }
        // 填入以往的表单
        else {
          let value = item.clone();

          if value["value"]["dataType"] == "STRING" {
            json!({
              "stringValue": value["value"]["stringValue"],
            })
          } else if value["value"]["dataType"] == "ADDRESS_VALUE" {
            json!({
              "addressValue": value["value"]["addressValue"],
            })
          } else {
            json!({})
          }
        }
      };

      let post_field_payload = json!({
        "hide": item["hide"].clone(),
        "readonly": item["readonly"].clone(),
        "title": title,
        "value": new_value,
      });

      // 暂存表单项目
      post_value.insert(name.to_string(), post_field_payload);
    }
    _stage = ReportStage::ChangeFormContentDone;

    // 构造表单
    let mut post_array: Vec<Value> = Vec::new();
    for item in form_components.iter() {
      let name = item["name"].as_str().ok_or("Cannot destruct name")?;

      if post_value.contains_key(name) {
        // 因为上一步一定断言了 contains_key，所以这里直接 unwrap
        let field_item = post_value.get(name).unwrap();

        let mut hide = if field_item["hide"].is_null() {
          true
        } else {
          field_item["hide"].as_bool().ok_or("Cannot destruct hide")?
        };

        if String::from_str(name)?.contains("select")
          && !field_item["value"]
            .as_object()
            .ok_or("Cannot convert post_value to object")?
            .contains_key("stringValue")
          && field_item["value"]["stringValue"] == ""
        {
          hide = true;
        }

        post_array.push(json!({
          "name": name,
          "title": field_item["title"],
          "value": field_item["value"],
          "hide": hide,
        }));
      } else {
        post_array.push(json!({
          "name": name,
          "title": item["title"],
          "value": {},
          "hide": !String::from_str(name)?.contains("label"),
        }));
      }
    }

    let post_json = json!({
      "formData": post_array,
      "playerId": "owner",
    });

    _stage = ReportStage::ConstructPostDataDone;

    // 发送请求
    let report_resp = client
      .post(POST_CHANGE_URL.replace("$1", instance_id))
      .json(&post_json)
      .send()
      .await?
      .text()
      .await?;

    let report_result_json: Value = from_str(&report_resp)?;

    if report_result_json["state"]
      .as_bool()
      .ok_or("Report response invalid")?
      == true
    {
      _stage = ReportStage::ReportSuccess;
    } else {
      _stage = ReportStage::ReportFailed;
    }

    Ok(json!(post_array))
  };

  let return_payload = match report_result {
    Ok(data) => ReportResult {
      status_code: _stage,
      post_data: Some(data),
      error_message: None,
    },
    Err(e) => ReportResult {
      status_code: _stage,
      post_data: None,
      error_message: Some(e),
    },
  };

  Ok(return_payload)
}
