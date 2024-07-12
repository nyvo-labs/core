use chrono::{DateTime, Utc};

// https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-dosdatetimetofiletime

pub fn parse(date: u16, time: u16) -> DateTime<Utc> {
    let y = (date >> 9) + 1980;
    let m = (date >> 5) & 0x0F;
    let d = date & 0x1F;

    let h = time >> 11;
    let min = (time >> 5) & 0x3F;
    let s = (time & 0x1F) * 2;

    DateTime::parse_from_rfc3339(
        format!(
            "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}Z",
            y, m, d, h, min, s
        )
        .as_str(),
    )
    .unwrap_or_else(|_| DateTime::from_timestamp(0, 0).unwrap().into())
    .into()
}
