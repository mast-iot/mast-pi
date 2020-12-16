use diesel::SqliteConnection;

use crate::Conn;
use crate::models::Param;

pub mod power;

pub trait Device {
    fn handle(param: &Param, value: &String, conn: &SqliteConnection);
}

