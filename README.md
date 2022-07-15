# rusttracks-reporter

*It's not written in Rust if I don't mention it*

A project to replicate https://github.com/owntracks/recorder but of course in Rust :)

This is an experiment for myself to learn something and have something useful for me.

Currently utilising MQTT and SQLite.

## Feature map
- [x] Record all MQTT messages to SQLite
- [ ] Set up API backend
- [ ] Set up frontend to plot locations on map


## Env vars

 - MQTT_URL
    - The URL for the MQTT instance.
 - MQTT_PORT
    - The port for the MQTT instance. Defaults to 1883.
 - MQTT_USERNAME
    - The username to login to the MQTT instance.
 - MQTT_PASSWORD
    - The password to login to the MQTT instance.
 - MQTT_CLIENT_ID
    - The client id of this session. (?). Defaults to `rust_async_subscribe`.
 - TOPIC
    - The topic of the user(s) that you want to store the location data for.