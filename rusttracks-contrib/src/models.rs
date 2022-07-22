use super::schema::location;
use serde::{Deserialize, Serialize};


#[derive(Insertable)]
#[derive(Deserialize, Serialize, Debug)]
#[table_name = "location"]
pub struct NewLocationPayload {
    pub tst: i64,
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acc: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vac: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batt: Option<i32>,
    pub tid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vel: Option<i32>,
    pub created_at: i32
}

#[derive(Queryable)]
#[derive(Deserialize, Serialize)]
pub struct LocationPayload {
    pub tst: i64,
    pub lat: f64,
    pub lon: f64,
    pub acc: Option<i32>,
    pub alt: Option<i32>,
    pub vac: Option<i32>,
    pub batt: Option<i32>,
    pub tid: String,
    pub vel: Option<i32>,
    pub created_at: i32
}