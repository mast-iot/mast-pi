use crate::models::{Device, Group, Room, Param};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceView {
    pub id: i32,
    pub name: String,
    pub desc: Option<String>,
    pub device_type: String,
    pub icon: Option<String>,
    pub room: Option<Room>,
    pub group: Option<Group>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomView {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub desc: Option<String>,
    pub devices: Vec<Device>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupView {
    pub id: i32,
    pub name: String,
    pub devices: Vec<Device>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAuthView {
    pub id: i32,
    pub name: String,
    pub mobile: String,
    pub image: Option<String>,
    pub token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemarkableDeviceView {
    pub device: Option<DeviceView>,
    pub param: Option<Param>,
    pub sort: i32,
}
