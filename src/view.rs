use crate::models::{Room, Group};

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
