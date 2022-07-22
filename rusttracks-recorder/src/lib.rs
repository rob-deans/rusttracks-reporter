#[macro_use] extern crate diesel;
extern crate paho_mqtt as mqtt;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::{process,time::Duration};
use futures::{stream::StreamExt};

use rusttracks_contrib::models::NewLocationPayload;
use rusttracks_contrib::schema::location;


pub fn establish_connection(database_url: &String) -> SqliteConnection {

    // let database_url = env::var("SQLITE_DB_URL").expect("SQLITE_DB_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_payload(conn: &SqliteConnection, payload_msg: mqtt::Message) {
    let payload: NewLocationPayload = serde_json::from_str(&payload_msg.payload_str()).unwrap_or_else(|err| {
        println!("Error parsing payload string: {:?}", err);
        process::exit(1);
    });
    if let Err(err) = diesel::insert_into(location::table)
        .values(&payload)
        .execute(conn) {
            println!("{}", err);
        };
        
}


pub async fn listen(cli: mqtt::AsyncClient, mut strm: mqtt::AsyncReceiver<Option<mqtt::Message>>, conn: SqliteConnection) -> Result<(), mqtt::Error> {
    println!("Waiting for messages...");

    // Note that we're not providing a way to cleanly shut down and
    // disconnect. Therefore, when you kill this app (with a ^C or
    // whatever) the server will get an unexpected drop and then
    // should emit the LWT message.

    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            insert_payload(&conn, msg);
        }
        else {
            // A "None" means we were disconnected. Try to reconnect...
            println!("Lost connection. Attempting reconnect.");
            while let Err(err) = cli.reconnect().await {
                println!("Error reconnecting: {}", err);
                // For tokio use: tokio::time::delay_for()
                async_std::task::sleep(Duration::from_millis(1000)).await;
            }
        }
    }

    // Explicit return type for the async block
    Ok::<(), mqtt::Error>(())
}