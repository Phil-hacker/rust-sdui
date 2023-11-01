use crate::channel::Attachment;
use crate::user::PartialSduiUser;
use crate::{channel::Channel, prelude::*};
use serde::{Deserialize, Serialize};

pub async fn get_self_news(token: &str, page: u64) -> SduiResult<Vec<News>> {
    request(
        &format!("https://api.sdui.app/v1/users/self/feed/news?page={}", page),
        token,
    )
    .await
}

pub async fn get_news(token: &str, user_id: u64, page: u64) -> SduiResult<Vec<News>> {
    request(
        &format!(
            "https://api.sdui.app/v1/users/{}/feed/news?page={}",
            user_id, page
        ),
        token,
    )
    .await
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct News {
    pub attachments: Vec<Attachment>,
    pub can: NewsCan,
    pub channel_pivot: Vec<ChannelPivot>,
    pub channels: Vec<Channel>,
    pub content: String,
    pub content_rendered: String,
    pub created_at: String,
    pub has_emergency_sms: bool,
    pub has_translations: bool,
    pub id: u64,
    pub is_confirmable: bool,
    pub is_public: bool,
    pub is_official: u8,
    pub is_pinned: bool,
    pub meta: NewsMeta,
    pub preview: NewsPreview,
    pub publish_at: Option<String>,
    pub survey: Option<Survey>,
    pub survey_uuid: Option<String>,
    pub title: String,
    pub updated_at: Option<String>,
    pub user: PartialSduiUser,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NewsPreview {
    Attachment(Attachment),
    String(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ChannelPivot {
    news_id: u64,
    channel_id: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NewsCan {
    pub confirm: u8,
    pub delete: u8,
    pub notify: u8,
    pub pin: u8,
    pub update: u8,
    #[serde(rename = "view-statistics")]
    pub view_statistics: u8,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NewsMeta {
    pub confirm_uri: bool,
    pub csv: String,
    pub is_confirmed: bool,
    pub languages: Vec<String>,
    pub statistics: NewsStatistics,
    pub uri: String,
    pub xls: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NewsStatistics {
    pub confirmed: NewsStatisticsConfirmed,
    pub readby: NewsStatisticsReadBy,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NewsStatisticsConfirmed {
    pub total: u64,
    pub current: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct NewsStatisticsReadBy {
    pub total: u64,
    pub current: u64,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Survey {
    pub can: SurveyCan,
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub ended_at: Option<String>,
    pub expires_at: Option<String>,
    pub has_translations: bool,
    pub id: u64,
    pub is_anonymous: bool,
    pub is_freetext: bool,
    pub is_multi_answerable: bool,
    pub meta: SurveyMeta,
    pub question: String,
    pub results_visibility: String,
    pub updated_at: Option<String>,
    pub user: PartialSduiUser,
    pub uuid: String,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SurveyCan {
    pub delete: u8,
    pub download: u8,
    pub end: u8,
    pub results: u8,
    pub revoke: u8,
    pub view: u8,
    pub vote: u8,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SurveyMeta {
    pub csv: Option<String>,
    pub is_over: bool,
    pub is_user_voted: bool,
    pub languages: Vec<String>,
    pub options: Vec<SurveyOption>,
    pub xls: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SurveyOption {
    pub count: u64,
    pub is_chosen: bool,
    pub name: String,
    pub percentage: f32,
    pub uuid: String,
}
