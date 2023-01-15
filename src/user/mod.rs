use reqwest::StatusCode;
use serde::Deserialize;

use crate::{prelude::*, grade::Grade};

pub async fn get_self(token: &String) -> Result<(SduiUser,RateLimit),SduiError> {
    let response = CLIENT.get("https://api.sdui.app/v1/users/self").bearer_auth(token).send().await.map_err(SduiError::RequestError)?;
    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(SduiError::NotLoggedIn);
    }
    let rate_limit = RateLimit::from_headers(response.headers());
    let data = response.json::<SduiResponse<SduiUser>>().await.map_err(SduiError::RequestError)?;
    Ok((data.data,rate_limit))
}

pub async fn get_user(token: &String,user_id: &String) -> Result<(SduiUser,RateLimit),SduiError> {
    let response = CLIENT.get(format!("https://api.sdui.app/v1/users/{}",user_id)).bearer_auth(token).send().await.map_err(SduiError::RequestError)?;
    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(SduiError::NotLoggedIn);
    }
    let rate_limit = RateLimit::from_headers(response.headers());
    let data = response.json::<SduiResponse<SduiUser>>().await.map_err(SduiError::RequestError)?;
    Ok((data.data,rate_limit))
}

#[derive(Debug,Deserialize)]
pub struct SduiUser {
    pub uuid: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(rename="type")]
    pub role: String,
    pub title: Option<String>,
    pub sex: char,
    pub state: String,
    pub expire_at: Option<u64>,
    pub locale: String,
    pub shortcut: Option<String>,
    pub shortcut_id: Option<u64>,
    pub grade: Grade,
    pub grade_id: u64,
    pub meta: UserMeta,
}

#[derive(Debug,Deserialize)]
pub struct UserMeta {
    pub displayname: String,
    pub subtitle: String,
    #[serde(rename="type")]
    pub role: String,
    pub uri: String,
    pub avatar_uri: String,
    pub salutation: String,
    pub days_until_deletion: u64,
    pub is_signed: bool,
    pub is_paused: bool,
    pub archived_at: Option<u64>,
    pub deleted_at: Option<u64>,
    pub is_trackable_classbook_user: bool,
    pub calendar_notification_count: u64,
}