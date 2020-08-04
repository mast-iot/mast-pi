use std::collections::HashMap;
use std::env;

use rocket::{Config, Request, response};
use rocket::config::{Environment, Value};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::response::status::Custom;

const DATABASE_URL: &'static str = "/data/sqlite/mast.db";


pub fn config() -> Config {
    let environment = Environment::active().expect("No environment found");
    let port = env::var("MAST_PORT").unwrap_or(String::from("8000")).parse::<u16>().expect("parse port error");
    let address = env::var("MAST_ADDRESS").unwrap_or("localhost".to_string());
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    database_config.insert("url", Value::from(DATABASE_URL));
    databases.insert("diesel_sqlite_pool", Value::from(database_config));

    Config::build(environment)
        .environment(environment)
        .port(port)
        .address(address)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}


#[derive(Debug)]
pub struct RequestError {
    pub   code: i16,
    pub   msg: String,
}

pub fn error_500(msg: &str) -> RequestError {
    RequestError {
        code: 500,
        msg: msg.to_string(),
    }
}

impl RequestError {
    pub fn internal_error() -> Self {
        error_500("内部错误")
    }

    pub fn record_not_found() -> Self {
        error_500("未找到记录")
    }

    pub fn parameter_error() -> Self {
        error_500("请求参数有误")
    }
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
