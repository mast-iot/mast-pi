use std::sync::MutexGuard;
use std::sync::{Arc, Mutex};


use diesel::{ExpressionMethods, QueryDsl, SqliteConnection, RunQueryDsl};

use crate::devices::Device;
use crate::gpio::{GPIO, Io};
use crate::models::{Output, Param};

pub struct PowerSwitch;

impl Device for PowerSwitch {
    fn handle(pm: &Param, _: &String, conn: &SqliteConnection) {
        use crate::schema::output::dsl::*;
        if pm.out_id.is_some() {
            let output_id = pm.out_id.unwrap();
            use crate::schema::output::id as op_id;
            let op: Output = output.filter(op_id.eq(output_id)).get_result::<Output>(conn).expect("");
            let v = pm.value.parse::<i32>().unwrap() ^ 1;
            let mut io: MutexGuard<Io> = GPIO.lock().unwrap();
            io.output_and_flash(op.address as u8, v as u8);
        }
    }
}

lazy_static! {
    pub static ref POWER_LOCK: Arc<Mutex<u128>> = Arc::new(Mutex::new(0u128));
}



