use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::prelude::*;

pub async fn get_timetable(
    token: &str,
    user_id: &str,
    begin: &Date,
    end: &Date,
) -> Result<(TimeTable, RateLimit), SduiError> {
    let response = CLIENT
        .get(format!("https://api.sdui.app/v1/timetables/users/{}/timetable?begins_at={}-{}-{}&ends_at={}-{}-{}",user_id,begin.year,begin.month,begin.day,end.year,end.month,end.day))
        .bearer_auth(token)
        .send()
        .await
        .map_err(SduiError::RequestError)?;
    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(SduiError::NotLoggedIn);
    }
    let rate_limit = RateLimit::from_headers(response.headers());
    let data = response
        .json::<SduiResponse<TimeTable>>()
        .await
        .map_err(SduiError::RequestError)?;
    Ok((data.data, rate_limit))
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: i32,
}

impl Date {
    pub fn new(day: u8, month: u8, year: i32) -> Self {
        Date { day, month, year }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TimeTable {
    pub lessons: Vec<Lesson>,
    pub last_updated_at: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Lesson {
    pub bookables: Vec<Bookable>,
    pub grades: Vec<Grade>,
    pub teachers: Vec<Teacher>,
    pub id: u64,
    pub begins_at: u64,
    pub ends_at: u64,
    pub comment: String,
    pub course: Course,
    pub meta: LessonMeta,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Teacher {
    pub id: u64,
    pub name: String,
    pub shortcut: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Grade {
    pub id: u64,
    pub name: String,
    pub shortcut: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Bookable {
    pub id: u64,
    pub name: String,
    pub shortcut: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Course {
    pub meta: CourseMeta,
    pub subject: Subject,
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub subject_id: u64,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CourseMeta {
    pub displayname: String,
    pub shortname: String,
    pub color: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct LessonMeta {
    pub displayname_hour: String,
    pub moved_comment: String,
    pub displayname: String,
    pub shortname: String,
    pub displayname_kind: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Subject {
    pub color: String,
    pub meta: SubjectMeta,
    pub id: u64,
    pub shortcut: String,
    pub name: String,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SubjectMeta {
    pub displayname: String,
}

enum LessonType {
    NORMAL,
    SUBSTITUTION,
    CANCLED,
    ADDITIONAL,
    UNKNOWN,
}

impl From<Option<String>> for LessonType {
    fn from(value: Option<String>) -> Self {
        if let Some(value) = value {
            return match value.as_str() {
                "CANCLED" => Self::CANCLED,
                "SUBSTITUTION" => Self::SUBSTITUTION,
                "ADDITIONAL" => Self::ADDITIONAL,
                _ => Self::UNKNOWN,
            };
        }
        Self::NORMAL
    }
}
