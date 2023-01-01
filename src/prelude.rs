use serde::Deserialize;
#[derive(Debug)]

pub enum SduiError {
    RequestError,
    NotLoggedIn
}
pub type GenericSduiResponse = SduiResponse<serde_json::Value>;

#[derive(Deserialize, Debug)]
pub struct SduiResponse<T> {
    pub data: T,
    pub status: String,
    pub meta: SduiMeta
}

#[derive(Deserialize, Debug)]
pub struct SduiMeta {
    pub warnings: serde_json::Value,
    pub errors: serde_json::Value,
    pub success: serde_json::Value
}