extern crate paho_mqtt as mqtt;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use futures::stream::StreamExt;
use log::{error, info};
use std::time::Duration;

use rusttracks_contrib::models::NewLocationPayload;
use rusttracks_contrib::schema::location;

pub fn establish_connection(database_url: &String) -> SqliteConnection {
    SqliteConnection::establish(&database_url).expect("Error connecting to {database_url}")
}

pub fn insert_payload(conn: &SqliteConnection, payload_msg: mqtt::Message) {
    match serde_json::from_str::<NewLocationPayload>(&payload_msg.payload_str()) {
        Ok(payload) => {
            if let Err(err) = diesel::insert_into(location::table)
                .values(&payload)
                .execute(conn)
            {
                error!("{}", err);
            };
        }
        Err(e) => error!("{}", e),
    }
}

pub async fn listen(
    cli: mqtt::AsyncClient,
    mut strm: mqtt::AsyncReceiver<Option<mqtt::Message>>,
    conn: SqliteConnection,
) -> Result<(), mqtt::Error> {
    info!("Waiting for messages...");

    // Note that we're not providing a way to cleanly shut down and
    // disconnect. Therefore, when you kill this app (with a ^C or
    // whatever) the server will get an unexpected drop and then
    // should emit the LWT message.

    while let Some(msg_opt) = strm.next().await {
        if let Some(msg) = msg_opt {
            insert_payload(&conn, msg);
        } else {
            // A "None" means we were disconnected. Try to reconnect...
            info!("Lost connection. Attempting reconnect.");
            while let Err(err) = cli.reconnect().await {
                error!("Error reconnecting: {}", err);
                // For tokio use: tokio::time::delay_for()
                async_std::task::sleep(Duration::from_millis(1000)).await;
            }
        }
    }

    // Explicit return type for the async block
    Ok::<(), mqtt::Error>(())
}
