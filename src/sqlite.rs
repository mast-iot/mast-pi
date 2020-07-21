extern crate dotenv;
extern crate diesel;

use diesel::sqlite::SqliteConnection;
use self::diesel::Connection;


pub fn create_connection() -> SqliteConnection {
    let url = "/data/sqlite/mast.db";
    SqliteConnection::establish(url).unwrap()
}