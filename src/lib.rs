#![feature(proc_macro_hygiene, decl_macro)]
#![feature(in_band_lifetimes)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;

use rocket_contrib::json::JsonValue;

use crate::constant::Response;

pub mod config;
pub mod sqlite;
pub mod schema;
pub mod models;
pub mod routes;
pub mod constant;
pub mod devices;
pub mod view;
pub mod auth;
pub mod gpio;

#[catch(404)]
fn not_found() -> JsonValue {
    json!(Response{
        code: 404,
        msg: "".to_string(),
        data: ()
    })
}

#[catch(403)]
fn forbidden() -> JsonValue {
    json!(Response{
        code: 403,
        msg: "forbidden".to_string(),
        data: ()
    })
}


#[database("diesel_sqlite_pool")]
pub struct Conn(diesel::SqliteConnection);

pub fn rocket() -> rocket::Rocket {
    rocket::custom(config::config())
        .mount("/api",
               routes![
             routes::query::list_device,
             routes::query::list_room,
             routes::update::update_param,
             routes::query::get_server_ip,
             routes::query::get_client_ip,
             routes::query::list_param,
             routes::query::list_group,
             routes::user::login,
             routes::query::weather,
        ])
        .attach(Conn::fairing())
        .register(catchers![not_found , forbidden])
}
