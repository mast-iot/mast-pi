use std::sync::MutexGuard;

use diesel::{ExpressionMethods, QueryDsl};

use crate::devices::Device;
use crate::gpio::Io;
use crate::models::{Output, Param};

pub struct PowerSwitch;

impl Device for PowerSwitch {
    fn handle(pm: &Param, _: &String) {
        use crate::schema::output::dsl::output;
        if pm.out_id.is_some() {
            let output_id = pm.out_id.unwrap();
            use crate::schema::output::id as op_id;
            let op: Output = output.filter(op_id.eq(output_id)).get_result::<Output>(&conn.0).expect("");
            let v = pm.value.parse::<i32>().unwrap() ^ 1;
            let mut io: MutexGuard<Io> = GPIO.lock().unwrap();
            io.output_and_flash(op.address as u8, v as u8);
        }
    }
}
