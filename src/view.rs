use crate::models::{Device, Group, Room};

#[derive(Serialize)]
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
pub struct RoomView {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub desc: Option<String>,
    pub devices: Vec<Device>,
}

#[derive(Serialize)]
pub struct GroupView {
    pub id: i32,
    pub name: String,
    pub devices: Vec<Device>,
}