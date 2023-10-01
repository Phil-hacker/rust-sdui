use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Channel {
    pub meta: ChannelMeta,
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub description_members: Option<String>,
    pub subtitle: Option<String>,
    #[serde(rename = "type")]
    pub channel_type: String,
    pub uuid: String,
    pub user_id: Option<u64>,
    pub school_id: u64,
    pub chat_id: u64,
    pub cloud_id: u64,
    pub calendar_id: u64,
    pub target: String,
    pub intern_id: String,
    pub avatar: Option<String>,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub is_leavable: bool,
    pub is_public: bool,
    pub is_disabled: bool,
    pub is_twoway: bool,
    pub is_hidden_memberlist: bool,
    pub twoway_expires_at: Option<String>,
    pub activity_at: Option<String>,
    pub expires_at: Option<String>,
    pub expiration_reason: Option<String>,
    pub trashed_at: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub group: String,
    pub disabled_by_id: Option<u64>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ChannelMeta {
    pub is_official: u8,
    pub subtitle: String,
    pub displayname: String,
    pub shortcut: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Attachment {
    pub created_at: String,
    pub extension: String,
    pub file_type: String,
    pub id: u64,
    pub meta: AttachmentMeta,
    pub name: String,
    pub size: u64,
    pub source_id: u64,
    pub source_type: String,
    #[serde(rename = "type")]
    pub mime_type: String,
    pub updated_at: Option<String>,
    pub user_id: u64,
    pub uuid: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AttachmentMeta {
    pub download_uri: String,
    pub temp_uri: String,
    pub uri: String,
}
