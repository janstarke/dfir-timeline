use std::io::stdout;

use bodyfile::Bodyfile3Line;
use chrono::{DateTime, Utc};
use flow_record::artifacts::posix::FileMode;
use flow_record::artifacts::posix::FileType;
use flow_record::prelude::*;
use flow_record::derive::*;
use types::Filesize;
use types::Path;
use types::PathType;

fn main() {
    let sample_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxr-xr-x|0|0|92|1577092511|1577092511|1577092511|-1";
    let bf_line = Bodyfile3Line::try_from(sample_line).unwrap();
    let record = FileRecord::try_from(&bf_line).unwrap();
    let mut ser = Serializer::new(stdout());

    ser.serialize(record).unwrap();
}

#[derive(FlowRecord)]
#[flow_record(version = 1, source = "Posix", classification = "file")]
pub struct FileRecord {
    file_name: Path,
    user_id: i64,
    group_id: i64,
    file_type: FileType,
    mode: FileMode,
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

impl TryFrom<&Bodyfile3Line> for FileRecord {
    type Error = flow_record_common::Error;
    fn try_from(line: &Bodyfile3Line) -> Result<Self, Self::Error> {
        Ok(Self {
            file_name: Path::new(line.get_name().to_string().into(), PathType::Posix),
            user_id: i64::try_from(line.get_uid())?,
            group_id: i64::try_from(line.get_gid())?,
            mode: line.get_mode().try_into()?,
            file_type: line.get_mode().try_into()?,
            size: line.get_size().into(),
            modified: UnixTimestamp::from(line.get_mtime()).into(),
            accessed: UnixTimestamp::from(line.get_atime()).into(),
            changed: UnixTimestamp::from(line.get_ctime()).into(),
            birth: UnixTimestamp::from(line.get_crtime()).into(),
        })
    }
}
