use chrono::{DateTime, FixedOffset, TimeZone, Utc};

/// Indian Standard Time (UTC+05:30)
pub const IST: FixedOffset = FixedOffset::east_opt(5 * 3600 + 30 * 60).unwrap();

/// Create a UTC DateTime from an IST date/time.
///
/// Users don't need to bother with manually converting to UTC.
pub fn ist_datetime(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    IST.with_ymd_and_hms(year, month, day, hour, minute, 0)
        .unwrap()
        .with_timezone(&Utc)
}
