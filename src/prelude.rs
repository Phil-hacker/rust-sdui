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


#[derive(Debug)]
pub struct School {
    id: u64,
    name: String,
    name_alias: Option<String>,
    slink: String,
    uuid: String
}

impl School {
    pub(crate) fn from_value(value: &serde_json::Value) -> Option<Self> {
        let map = value.as_object()?;
        Some(School {
            id: map.get("id")?.as_u64()?,
            name: map.get("name")?.as_str()?.to_string(),
            name_alias: map.get("name_alias")?.as_str().map(|str| str.to_string()),
            slink: map.get("slink")?.as_str()?.to_string(),
            uuid: map.get("uuid")?.as_str()?.to_string()
        })
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_name_alias(&self) -> Option<String> {
        self.name_alias.clone()
    }
    pub fn get_slink(&self) -> String {
        self.slink.clone()
    }
    pub fn get_uuid(&self) -> String {
        self.uuid.clone()
    }
}