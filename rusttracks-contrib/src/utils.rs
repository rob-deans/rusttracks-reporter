use chrono::{NaiveDateTime};
use chrono::format::ParseError;

pub fn date_to_unixts(date: String) -> Result<i64, ParseError> {
    let datetime = NaiveDateTime::parse_from_str(&date, "%Y-%m-%dT%H:%M")?.timestamp();
    Ok(datetime)
}