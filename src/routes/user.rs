use chrono::{Duration, Utc};
use diesel::*;
use rocket_contrib::json::{Json, JsonValue};

use crate::auth::Auth;
use crate::Conn;
use crate::constant::success;
use crate::models::User;
use crate::config::{RequestError, error_500};

#[derive(Deserialize, Clone)]
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
    let user_result: User = user.filter(mobile.eq(request.mobile.clone()))
        .get_result::<User>(&conn.0).map_err(|_err| error_500("手机号不存在"))?;
    let u = user_result.clone();
    if request.password.to_lowercase() == user_result.password.to_lowercase() {
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
