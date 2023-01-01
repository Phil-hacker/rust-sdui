use std::str;

use serde_json::Number;

use crate::prelude::*;
pub async fn search_schools(school: &str) -> Result<Vec<School>,SduiError> {
    let response = reqwest::get(format!("https://api.sdui.app/v1/leads?search={}",school)).await.map_err(|_| SduiError::RequestError)?;
    let data = response.json::<GenericSduiResponse>().await.map_err(|_| SduiError::RequestError)?;
    let schools = data.data.as_array().ok_or(SduiError::RequestError)?
    .iter()
    .filter_map(|school| School::from_value(school))
    .collect();
    Ok(schools)
}

pub async fn login() {

}

#[derive(Debug)]
pub struct School {
    id: u64,
    name: String,
    name_alias: Option<String>,
    slink: String,
}

impl School {
    fn from_value(value: &serde_json::Value) -> Option<Self> {
        let map = value.as_object()?;
        Some(School {
            id: map.get("id")?.as_u64()?,
            name: map.get("name")?.as_str()?.to_string(),
            name_alias: map.get("name_alias")?.as_str().map(|str| str.to_string()),
            slink: map.get("slink")?.as_str()?.to_string(),
        })
    }
}