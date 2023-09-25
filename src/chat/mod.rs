use serde::{Deserialize, Serialize};

use crate::{cloud::Cloud, prelude::*};

pub struct ChatRequest {
    token: String,
    page: u64,
    limit: u64,
    with_archived: bool,
    search: String,
}

impl ChatRequest {
    pub fn new(token: &str) -> Self {
        ChatRequest {
            token: token.to_owned(),
            page: 1,
            limit: 10,
            with_archived: false,
            search: String::new(),
        }
    }

    pub fn page(mut self, page: u64) -> Self {
        self.page = page;
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit = limit;
        self
    }

    pub fn search(mut self, search: &str) -> Self {
        self.search = search.to_owned();
        self
    }

    pub fn with_archived(mut self, with_archived: bool) -> Self {
        self.with_archived = with_archived;
        self
    }

    pub async fn request(&self) -> SduiResult<SduiResponse<Vec<Chat>>> {
        request(
            &format!(
                "https://api.sdui.app/v1/users/self/channels/chats?&with_archived={}&page={}&search={}&limit={}",
                self.with_archived, self.page, self.search, self.limit
            ),
            &self.token,
        )
        .await
    }
}

pub async fn get_chat(token: &str, id: &u64) -> SduiResult<SduiResponse<Chat>> {
    request(&format!("https://api.sdui.app/v1/channels/{}", id), token).await
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Chat {
    pub activity_at: String,
    pub admin_ids: Vec<u64>,
    pub avatar: Option<String>,
    pub calendar_id: u64,
    pub can: ChatCan,
    pub chat: PartialChat,
    pub chat_id: u64,
    pub cloud: Cloud,
    pub cloud_id: u64,
    pub code: String,
    pub color: Option<String>,
    pub content_move_decision_mate_at: Option<String>,
    pub created_at: String,
    pub description: String,
    pub description_members: String,
    pub disabled_by_id: Option<u64>,
    pub expiration_reason: Option<String>,
    pub expires_at: Option<String>,
    pub group: Option<String>,
    pub icon: Option<String>,
    pub id: u64,
    pub intern_id: String,
    pub is_disabled: bool,
    pub is_hidden_memberlist: bool,
    pub is_leavable: bool,
    pub is_public: bool,
    pub is_twoway: bool,
    pub meta: ChatMeta,
    pub name: String,
    pub school: School,
    pub school_id: u64,
    pub subtitle: Option<String>,
    pub target: String,
    pub trashed_at: Option<String>,
    pub twoway_expires_at: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: String,
    pub updated_at: String,
    pub user_id: Option<u64>,
    pub users_count: u64,
    pub uuid: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChatCan {
    #[serde(rename = "create-survey")]
    pub create_survey: u8,
    pub delete: u8,
    #[serde(rename = "delete-message-history")]
    pub delete_message_history: u8,
    pub knock: u8,
    pub leave: u8,
    #[serde(rename = "manage-admins")]
    pub manage_admins: u8,
    #[serde(rename = "manage-users")]
    pub manage_users: u8,
    #[serde(rename = "move_channel_content")]
    pub move_channel_content: u8,
    pub pin: u8,
    #[serde(rename = "start-conference")]
    pub start_conference: u8,
    #[serde(rename = "toggle-memberlist")]
    pub toggle_memberlist: u8,
    #[serde(rename = "toggle-state")]
    pub toggle_state: u8,
    #[serde(rename = "toggle_twoway")]
    pub toggle_twoway: u8,
    pub update: u8,
    #[serde(rename = "view-users")]
    pub view_users: u8,
    #[serde(rename = "voice-memo")]
    pub voice_memo: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartialChat {
    pub can: PartialChatCan,
    pub disabled_at: Option<String>,
    pub id: u8,
    pub meta: Vec<PartialChatMeta>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartialChatCan {
    #[serde(rename = "post-message")]
    pub post_message: u8,
    #[serde(rename = "toggle-oneway")]
    pub toggle_oneway: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartialChatMeta {}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChatMeta {
    pub description: String,
    pub displayname: String,
    pub is_archived: u8,
    pub is_moveable: bool,
    pub is_muted: bool,
    pub is_official: bool,
    pub is_paused: bool,
    pub is_pinned: bool,
    pub is_unread: bool,
    pub languages: Vec<String>,
    pub last_knocked_at: Option<String>,
    pub last_unread_count: u8,
    pub next_possible_knock: NextPossibleKnock,
    pub read_at: String,
    pub shortcut: String,
    pub subtitle: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NextPossibleKnock {}
