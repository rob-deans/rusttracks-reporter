extern crate openssl;
#[macro_use] extern crate diesel;
extern crate diesel_migrations;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;

pub mod routes;
// pub mod utils;
// pub mod schema;
// pub mod models;

use std::{env, process, format, time::Duration};
use rocket::figment::{value::{Map, Value}, util::map};
use rocket::fs::{FileServer, relative};
use self::diesel::prelude::*;
use rocket::serde::{json::Json};
use rusttracks_contrib::models::LocationPayload;
use rusttracks_contrib::schema;

#[database("owntracks")]
pub struct OwntracksDB(diesel::SqliteConnection);

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let database_url = env::var("SQLITE_DB_URL").expect("SQLITE_DB_URL must be set");

    let db: Map<_, Value> = map! {
        "url" => database_url.into(),
        "pool_size" => 10.into(),
        "timeout" => 5.into(),
    };

    let figment = rocket::Config::figment()
        .merge(("databases", map!["owntracks" => db]));


    let rocket = rocket::custom(figment)
        .attach(OwntracksDB::fairing())
        // .mount("/", routes![routes::hello])
        .mount("/", FileServer::from(relative!("static")))
        .mount("/api", routes![routes::get_locations])
        .launch()
        .await?;

    Ok(())

}
