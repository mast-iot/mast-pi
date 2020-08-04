use chrono::{Duration, Utc};
use diesel::*;
use rocket_contrib::json::{Json, JsonValue};

use crate::auth::Auth;
use crate::Conn;
use crate::constant::success;
use crate::models::User;
use crate::config::RequestError;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    mobile: String,
    password: String,
}

#[post("/login", format = "json", data = "<request>")]
pub fn login(
    request: Json<LoginRequest>,
    conn: Conn,
) -> Result<JsonValue, RequestError> {
    use crate::schema::user::dsl::*;
    let user_result: User = user.filter(mobile.eq(&request.mobile)).get_result(&conn.0).expect("error");
    let u = user_result.clone();
    if request.password == user_result.password {
        let token = Auth {
            id: user_result.id,
            mobile: user_result.mobile,
            name: user_result.name,
            exp: (Utc::now() + Duration::days(1000)).timestamp(),
        }.token();
        Ok(json!(success(u.auth_view(&token))))
    } else {
        Err(RequestError::auth_failed())
    }
}
