use chrono::{DateTime, Local};

// https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-dosdatetimetofiletime

pub fn parse(date: &u16, time: &u16) -> DateTime<Local> {
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

pub fn serialize(datetime: &DateTime<Local>) -> (u16, u16) {
    let date = datetime.format("%Y-%m-%d").to_string();
    let time = datetime.format("%H:%M:%S").to_string();

    let y = date[0..4].parse::<u16>().unwrap();
    let m = date[5..7].parse::<u16>().unwrap();
    let d = date[8..10].parse::<u16>().unwrap();

    let h = time[0..2].parse::<u16>().unwrap();
    let min = time[3..5].parse::<u16>().unwrap();
    let s = time[6..8].parse::<u16>().unwrap();

    let date = ((y - 1980) << 9) | (m << 5) | d;
    let time = (h << 11) | (min << 5) | (s / 2);

    (date, time)
}
