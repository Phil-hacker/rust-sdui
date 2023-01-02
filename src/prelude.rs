use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

lazy_static!{
    pub(crate) static ref CLIENT: reqwest::Client = reqwest::Client::builder().https_only(true).user_agent(concat!(env!("CARGO_PKG_NAME"),"/",env!("CARGO_PKG_VERSION"),)).build().unwrap_or_default();
}

#[derive(Debug)]
pub struct RateLimit {
    pub limit: u64,
    pub remaining: u64
}

impl RateLimit {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        RateLimit { 
            limit: headers.get("x-ratelimit-limit").unwrap_or(&HeaderValue::from_str("0").unwrap()).to_str().unwrap_or("0").to_string().parse::<u64>().unwrap_or(0),
            remaining: headers.get("x-ratelimit-limit").unwrap_or(&HeaderValue::from_str("0").unwrap()).to_str().unwrap_or("0").to_string().parse::<u64>().unwrap_or(0)
        }
    }
}

#[derive(Debug)]
pub enum SduiError {
    RequestError(reqwest::Error),
    JSONError,
    NotLoggedIn,
    LoginError,
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