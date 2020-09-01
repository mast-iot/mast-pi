use std::sync::{Arc, Mutex, RwLock};

use rppal::gpio::*;

const DS: u8 = 2;
//to pin14
const SHCP: u8 = 3;
// to pin11
const STCP: u8 = 4; //to pin12

static mut GPIO: Option<Arc<Mutex<GpioInstance>>> = None;
static mut IO: Option<Arc<Mutex<&[u8]>>> = None;


pub struct GpioInstance {}


pub fn get_instance() -> &'static mut Arc<Mutex<GpioInstance>> {
    unsafe {
        return GPIO.get_or_insert(Arc::new(Mutex::new(GpioInstance {})))
    }
}

pub fn flash() -> Result<()> {
    let a = get_instance().lock();
    let gpio = Gpio::new()?;
    let ds_pin = gpio.get(DS)?.into_output();
    let st_pin = gpio.get(STCP)?.into_output();
    let sh_pin = gpio.get(SHCP)?.into_output();

    Ok(())
}

fn test() {
    let io: GpioInstance = GpioInstance {};
}
