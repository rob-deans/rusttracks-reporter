use std::{env, process, format, time::Duration};
use futures::{executor::block_on, stream::StreamExt};
use serde::{Deserialize, Serialize};
// use serde_json::Result;
use rusqlite::{params, Connection, Result};

extern crate paho_mqtt as mqtt;

const MQTT_CLIENT_ID: &str = "rust_async_subscribe";
const TOPIC: &str = "owntracks/hass/rob";
const QOS: i32 = 1;

#[derive(Deserialize, Serialize, Debug)]
struct LocationPayload {
    _type: String,
    acc: u32,
    alt: u32,
    batt: u8,
    bs: u8,
    conn: char,
    created_at: u32,
    lat: f64,
    lon: f64,
    m: u8,
    tid: String,
    tst: u32,
    vac: u32,
    vel: u16
}

fn handle_payload_msg(conn: &Connection, msg: mqtt::Message) {
    let payload: LocationPayload = serde_json::from_str(&msg.payload_str()).unwrap_or_else(|err| {
        println!("Error parsing payload string: {:?}", err);
        process::exit(1);
    });
    println!("{}", msg.payload_str());
    if let Err(err) = conn.execute(
        "INSERT INTO location (tst, lat, lon, acc, alt, vac, batt, tid, vel, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![payload.tst, payload.lat, payload.lon, payload.acc, payload.alt, payload.vac, payload.batt, payload.tid, payload.vel, payload.created_at]
    ){
        println!("{}", err)
    };
}


fn main() -> Result<()> {

    let mqtt_url: String = env::var("MQTT_URL").unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    let mqtt_port = match env::var("MQTT_PORT") {
        Ok(val) => val,
        Err(_e) => "1883".to_string()
    };
    let mqtt_username: String = env::var("MQTT_USERNAME").unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    let mqtt_password: String = env::var("MQTT_PASSWORD").unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let conn = Connection::open_in_memory()?;


    conn.execute(
        "CREATE TABLE location (
                  tst INTEGER PRIMARY KEY,
                  lat            TEXT NOT NULL,
                  lon BLOB,
                  acc INTEGER,
                  alt INTEGER,
                  vac INTEGER,
                  batt INTEGER,
                  tid TEXT NOT NULL,
                  vel INTEGER,
                  created_at INTEGER
                  )",
        [],
    )?;

    let host = format!("tcp://{mqtt_url}:{mqtt_port}");
    println!("{}", host);

     // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(&host)
        .client_id(MQTT_CLIENT_ID)
        .finalize();

    // Create a client & define connect options
    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    if let Err(err) = block_on(async {
        // Get message stream before connecting.
        let mut strm = cli.get_stream(25);

        // Define the set of options for the connection
        let lwt = mqtt::Message::new("test", "Async subscriber lost connection", mqtt::QOS_1);

        let conn_opts = mqtt::ConnectOptionsBuilder::new()
            .keep_alive_interval(Duration::from_secs(30))
            .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
            .clean_session(false)
            .will_message(lwt)
            .user_name(mqtt_username)
            .password(mqtt_password)
            .finalize();

        // Make the connection to the broker
        println!("Connecting to the MQTT server...");
        cli.connect(conn_opts).await?;

        println!("Subscribing to topics: {:?}", TOPIC);
        cli.subscribe(TOPIC, QOS).await?;

        // Just loop on incoming messages.
        println!("Waiting for messages...");

        // Note that we're not providing a way to cleanly shut down and
        // disconnect. Therefore, when you kill this app (with a ^C or
        // whatever) the server will get an unexpected drop and then
        // should emit the LWT message.

        while let Some(msg_opt) = strm.next().await {
            if let Some(msg) = msg_opt {
                handle_payload_msg(&conn, msg)
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
    }) {
        eprintln!("{}", err);
    }

    Ok(())

}