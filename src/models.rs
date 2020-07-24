use crate::view::{DeviceView, RoomView, GroupView};
use crate::schema::*;

#[derive(Identifiable, Queryable, Debug, Serialize, Associations)]
#[belongs_to(Room)]
#[belongs_to(Group)]
#[table_name = "device"]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub desc: Option<String>,
    pub device_type: String,
    pub icon: Option<String>,
    pub room_id: i32,
    pub group_id: Option<i32>,
}

#[derive(Identifiable, Queryable, Serialize)]
#[table_name = "room"]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub desc: Option<String>,
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "group"]
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "param"]
pub struct Param {
    pub id: i32,
    pub param_type: String,
    pub key: String,
    pub desc: Option<String>,
    pub options: String,
    pub value: String,
    pub usage: String,
    pub device_id: i32,
}


impl Device {
    pub fn view(self, room: Option<Room>, group: Option<Group>) -> DeviceView {
        DeviceView {
            id: self.id,
            name: self.name,
            desc: self.desc,
            device_type: self.device_type,
            icon: self.icon,
            room,
            group,
        }
    }
}

impl Room {
    pub fn view(self, devices: Vec<Device>) -> RoomView {
        RoomView {
            id: self.id,
            name: self.name,
            desc: self.desc,
            devices,
        }
    }
}

impl Group {
    pub fn view(self, devices: Vec<Device>) -> GroupView {
        GroupView {
            id: self.id,
            name: self.name,
            devices,
        }
    }
}