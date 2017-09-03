
extern crate chrono;
#[macro_use]
extern crate try_opt;

use chrono::{Datelike, NaiveDate};

pub mod calendar_duration;
pub mod date_iterator;

pub use calendar_duration::{CalendarDuration, add, checked_add};

//from https://github.com/chronotope/chrono/issues/29

pub fn is_leap_year(year: i32) -> bool {
    NaiveDate::from_ymd_opt(year, 2, 29).is_some()
}

pub fn last_day_of_month_0(year: i32, month_0: u32) -> u32 {
    last_day_of_month(year, month_0 + 1)
}

pub fn last_day_of_month(year: i32, month: u32) -> u32 {
    NaiveDate::from_ymd_opt(year, month + 1, 1)
        .unwrap_or(NaiveDate::from_ymd(year + 1, 1, 1))
        .pred()
        .day()
}
