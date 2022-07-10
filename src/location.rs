use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LocationPayload {
    pub _type: String,
    pub acc: u32,
    pub alt: u32,
    pub batt: u8,
    pub bs: u8,
    pub conn: char,
    pub created_at: u32,
    pub lat: f64,
    pub lon: f64,
    pub m: u8,
    pub tid: String,
    pub tst: u32,
    pub vac: u32,
    pub vel: u16
}