use diesel::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::Conn;
use crate::models::Param;

use super::super::constant::success;
use super::super::schema::param::dsl::*;

#[derive(Deserialize)]
pub struct ParamUpdate {
    id: i32,
    value: String,
}

#[post("/param/update", format = "json", data = "<param_update>")]
pub fn update_param(
    param_update: Json<ParamUpdate>,
    conn: Conn,
) -> JsonValue {
    let target = param.filter(id.eq(&param_update.id));
    let origin: Param = target.get_result::<Param>(&conn.0).expect("record not found");
    let opts: Vec<String> = origin.options.split(",").map(|s| s.to_string()).collect();
    if opts.contains(&param_update.value) {
        diesel::update(target).set(value.eq(&param_update.value)).execute(&conn.0);
        let response = success(String::from(""));
        json!(response)
    } else {
        panic!("value not support!!")
    }
}