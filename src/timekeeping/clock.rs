use chrono::{DateTime, Local};

pub struct Clock;

impl Clock {
    pub fn get() -> DateTime<Local> {
        Local::now()
    }

    pub fn set() -> ! {
        unimplemented!()
    }
}
