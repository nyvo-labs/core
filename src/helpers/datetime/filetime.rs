use chrono::{DateTime, Utc};

pub fn parse(timestamp: &u64) -> DateTime<Utc> {
    // TODO
    println!("{:?}", timestamp);
    DateTime::from_timestamp(0, 0).unwrap()
}

pub fn serialize(datetime: DateTime<Utc>) -> u64 {
    todo!("{:?}", datetime);
}
