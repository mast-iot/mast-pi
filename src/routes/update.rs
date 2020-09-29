use std::sync::MutexGuard;

use diesel::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::auth::{Auth, PassRequired};
use crate::config::RequestError;
use crate::Conn;
use crate::constant::success;
use crate::devices::Device;
use crate::devices::power::PowerSwitch;
use crate::gpio::{GPIO, Io};
use crate::models::{Output, Param};
use crate::schema::output::dsl::*;
use crate::schema::param::dsl::*;

#[derive(Deserialize)]
pub struct ParamUpdate {
    id: i32,
    value: String,
}

#[post("/param/update", format = "json", data = "<param_update>")]
pub fn update_param(
    param_update: Json<ParamUpdate>,
    conn: Conn,
    //auth: Auth,
    //   password: PassRequired,
) -> Result<JsonValue, RequestError> {
    // password.validate(&conn, &auth.mobile)?;
    let target = param.filter(crate::schema::param::id.eq(&param_update.id));
    let result: Result<Param, _> = target.get_result::<Param>(&conn.0);
    if let Ok(pm) = result {
        if pm.out_id.is_some() {
            match pm.param_type.as_str() {
                "power" => { PowerSwitch::handle(&pm, &param_update.value) }
                _ => {}
            }
            Err(RequestError::success())
        } else {
            Err(RequestError::internal_error())
        }
    } else {
        Err(RequestError::record_not_found())
    }
}
