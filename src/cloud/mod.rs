use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartialCloud {
    pub can: PartialCloudCan,
    pub disabled_at: Option<String>,
    pub id: u64,
    pub meta: PartialCloudMeta,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartialCloudCan {
    pub upload: u8,
    #[serde(rename = "create-protected-folder")]
    pub create_protected_folder: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PartialCloudMeta {
    pub download: Option<String>,
    pub forbidden: Vec<String>,
    pub max_number: u64,
    pub rename: Option<String>,
    pub upload: Option<String>,
    pub upload_limit: Option<u64>,
}
