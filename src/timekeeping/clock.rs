#[cfg(windows)]
use kernel32;
#[cfg(not(windows))]
use libc;
use std::mem::zeroed;
#[cfg(windows)]
use winapi;

use chrono::{DateTime, Datelike, Local, TimeZone, Timelike};
pub struct Clock;

impl Clock {
    pub fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    pub fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);

        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        let mut ns = t.nanosecond();
        let is_leap_second = ns > 1_000_000_000;

        if is_leap_second {
            ns -= 1_000_000_000;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = t.second() as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME;

        unsafe {
            SetSystemTime(systime_ptr);
        }
    }
}
