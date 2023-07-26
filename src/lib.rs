extern crate chrono;
use chrono::{NaiveDateTime, Utc, TimeZone};

pub fn parse_unix_timestamp(unix_timestamp: i64) -> String {
    let datetime = NaiveDateTime::from_timestamp(unix_timestamp, 0);
    let utc_datetime = Utc.from_utc_datetime(&datetime);
    utc_datetime.to_string()
}
