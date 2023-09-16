use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Deserializer, Serialize};
use iso8601_duration::Duration;

#[derive(Debug, Clone,
Serialize, Deserialize)]
pub struct DeleteWorklog {
    pub issue: String,
    pub worklog: i32,
}

#[derive(Debug, Clone,
Serialize, Deserialize)]
pub struct Datasets {
    pub datasets: Vec<Vec<Option<DataDurations>>>,
    pub labels: Vec<NaiveDate>,
    pub weekends: Vec<i8>,
    pub total_duration: i32
}

#[derive(Debug, Clone,
Serialize, Deserialize)]
pub struct DataDurations {
    pub duration: i32,
    pub worklog: i32,
    pub key: String,
    pub issue: String,
    pub title: String,
    pub background_colors: String
}

#[derive(Debug, Clone,
Serialize, Deserialize)]
pub struct WorklogQuery {
    pub date_from: NaiveDate,
    pub date_to: NaiveDate,
}

#[derive(Debug, Clone,
Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub display: String,
}

#[derive(Debug, Clone,
Serialize, Deserialize)]
pub struct Worklog {
    pub id: i32,
    #[serde(deserialize_with = "to_datetime")]
    pub start: NaiveDate,
    #[serde(deserialize_with = "to_duration")]
    pub duration: i32,
    pub issue: Issue,
}

fn to_datetime<'de, D: Deserializer<'de>>(d: D) -> Result<NaiveDate, D::Error> {
    let s: String = Deserialize::deserialize(d)?;
    DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.3f%z")
        .map_err(serde::de::Error::custom)
        .and_then(|r| Ok(r.date_naive()))
}

fn to_duration<'de, D: Deserializer<'de>>(d: D) -> Result<i32, D::Error> {
    let s: String = Deserialize::deserialize(d)?;
    s.parse::<Duration>()
        .map_err(|e| serde::de::Error::custom(e.input))
        .and_then(|r| r.num_seconds()
            .ok_or(serde::de::Error::custom("error".to_string())))
        .and_then(|r| Ok(r as i32))
}