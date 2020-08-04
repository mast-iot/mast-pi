use diesel::*;
use local_ipaddress;
use rocket_contrib::json::JsonValue;

use crate::Conn;
use crate::constant::success;
use crate::models::{Device, Group, Param, Room};
use crate::view::{DeviceView};
use crate::auth::IpHolder;

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
        .get_results::<(Device, Option<Room>, Option<Group>)>(&conn.0).expect("error")
        .into_iter().map(|(dev, rm, gp)| dev.view(rm, gp)).collect::<Vec<_>>();
    json!(success(devices))
}

#[get("/room/list")]
pub fn list_room(conn: Conn) -> JsonValue {
    use super::super::schema::room::dsl::*;
    let rooms: Vec<Room> = room.load::<Room>(&conn.0).expect("error");
    let devices: Vec<Vec<Device>> = Device::belonging_to(&rooms).load::<Device>(&conn.0).expect("").grouped_by(&rooms);
    let result = rooms.into_iter().zip(devices)
        .map(|(rm, devices)| rm.view(devices))
        .collect::<Vec<_>>();
    json!(success(result))
}

#[get("/group/list")]
pub fn list_group(conn: Conn) -> JsonValue {
    use super::super::schema::group::dsl::*;
    let groups = group.load::<Group>(&conn.0).expect("error");
    let devices: Vec<Vec<Device>> = Device::belonging_to(&groups).load::<Device>(&conn.0).expect("error").grouped_by(&groups);
    let result = groups.into_iter().zip(devices)
        .map(|(gp, devices)| gp.view(devices)).collect::<Vec<_>>();
    json!(success(result))
}

#[get("/param/<dev_id>")]
pub fn list_param(dev_id: i32, conn: Conn) -> JsonValue {
    use super::super::schema::param::dsl::*;
    let params: Vec<Param> = param.filter(device_id.eq(dev_id)).get_results::<Param>(&conn.0).expect("error");
    json!(success(params))
}

#[get("/ping")]
pub fn get_server_ip() -> JsonValue {
    let ip = local_ipaddress::get().unwrap();
    json!(success(ip))
}

#[get("/pong")]
pub fn get_client_ip(
    ip_holder: IpHolder
) -> JsonValue {
    json!(ip_holder.ip)
}