use crate::view::DeviceView;

#[derive(Queryable, Debug, Serialize)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub desc: Option<String>,
    pub device_type: String,
    pub icon: Option<String>,
    pub room_id: i32,
    pub group_id: Option<i32>,
}

#[derive(Queryable, Serialize)]
pub struct Room {
    pub id: i32,
    pub name: String,
    pub desc: Option<String>,
}

#[derive(Queryable, Serialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Serialize)]
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
    pub fn attach(self, room: Option<Room>, group: Option<Group>) -> DeviceView {
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