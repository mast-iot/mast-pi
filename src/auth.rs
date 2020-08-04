use diesel::*;
use jsonwebtoken as jwt;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use rocket::{Outcome, Request, Response, response};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::response::Responder;
use rocket::response::status::Custom;

use crate::Conn;
use crate::models::User;

pub const SECRET: &'static str = "1U3ILPrtYK8dHKtOGVQdq1QdJqTwr5QM";

#[derive(Serialize, Deserialize)]
pub struct Auth {
    pub id: i32,
    pub mobile: String,
    pub name: String,
    pub exp: i64,
}

impl Auth {
    pub fn token(&self) -> String {
        jwt::encode(&jwt::Header::default(), self, &EncodingKey::from_secret(SECRET.as_bytes())).expect("error")
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Auth, (Status, Self::Error), ()> {
        let token = request.headers().get_one("authorization");
        let opt_auth = token.and_then(|token| decode_token(token));
        if let Some(auth) = opt_auth {
            Outcome::Success(auth)
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn decode_token(token: &str) -> Option<Auth> {
    jwt::decode(token, &DecodingKey::from_secret(SECRET.as_bytes()), &Validation::new(Algorithm::HS256))
        .map_err(|err| {
            eprintln!("Auth decode error: {:?}", err);
        })
        .ok()
        .map(|data| data.claims)
}

#[derive(Debug)]
pub struct RequestError {
    code: i16,
    msg: String,
}

impl<'r> Responder<'r> for RequestError {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        let response = crate::constant::Response {
            code: self.code,
            msg: self.msg,
            data: (),
        };
        Custom(
            Status::Ok,
            json!(response),
        ).respond_to(request)
    }
}


#[derive(Serialize, Deserialize)]
pub struct PassRequired {
    pub  password: String
}

impl PassRequired {
    pub fn validate(self, conn: &Conn, mobile: &String) -> Result<(), RequestError> {
        use crate::schema::user::dsl::*;
        let u: User = user.filter(mobile.eq(mobile)).get_result(&conn.0).expect("error");
        if u.password == self.password {
            Ok(())
        } else {
            Err(RequestError { code: 403, msg: "密码错误".to_string() })
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for PassRequired {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let local_network = request.headers().get_one("X-Local-Network").unwrap_or("0");
        let password = request.headers().get_one("X-Password").unwrap_or("");
        if local_network == "0" && password == "" {
            Outcome::Failure((Status::Forbidden, ()))
        } else {
            Outcome::Success(PassRequired {
                password: password.to_string()
            })
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IpHolder<'a> {
    pub ip: &'a str
}

impl<'a, 'r> FromRequest<'a, 'r> for IpHolder<'a> {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let ip = request.headers().get_one("X-Real-IP").unwrap_or("0.0.0.0");
        Outcome::Success(IpHolder { ip })
    }
}