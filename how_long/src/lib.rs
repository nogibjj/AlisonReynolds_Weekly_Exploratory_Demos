use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use humantime::format_duration;

// converts date string to datetime
pub fn date_string_to_datetime(date: String) -> DateTime<Utc> {
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").unwrap();
    let date = DateTime::<Utc>::from_utc(date.and_hms(0, 0, 0), Utc);
    date
}

// converts time string to naivetime
pub fn time_string_to_naive_time(time: String) -> NaiveTime {
    let time = NaiveTime::parse_from_str(&time, "%H:%M:%S").unwrap();
    time
}

// calculates duration since date and returns nicely formatted string
pub fn how_long_date(date: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(date);
    format_duration(duration.to_std().unwrap()).to_string()
}

// calculates duration since time and returns nicely formatted string
pub fn how_long_time(time: NaiveTime) -> String {
    let now = Utc::now();
    let duration = now.time().signed_duration_since(time);
    format_duration(duration.to_std().unwrap()).to_string()
}
