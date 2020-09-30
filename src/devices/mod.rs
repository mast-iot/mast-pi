use crate::models::Param;
use crate::Conn;
use diesel::SqliteConnection;

pub mod power;

pub trait Device {
    fn handle(param: &Param, value: &String, conn: &SqliteConnection);
}
