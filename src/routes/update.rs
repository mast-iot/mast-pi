use diesel::*;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

use crate::auth::{Auth, PassRequired};
use crate::config::RequestError;
use crate::Conn;
use crate::constant::success;
use crate::models::{Output, Param};
use crate::schema::output::dsl::*;
use crate::schema::param::dsl::*;
use crate::gpio::{GPIO, Io};
use std::sync::MutexGuard;

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
    // password.validate(&conn, &auth.mobile)?;
    let target = param.filter(crate::schema::param::id.eq(&param_update.id));
    let result: Result<Param, _> = target.get_result::<Param>(&conn.0);
    if let Ok(pm) = result {
        if pm.out_id.is_some() {
            let output_id = pm.out_id.unwrap();
            use crate::schema::output::id as op_id;
            let op: Output = output.filter(op_id.eq(output_id)).get_result::<Output>(&conn.0).expect("");
            let v = pm.value.parse::<i32>().unwrap();
            if op.state == v { return Err(RequestError::parameter_error()); } else {
                let mut io: MutexGuard<Io> = GPIO.lock().unwrap();
                io.output_and_flash(op.address as u8, v as u8);
            }
        }
        Err(RequestError::internal_error())
    } else {
        Err(RequestError::record_not_found())
    }
}
