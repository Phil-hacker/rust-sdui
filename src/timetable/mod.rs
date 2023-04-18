use reqwest::StatusCode;
use serde::{de::Visitor, Deserialize, Serialize};

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

pub async fn get_times(token: &str) -> Result<(Vec<Time>, RateLimit), SduiError> {
    let response = CLIENT
        .get("https://api.sdui.app/v1/timetables/times")
        .bearer_auth(token)
        .send()
        .await
        .map_err(SduiError::RequestError)?;
    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(SduiError::NotLoggedIn);
    }
    let rate_limit = RateLimit::from_headers(response.headers());
    let data = response
        .json::<SduiResponse<Vec<Time>>>()
        .await
        .map_err(SduiError::RequestError)?;
    Ok((data.data, rate_limit))
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Time {
    pub begins_at: u64,
    pub description: Option<String>,
    pub ends_at: u64,
    pub hour: u8,
    pub id: u64,
    pub is_hidden: bool,
    pub meta: TimeMeta,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: TimeKind,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum TimeKind {
    BREAK,
    LESSON,
}
impl Serialize for TimeKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TimeKind::BREAK => serializer.serialize_str("BREAK"),
            TimeKind::LESSON => serializer.serialize_str("LESSON"),
        }
    }
}

impl<'de> Deserialize<'de> for TimeKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TimeKindVisitor;

        impl<'de> Visitor<'de> for TimeKindVisitor {
            type Value = TimeKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Expected BREAK,LESSON")
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    "BREAK" => Ok(TimeKind::BREAK),
                    "LESSON" => Ok(TimeKind::LESSON),
                    _ => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(v),
                        &self,
                    )),
                }
            }
        }
        deserializer.deserialize_str(TimeKindVisitor {})
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TimeMeta {
    pub displayname: String,
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
    pub kind: LessonKind,
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

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum LessonKind {
    NORMAL,
    SUBSTITUTION,
    CANCLED,
    ADDITIONAL,
}

impl Serialize for LessonKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            LessonKind::NORMAL => serializer.serialize_none(),
            LessonKind::SUBSTITUTION => serializer.serialize_str("SUBSTITUTION"),
            LessonKind::CANCLED => serializer.serialize_str("CANCLED"),
            LessonKind::ADDITIONAL => serializer.serialize_str("ADDITIONAL"),
        }
    }
}
impl<'de> Deserialize<'de> for LessonKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct LessonKindVisitor;

        impl<'de> Visitor<'de> for LessonKindVisitor {
            type Value = LessonKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Expected none,SUBSTITUTION,CANCLED or ADDITIONAL")
            }
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(LessonKind::NORMAL)
            }
            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct LessonKindSomeVisitor;

                impl<'de> Visitor<'de> for LessonKindSomeVisitor {
                    type Value = LessonKind;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("Expected none,SUBSTITUTION,CANCLED or ADDITIONAL")
                    }
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "SUBSTITUTION" => Ok(LessonKind::SUBSTITUTION),
                            "CANCLED" => Ok(LessonKind::CANCLED),
                            "ADDITIONAL" => Ok(LessonKind::ADDITIONAL),
                            _ => Err(serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(v),
                                &self,
                            )),
                        }
                    }
                }
                deserializer.deserialize_str(LessonKindSomeVisitor)
            }
        }
        deserializer.deserialize_option(LessonKindVisitor {})
    }
}
