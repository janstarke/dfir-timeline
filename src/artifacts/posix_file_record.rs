use bodyfile::Bodyfile3Line;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Serialize};
use recorddescriptor_derive::HasRecordDescriptor;
use record_types::HasRecordDescriptor;
use record_types::ToFieldDescriptor;

#[derive(Debug, Serialize, HasRecordDescriptor)]
pub struct PosixFileRecord {
    file_name: String,
    user_id: i64,
    group_id: i64,
    mode: String,
    size: i64,
    modified: Option<DateTime<Utc>>,
    accessed: Option<DateTime<Utc>>,
    changed: Option<DateTime<Utc>>,
    birth: Option<DateTime<Utc>>,
}
/*
lazy_static! {
    static ref DESCRIPTOR: RecordDescriptor = schema_for!(PosixFileRecord).into();
}

impl DfirRecord for PosixFileRecord {
    fn descriptor() -> &'static RecordDescriptor {
        &DESCRIPTOR
    }
}
 */

struct UnixTimestamp(i64);

impl From<i64> for UnixTimestamp {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<UnixTimestamp> for Option<DateTime<Utc>> {
    fn from(value: UnixTimestamp) -> Self {
        if value.0.is_negative() {
            None
        } else {
            NaiveDateTime::from_timestamp_opt(value.0, 0)
                .map(|ndt| DateTime::<Utc>::from_utc(ndt, Utc))
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
            size: i64::try_from(line.get_size())?,
            modified: UnixTimestamp::from(line.get_mtime()).into(),
            accessed: UnixTimestamp::from(line.get_atime()).into(),
            changed: UnixTimestamp::from(line.get_ctime()).into(),
            birth: UnixTimestamp::from(line.get_crtime()).into(),
        })
    }
}

