use rocket_contrib::json::JsonValue;
use crate::models::{Device, Room, Group};
use crate::constant::success;
use diesel::*;
use crate::Conn;
use crate::view::DeviceView;


#[get("/device/list")]
pub fn list_device(conn: Conn) -> JsonValue {
    use super::super::schema::device::dsl::*;
    use super::super::schema::room::dsl::*;
    use super::super::schema::group::dsl::*;
    use super::super::schema::room::id as table_room_key;
    use super::super::schema::group::id as table_group_key;
    let devices: Vec<DeviceView> = device
        .left_join(room.on(room_id.eq(table_room_key)))
        .left_join(group.on(group_id.eq(table_group_key.nullable())))
        .get_results::<(Device, Option<Room>, Option<Group>)>(&conn.0)
        .map(|list| {
            list.into_iter()
                .map(|(dev, rm, gp)| dev.attach(rm, gp)).collect()
        })
        .expect("error");
    json!(success(devices))
}

#[get("/room/list")]
pub fn list_room(conn: Conn) -> JsonValue {
    use super::super::schema::room::dsl::*;
    let rooms: Vec<Room> = room.load::<Room>(&conn.0).expect("error");
    json!(success(rooms))
}
