extern crate openssl;
extern crate diesel;
extern crate diesel_migrations;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_sync_db_pools;

pub mod routes;

use std::env;
use rocket::figment::{value::{Map, Value}, util::map};
use rocket::fs::{FileServer, relative};

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


    let _ = rocket::custom(figment)
        .attach(OwntracksDB::fairing())
        .mount("/", FileServer::from(relative!("dist")))
        .mount("/api", routes![routes::get_locations])
        .launch()
        .await?;

    Ok(())

}
