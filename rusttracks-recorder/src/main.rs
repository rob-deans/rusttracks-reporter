extern crate openssl;
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate paho_mqtt as mqtt;

use std::{env, process, format, time::Duration};
use futures::{executor::block_on};
use uuid::Uuid;

use rusttracks_recorder::{establish_connection, listen};

embed_migrations!();

const QOS: i32 = 1;

fn main() {

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
    let mqtt_client_id = match env::var("MQTT_CLIENT_ID") {
        Ok(val) => val,
        Err(_e) => Uuid::new_v4().to_string()
    };
    let topic: String = env::var("TOPIC").unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    let database_url = env::var("SQLITE_DB_URL").expect("SQLITE_DB_URL must be set");

    let conn = establish_connection(&database_url);

    if let Err(err) = embedded_migrations::run(&conn) {
        println!("{}", err);
        process::exit(1);
    };

    let host = format!("tcp://{mqtt_url}:{mqtt_port}");
    println!("Host URL: {host}");

    // Create the client. Use an ID for a persistent session.
    // A real system should try harder to use a unique ID.
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(&host)
        .client_id(mqtt_client_id)
        .finalize();

    let mut cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let strm = cli.get_stream(25);

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(30))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(false)
        .user_name(mqtt_username)
        .password(mqtt_password)
        .finalize();

    if let Err(err) = block_on(async {
        cli = setup_client(cli, conn_opts, topic).await;
        println!("{}", cli.is_connected());
        

        if let Err(err) = listen(cli, strm, conn).await {
            eprintln!("{}", err);
        };

        Ok::<(), mqtt::Error>(())

    }) {
        eprintln!("{}", err)
    }

}

pub async fn setup_client(cli: mqtt::AsyncClient, conn_opts: mqtt::ConnectOptions, topic: String) -> mqtt::AsyncClient {

    println!("Connecting to the MQTT server...");
    match cli.connect(conn_opts).await {
        Err(why) => panic!("{}", why),
        Ok(_) => println!("Connected.")
    };

    println!("Subscribing to topics: {:?}", topic);
    match cli.subscribe(topic, QOS).await {
        Err(why) => panic!("{}", why),
        Ok(_) => println!("Subbed to topic.")
    };

    cli

}
