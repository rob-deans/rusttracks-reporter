extern crate diesel;

pub use self::diesel::prelude::*;
pub use rocket::serde::{json::Json};

use crate::OwntracksDB;
use rusttracks_contrib::models::LocationPayload;
use rusttracks_contrib::schema;
use rusttracks_contrib::utils::date_to_unixts;

#[get("/locations?<start_date>&<end_date>")]
pub async fn get_locations(db: OwntracksDB, start_date: Option<String>, end_date: Option<String>) ->  Json<Vec<LocationPayload>> {

    let unwrapped_start_date = if let Some(d) = start_date { d } else { "1970-01-01 00:00:00".to_string() };
    let unwrapped_end_date = if let Some(d) = end_date { d } else { "2030-01-01 00:00:00".to_string() };

    let locations = db.run(|c| {
        let start_date = date_to_unixts(unwrapped_start_date).unwrap();
        let end_date = date_to_unixts(unwrapped_end_date).unwrap();
        let locations = schema::location::table
            .filter(schema::location::tst.ge(start_date))
            .filter(schema::location::tst.le(end_date))
            .load::<LocationPayload>(&*c);
        locations
    }).await;

    match locations {
        Ok(r) => Json(r),
        Err(_) => Json(vec![])
    }
}