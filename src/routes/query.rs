use diesel::*;
use local_ipaddress;
use rocket_contrib::json::JsonValue;

use crate::auth::{Auth, IpHolder};
use crate::Conn;
use crate::constant::success;
use crate::models::{Device, Group, Param, Room, RemarkableDevice};
use crate::view::{DeviceView, RemarkableDeviceView};
use serde_json::Value;

#[get("/device/list")]
pub fn list_device(
    conn: Conn,
    _auth: Auth,
) -> JsonValue {
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

#[get("/remarkable/list")]
pub fn remarkable_device_list(
    conn: Conn
) -> JsonValue {
    use super::super::schema::remarkable_device::dsl::*;
    use super::super::schema::param::dsl::*;
    use super::super::schema::device::dsl::*;
    use super::super::schema::param::id as table_param_id;
    use super::super::schema::device::id as table_device_id;
    let remark: Vec<RemarkableDeviceView> = remarkable_device
        .left_join(param.on(param_id.eq(table_param_id)))
        .left_join(device.on(device_id.eq(table_device_id)))
        .get_results::<(RemarkableDevice, Option<Param>, Option<Device>)>(&conn.0).expect("error")
        .into_iter().map(|(rd, pm, dev)| rd.view(pm, dev)).collect::<Vec<_>>();
    json!(remark)
}

#[get("/room/list")]
pub fn list_room(
    conn: Conn,
    _auth: Auth,
) -> JsonValue {
    use super::super::schema::room::dsl::*;
    let rooms: Vec<Room> = room.load::<Room>(&conn.0).expect("error");
    let devices: Vec<Vec<Device>> = Device::belonging_to(&rooms).load::<Device>(&conn.0).expect("").grouped_by(&rooms);
    let result = rooms.into_iter().zip(devices)
        .map(|(rm, devices)| rm.view(devices))
        .collect::<Vec<_>>();
    json!(success(result))
}

#[get("/group/list")]
pub fn list_group(
    conn: Conn,
    _auth: Auth,
) -> JsonValue {
    use super::super::schema::group::dsl::*;
    let groups = group.load::<Group>(&conn.0).expect("error");
    let devices: Vec<Vec<Device>> = Device::belonging_to(&groups).load::<Device>(&conn.0).expect("error").grouped_by(&groups);
    let result = groups.into_iter().zip(devices)
        .map(|(gp, devices)| gp.view(devices)).collect::<Vec<_>>();
    json!(success(result))
}

#[get("/param/<dev_id>")]
pub fn list_param(
    dev_id: i32,
    conn: Conn,
    _auth: Auth,
) -> JsonValue {
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

#[get("/weather")]
pub fn weather() -> JsonValue {
    use easy_http_request::DefaultHttpRequest;
    // let response = DefaultHttpRequest::get_from_url_str("https://api.openweathermap.org/data/2.5/onecall?lat=30.287716668804674&lon=120.06577596450808\
    // &lang=zh_cn&units=metric&appid=2a49e2d2ff8a324e28c6f717685f55e3")
    //     .unwrap().send().unwrap();
    let json = include_str!("../../assets/weather.json");
    let result = serde_json::from_str(json);
    // let result = serde_json::from_str(String::from_utf8(response.body).unwrap().as_str());
    let fin: Value = result.unwrap();
    json!(fin)
}
