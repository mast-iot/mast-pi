#![feature(proc_macro_hygiene, decl_macro)]
extern crate mast;

use rppal::gpio::Gpio;

fn main() {
    let mut pin = Gpio::new().expect("error").get(17).expect("error").into_output();
    pin.set_high();

    mast::rocket().launch();
}
