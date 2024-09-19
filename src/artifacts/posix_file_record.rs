use crate::Record;
use bodyfile::Bodyfile3Line;
use chrono::{DateTime, Utc};
use flow_record_common::types::Filesize;
use flow_record_derive::Record;

#[derive(Debug, Record)]
pub struct PosixFileRecord {
    file_name: String,
    user_id: i64,
    group_id: i64,
    mode: String,
    size: Filesize,

    modified: Option<DateTime<Utc>>,
    accessed: Option<DateTime<Utc>>,
    changed: Option<DateTime<Utc>>,
    birth: Option<DateTime<Utc>>,
}

struct UnixTimestamp(i64);

impl From<i64> for UnixTimestamp {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<UnixTimestamp> for Option<DateTime<Utc>> {
    fn from(value: UnixTimestamp) -> Self {
        if value.0 != -1 {
            DateTime::from_timestamp(value.0, 0)
        } else {
            None
        }
    }
}

impl TryFrom<&Bodyfile3Line> for PosixFileRecord {
    type Error = std::num::TryFromIntError;
    fn try_from(line: &Bodyfile3Line) -> Result<Self, Self::Error> {
        Ok(Self {
            file_name: line.get_name().to_string(),
            user_id: i64::try_from(line.get_uid())?,
            group_id: i64::try_from(line.get_gid())?,
            mode: line.get_mode().to_string(),
            size: line.get_size().into(),
            modified: UnixTimestamp::from(line.get_mtime()).into(),
            accessed: UnixTimestamp::from(line.get_atime()).into(),
            changed: UnixTimestamp::from(line.get_ctime()).into(),
            birth: UnixTimestamp::from(line.get_crtime()).into(),
        })
    }
}
