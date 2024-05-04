use chrono::{Datelike, FixedOffset, NaiveDate, TimeZone, Utc};
use core::panic;
use time::{Date, Month};

/// return {day}/{month}/{year}
pub fn today() -> String {
    let (y, m, d) = today_ymd();
    format!("{d}/{m}/{y}")
}

/// return (year, month, day)
pub fn today_ymd() -> (i32, u32, u32) {
    let now = FixedOffset::east_opt(7 * 3600)
        .expect("valid tz offset")
        .from_utc_datetime(&Utc::now().naive_utc());
    (now.year(), now.month(), now.day())
}

pub fn today_naive_date() -> NaiveDate {
    let (y, m, d) = today_ymd();
    NaiveDate::from_ymd_opt(y, m, d).expect("valid year, month, day")
}

pub fn today_date() -> Date {
    let (y, m, d) = today_ymd();
    let month = match m {
        1 => Month::January,
        2 => Month::February,
        3 => Month::March,
        4 => Month::April,
        5 => Month::May,
        6 => Month::June,
        7 => Month::July,
        8 => Month::August,
        9 => Month::September,
        10 => Month::October,
        11 => Month::November,
        12 => Month::December,
        _ => panic!("invalid date"),
    };
    Date::from_calendar_date(y, month, d as u8).expect("valid date range")
}
