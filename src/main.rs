#![feature(proc_macro_hygiene, decl_macro)]
extern crate mast;


fn main() {
    mast::rocket().launch();
}
