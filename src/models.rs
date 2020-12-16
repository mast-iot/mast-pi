use crate::schema::*;
use crate::view::{DeviceView, GroupView, RemarkableDeviceView, RoomView, UserAuthView};

#[derive(Identifiable, Queryable, Debug, Serialize, Associations)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub desc: Option<String>,
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "group"]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[table_name = "param"]
#[serde(rename_all = "camelCase")]
pub struct Param {
    pub id: i32,
    pub param_type: String,
    pub value_type: String,
    pub key: String,
    pub desc: Option<String>,
    pub options: String,
    pub value: String,
    pub usage: String,
    pub device_id: i32,
    pub in_id: Option<i32>,
    pub out_id: Option<i32>,
}

#[derive(Queryable, Serialize, Identifiable, Associations)]
#[table_name = "remarkable_device"]
#[serde(rename_all = "camelCase")]
pub struct RemarkableDevice {
    pub id: i32,
    pub sort: i32,
    pub param_id: i32,
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "input"]
#[serde(rename_all = "camelCase")]
pub struct Input {
    pub id: u8,
    pub address: u8,
    pub state: u8,
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "output"]
#[serde(rename_all = "camelCase")]
pub struct Output {
    pub id: i32,
    pub address: i32,
    pub state: i32,
}

#[derive(Queryable, Serialize, Identifiable, Clone)]
#[table_name = "user"]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub mobile: String,
    pub image: Option<String>,
    pub password: String,
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
            image: self.image,
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

impl User {
    pub fn auth_view(self, token: &str) -> UserAuthView {
        UserAuthView {
            id: self.id,
            name: self.name,
            mobile: self.mobile,
            image: self.image,
            token: String::from(token),
        }
    }
}


impl RemarkableDevice {
    pub fn view(self, pm: Option<Param>, dev: Option<Device>) -> RemarkableDeviceView {
        let d = dev.map(|d| d.view(None, None));
        RemarkableDeviceView {
            device: d,
            param: pm,
            sort: self.sort,
        }
    }
}
