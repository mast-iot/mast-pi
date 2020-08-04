use diesel::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::Conn;
use crate::models::Param;

use super::super::constant::success;
use super::super::schema::param::dsl::*;
use crate::auth::{Auth, PassRequired};
use crate::config::RequestError;

#[derive(Deserialize)]
pub struct ParamUpdate {
    id: i32,
    value: String,
}

#[post("/param/update", format = "json", data = "<param_update>")]
pub fn update_param(
    param_update: Json<ParamUpdate>,
    conn: Conn,
    auth: Auth,
    password: PassRequired,
) -> Result<JsonValue, RequestError> {
    password.validate(&conn, &auth.mobile)?;
    let target = param.filter(id.eq(&param_update.id));
    let result: Result<Param, diesel::result::Error> = target.get_result::<Param>(&conn.0);
    if let Ok(origin) = result {
        let opts: Vec<String> = origin.options.split(",").map(|s| s.to_string()).collect();
        if opts.contains(&param_update.value) {
            let _result = diesel::update(target).set(value.eq(&param_update.value)).execute(&conn.0);
            let response = success(String::from(""));
            Ok(json!(response))
        } else {
            Err(RequestError::internal_error())
        }
    } else {
        Err(RequestError::record_not_found())
    }

}