use chrono::{DateTime, Datelike, Utc, Weekday};
use json::JsonValue;
use std::collections::HashMap;
use std::fs;

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for commit in data.members() {
        // Try to get the login of the author
        if let Some(login) = commit["author"]["login"].as_str() {
            *map.entry(login.to_string()).or_insert(0) += 1;
        }
    }

    map
}

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for commit in data.members() {
        if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
            if let Ok(datetime) = DateTime::parse_from_rfc3339(date_str) {
                let datetime_utc = datetime.with_timezone(&Utc);
                let iso_week = datetime_utc.iso_week();
                let key = format!("{}-W{}", iso_week.year(), iso_week.week());

                *map.entry(key).or_insert(0) += 1;
            }
        }
    }

    map
}
