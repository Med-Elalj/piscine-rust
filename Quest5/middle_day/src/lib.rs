use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    // Check if the year is leap year
    let is_leap = {
        // Leap year rule:
        // Divisible by 4 AND (not divisible by 100 OR divisible by 400)
        (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
    };

    // Number of days in the year
    let days_in_year = if is_leap { 366 } else { 365 };

    // If even number of days, no single middle day
    if days_in_year % 2 == 0 {
        return None;
    }

    // Middle day index (1-based)
    // For odd days, middle is exactly (days_in_year + 1) / 2
    let middle_day_index = (days_in_year + 1) / 2;

    // Construct the date for January 1st of the year
    let first_day = NaiveDate::from_ymd_opt(year as i32, 1, 1)?;

    // Calculate middle day by adding (middle_day_index - 1) days to Jan 1
    let middle_date = first_day.checked_add_signed(chrono::Duration::days((middle_day_index - 1) as i64))?;

    // Return the weekday of the middle date
    Some(middle_date.weekday())
}
