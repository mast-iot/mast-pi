use crate::models::Param;

pub mod power;

pub trait  Device{
    fn handle(param:&Param , value :String);
}
