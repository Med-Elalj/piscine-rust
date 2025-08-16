use std::collections::HashMap;
use chrono::{DateTime, Utc, Datelike};
use json::JsonValue;

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for commit in data.members() {
        if let Some(author) = commit["author"]["login"].as_str() {
            *map.entry(author.to_string()).or_insert(0) += 1;
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
                let year = datetime_utc.iso_week().year();
                let week = datetime_utc.iso_week().week();
                let week_str = format!("{}-W{}", year, week);
                *map.entry(week_str).or_insert(0) += 1;
            }
        }
    }

    map
}
