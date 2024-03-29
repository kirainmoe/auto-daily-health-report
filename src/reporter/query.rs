use reqwest::Client;

use serde_json::{from_str, Value};

use super::client::get;
use super::constant::{
    CHANGELOG_URL, CURRENT_FORM_URL, MY_FORM_INSTANCE_URL, PROFILE_URL, QRCODE_URL,
};
use super::utils::get_system_date;
use crate::print_on_debug_env;

use anyhow::anyhow;

/// 修改记录
#[derive(Debug)]
pub struct ModifyLogResult {
    pub logs: Vec<Value>,
    pub date: String,
}

/// 个人资料
#[derive(Debug)]
pub struct ProfileResult {
    pub name: String,
    pub id: String,
}

/// 获取当日表单修改记录
///
/// @param `client: &Client`
///
/// @return `Result<ModifyLogResult, anyhow::Error>
pub async fn modify_log(client: &Client) -> Result<ModifyLogResult, anyhow::Error> {
    // 获取每日表单的 businessID
    let resp = get(&client, CURRENT_FORM_URL).await?;
    let form_business_json: Value = from_str(&resp)?;
    let form_id = form_business_json["data"][0]["business"]["id"].to_string();
    let form_date = form_business_json["data"][0]["business"]["name"]
        .as_str()
        .ok_or(anyhow!("Cannot destruct name"))?;

    print_on_debug_env!("[Debug] Form business ID of today: {}", &form_id);

    // 获取表单 instanceID
    let my_form_instance_url = MY_FORM_INSTANCE_URL.replace("$1", &form_id[..]);
    let resp = get(&client, &my_form_instance_url).await?;
    let mut my_form_instance_json: Value = from_str(&resp)?;
    let form_json = &mut my_form_instance_json["data"];
    let id_value = form_json["id"].clone();
    let instance_id = id_value
        .as_str()
        .ok_or(anyhow!("Cannot destruct intance_id!"))?;

    print_on_debug_env!("[Debug] Form instance ID of today: {}", &instance_id);

    // 获取修改记录
    let changelog_url = CHANGELOG_URL
        .replace("$1", &instance_id[..])
        .replace("$2", &form_id[..]);
    let resp = get(&client, &changelog_url).await?;
    let changelog_json: Value = from_str(&resp)?;
    let changelogs: Vec<Value> = changelog_json["data"]["logs"]
        .as_array()
        .ok_or(anyhow!("Cannot convert changelogs into array!"))?
        .to_vec();
    Ok(ModifyLogResult {
        logs: changelogs,
        date: String::from(form_date),
    })
}

/// 获取当日表单最后修改打卡时间
pub async fn last_update_time(client: &Client) -> Result<String, anyhow::Error> {
    // 获取每日表单的 form ID
    let resp = get(&client, CURRENT_FORM_URL).await?;
    let form_business_json: Value = from_str(&resp)?;
    let form_id = form_business_json["data"][0]["business"]["id"].to_string();

    print_on_debug_env!("[Debug] Form business ID of today: {}", &form_id);

    // 获取表单所有者修改记录信息
    let my_form_instance_url = MY_FORM_INSTANCE_URL.replace("$1", &form_id[..]);
    let resp = get(&client, &my_form_instance_url).await?;
    let mut my_form_instance_json: Value = from_str(&resp)?;

    let form_json = &mut my_form_instance_json["data"];
    let id_value = form_json["id"].clone();
    let instance_id = id_value
        .as_str()
        .ok_or(anyhow!("Cannot destruct intance_id!"))?;

    print_on_debug_env!("[Debug] Form instance ID of today: {}", instance_id);

    let form_data = form_json["formData"]
        .as_array_mut()
        .ok_or(anyhow!("Cannot destruct formData"))?;

    // 提取 “打卡时间（无需填写，保存后会自动更新）”字段内容
    for item in form_data.iter() {
        let name = item["name"]
            .as_str()
            .ok_or(anyhow!("Cannot get name of field"))?;
        let title = item["title"]
            .as_str()
            .ok_or(anyhow!("Cannot get title of field"))?;
        if title.to_owned().contains("打卡时间")
            || name.to_owned().contains("datetime_1611146487222")
        {
            let date = item["value"]["dateValue"]
                .as_str()
                .ok_or(anyhow!("Cannot destruct date"))?;

            print_on_debug_env!("[Debug] Remote last modified date (full): {}", date);

            let splited_result: Vec<&str> = date.split(" ").collect();

            if splited_result.len() != 2 {
                return Err(anyhow!("Cannot get last modified date"));
            }

            return Ok(splited_result[0].to_owned());
        }
    }

    return Err(anyhow!("Cannot get last modified date"));
}

/// 检查今日是否打卡
///
/// @param `client: &Client`
///
/// @return `Result<bool, anyhow::Error>`
pub async fn is_today_reported(client: &Client) -> Result<(bool, String), anyhow::Error> {
    let last_modified_time = last_update_time(&client).await?;
    let today_date = get_system_date();

    print_on_debug_env!("[Debug] Remote form date is: {}", last_modified_time);
    print_on_debug_env!("[Debug] Current system date is: {}", today_date);

    Ok((last_modified_time == today_date, last_modified_time))
}

/// 获取连续打卡天数
///
/// @param `client: &Client`
///
/// @return `Result<i64, anyhow::Error>
pub async fn get_continuous_report_day_count(client: &Client) -> Result<i64, anyhow::Error> {
    let resp = get(&client, QRCODE_URL).await?;
    let resp_json: Value = from_str(&resp)?;
    let days_count_str = resp_json["data"]["clockDay"]
        .as_str()
        .ok_or(anyhow!("Cannot destruct clockDay."))?;

    let days_count = days_count_str.parse::<i64>()?;

    Ok(days_count)
}

/// 获取个人资料
///
/// @param `client: &Client`
///
/// @return `Result<ProfileResult, anyhow::Error>`
pub async fn get_profile(client: &Client) -> Result<ProfileResult, anyhow::Error> {
    let resp = get(&client, PROFILE_URL).await?;
    let resp_json: Value = from_str(&resp)?;

    let name = resp_json["data"]["name"]
        .as_str()
        .ok_or(anyhow!("parse name failed"))?;
    let id = resp_json["data"]["userNo"]
        .as_str()
        .ok_or(anyhow!("parse id failed"))?;

    Ok(ProfileResult {
        name: name.to_string(),
        id: id.to_string(),
    })
}
