use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub async fn get_cloud(token: &str, id: u64) -> SduiResult<Cloud> {
    request(
        &format!("https://api.sdui.app/v1/users/self/channels/cloud/{}", id),
        token,
    )
    .await
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Cloud {
    pub can: CloudCan,
    pub disabled_at: Option<String>,
    pub id: u64,
    pub meta: CloudMeta,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CloudCan {
    pub upload: u8,
    #[serde(rename = "create-protected-folder")]
    pub create_protected_folder: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CloudMeta {
    pub download: Option<String>,
    pub forbidden: Vec<String>,
    pub max_number: u64,
    pub rename: Option<String>,
    pub upload: Option<String>,
    pub upload_limit: Option<u64>,
}
