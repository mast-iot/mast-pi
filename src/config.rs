use std::env;
use std::collections::HashMap;
use rocket::config::{Value, Environment};
use rocket::Config;


const DATABASE_URL: &'static str = "/data/sqlite/mast.db";


pub fn config() -> Config {
    let environment = Environment::active().expect("No environment found");
    let port = env::var("MAST_PORT").unwrap_or(String::from("8000")).parse::<u16>().expect("parse port error");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(DATABASE_URL));
    databases.insert("diesel_sqlite_pool", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}