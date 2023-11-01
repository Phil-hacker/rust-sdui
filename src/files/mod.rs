use bytes::Bytes;
use serde::{Deserialize, Serialize};

use crate::{cloud::Cloud, prelude::*, user::PartialSduiUser};

pub struct FileRequest {
    token: String,
    page: u64,
    parent: Option<File>,
    limit: u64,
    order_direction: OrderDirection,
    order_by: OrderBy,
    search: String,
}

pub enum OrderDirection {
    ASCENDING,
    DESCENDING,
}

impl ToString for OrderDirection {
    fn to_string(&self) -> String {
        match self {
            OrderDirection::ASCENDING => "asc".to_owned(),
            OrderDirection::DESCENDING => "desc".to_owned(),
        }
    }
}
pub enum OrderBy {
    NAME,
    SIZE,
    TYPE,
    CREATEDAT,
}

impl ToString for OrderBy {
    fn to_string(&self) -> String {
        match self {
            OrderBy::NAME => "name".to_owned(),
            OrderBy::SIZE => "size".to_owned(),
            OrderBy::TYPE => "type".to_owned(),
            OrderBy::CREATEDAT => "created_at".to_owned(),
        }
    }
}

impl FileRequest {
    pub fn new(token: &str) -> Self {
        FileRequest {
            token: token.to_owned(),
            parent: None,
            page: 1,
            limit: 10,
            order_by: OrderBy::NAME,
            order_direction: OrderDirection::ASCENDING,
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

    pub fn parent(mut self, parent: File) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn order_by(mut self, order_by: OrderBy) -> Self {
        self.order_by = order_by;
        self
    }

    pub fn order_direction(mut self, order_direction: OrderDirection) -> Self {
        self.order_direction = order_direction;
        self
    }

    pub async fn request(self) -> SduiResult<Vec<File>> {
        request(
            &format!(
                "https://api.sdui.app/v1/users/self/channels/chats?file={}&order-dir={}&order-by={}&page={}&search={}&limit={}",
                self.parent.map_or_else(|| "".to_owned(), |v| v.uuid),self.order_direction.to_string(), self.order_by.to_string(), self.page, self.search, self.limit
            ),
            &self.token,
        )
        .await
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct File {
    pub cloud: Cloud,
    pub cloud_id: u64,
    pub created_at: String,
    pub deleted_at: Option<String>,
    pub description: Option<String>,
    pub disk_id: Option<u64>,
    pub duration_in_seconds: Option<u64>,
    pub expires_at: Option<String>,
    pub extension: Option<String>,
    pub file_type: String,
    pub has_thumbnail: Option<bool>,
    pub hash: Option<String>,
    pub is_collaborative: bool,
    pub is_protected_folder: u8,
    pub meta: FileMeta,
    pub name: String,
    pub parent: Option<Box<File>>,
    pub parent_id: String,
    pub path: String,
    pub referenced_permissions: Option<String>,
    pub referenced_until: Option<String>,
    pub referenced_uuid: Option<String>,
    pub reserved: Option<String>,
    pub size: u64,
    #[serde(rename = "type")]
    pub mime_type: Option<String>,
    pub updated_at: String,
    pub upload_limited_at: Option<String>,
    pub user: Option<PartialSduiUser>,
    pub user_id: Option<u64>,
    pub uuid: String,
}

impl File {
    pub async fn download(&self) -> Result<Bytes, SduiError> {
        download(&self.meta.download_uri).await
    }
    pub async fn content(&self, token: &str) -> SduiResult<Vec<File>> {
        request(&self.meta.content_uri, token).await
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct FileMeta {
    pub absolute_path: String,
    pub content_uri: String,
    pub details_uri: String,
    pub download_uri: String,
    pub edit_access_token: Option<String>,
    pub edit_uri: Option<String>,
    pub files_count: u64,
    pub has_audo: u8,
    pub has_image: u8,
    pub has_text_document: u8,
    pub has_thumbnail: u8,
    pub has_video: u8,
    pub has_wopi_support: u8,
    pub location: String,
    pub parent: Option<Box<File>>,
    pub subtitle: Option<String>,
    pub thumbnail_uri: Option<String>,
    pub uri: String,
    pub username: Option<String>,
}
