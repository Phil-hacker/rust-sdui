use serde::{Deserialize, Serialize};

use crate::{grade::Grade, prelude::*};

pub async fn get_self(token: &str) -> Result<(SduiUser, RateLimit), SduiError> {
    request("https://api.sdui.app/v1/users/self", token).await
}

pub async fn get_user(
    token: &String,
    user_id: &String,
) -> Result<(SduiUser, RateLimit), SduiError> {
    request(&format!("https://api.sdui.app/v1/users/{}", user_id), token).await
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SduiUser {
    pub uuid: String,
    pub firstname: String,
    pub lastname: String,
    #[serde(rename = "type")]
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct UserMeta {
    pub displayname: String,
    pub subtitle: String,
    #[serde(rename = "type")]
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
