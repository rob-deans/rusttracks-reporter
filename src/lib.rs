#[macro_use]
extern crate diesel;
extern crate paho_mqtt as mqtt;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::{env,process};

use models::LocationPayload;
use schema::location;

pub fn establish_connection() -> SqliteConnection {

    let database_url = env::var("SQLITE_DB_URL").expect("SQLITE_DB_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_payload(conn: &SqliteConnection, payload_msg: mqtt::Message) {
    let payload: LocationPayload = serde_json::from_str(&payload_msg.payload_str()).unwrap_or_else(|err| {
        println!("Error parsing payload string: {:?}", err);
        process::exit(1);
    });
    if let Err(err) = diesel::insert_into(location::table)
        .values(&payload)
        .execute(conn) {
            println!("{}", err);
        };
        
}
// pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> usize {
//     use schema::posts;

//     let new_post = NewPost { title, body };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .execute(conn)
//         .expect("Error saving new post")
// }