use chrono::{Duration, Utc};
use diesel::*;
use rocket_contrib::json::{Json, JsonValue};

use crate::auth::Auth;
use crate::auth::SECRET;
use crate::Conn;
use crate::constant::success;
use crate::models::User;

#[derive(Deserialize)]
pub struct LoginRequest {
    mobile: String,
    password: String,
}

#[post("/login", format = "json", data = "<request>")]
pub fn login(
    request: Json<LoginRequest>,
    conn: Conn,
) -> JsonValue {
    use crate::schema::user::dsl::*;
    let user_result: User = user.filter(mobile.eq(&request.mobile)).get_result(&conn.0).expect("error");
    let u = user_result.clone();
    let token = Auth {
        id: user_result.id,
        mobile: user_result.mobile,
        name: user_result.name,
        exp: (Utc::now() + Duration::days(1000)).timestamp(),
    }.token();
    json!(success(u.auth_view(&token)))
}
