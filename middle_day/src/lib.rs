use chrono::{Datelike, NaiveDate};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let last_day = NaiveDate::from_ymd_opt(year as i32, 12, 31)?;
    let days_in_year = last_day.ordinal();

    if days_in_year % 2 == 0 {
        return None;
    }

    let middle_ordinal = days_in_year / 2 + 1;

    let middle_date = NaiveDate::from_yo_opt(year as i32, middle_ordinal)?;

    Some(middle_date.weekday())
}
