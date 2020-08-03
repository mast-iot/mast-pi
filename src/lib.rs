#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use crate::constant::Response;
use rocket_contrib::json::JsonValue;


pub mod config;
pub mod sqlite;
pub mod schema;
pub mod models;
pub mod routes;
pub mod constant;
pub mod devices;
pub mod view;

#[catch(404)]
fn not_found() -> JsonValue {
    json!(Response{
        code: 404,
        msg: "".to_string(),
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
             routes::query::list_param,
             routes::query::list_group,
        ])
        .attach(Conn::fairing())
        .register(catchers![not_found])
}
