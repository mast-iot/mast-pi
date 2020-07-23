#![feature(proc_macro_hygiene, decl_macro)]
extern crate mast;

use rppal::gpio::Gpio;

fn main() {
    mast::rocket().launch();
}
