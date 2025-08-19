use chrono::{DateTime, Local};

pub struct FileEntry {
    pub index: u64,
    pub name: String,
    pub size: u64,
    pub directory: bool,
    pub encrypted: bool,
    pub modified: DateTime<Local>,
    pub created: DateTime<Local>,
    pub accessed: DateTime<Local>,
}
