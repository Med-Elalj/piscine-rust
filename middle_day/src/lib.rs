use chrono::{Datelike, NaiveDate, Weekday};

pub fn middle_day(year: u32) -> Option<Weekday> {
    let l = {
        (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
    };
    let d = if l { 366 } else { 365 };

    if d % 2 == 0 {
        return None;
    }

    let md = (d + 1) / 2;

    let fd = NaiveDate::from_ymd_opt(year as i32, 1, 1)?;

    let res = fd.checked_add_signed(chrono::Duration::days((md - 1) as i64))?;

    Some(res.weekday())
}
