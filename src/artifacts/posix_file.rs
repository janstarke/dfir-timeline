use apache_avro::AvroSchema;
use bodyfile::Bodyfile3Line;
use serde::{Deserialize, Serialize};

use crate::{Event, EventData};

#[derive(Debug, Serialize, Deserialize, AvroSchema)]
pub enum PosixFileEvent {
    Modified,
    Accessed,
    Changed,
    Created,
}

impl Event for PosixFileEvent {}

#[derive(Debug, Serialize, Deserialize, AvroSchema)]
pub struct PosixFile {
    file_name: String,
    user_id: i64,
    group_id: i64,
    mode: String,
    size: i64,
}

impl EventData for PosixFile {
    type EventType = PosixFileEvent;
}

impl TryFrom<&Bodyfile3Line> for PosixFile {
    type Error = std::num::TryFromIntError;
    fn try_from(line: &Bodyfile3Line) -> Result<Self, Self::Error> {
        Ok(Self {
            file_name: line.get_name().to_string(),
            user_id: i64::try_from(line.get_uid())?,
            group_id: i64::try_from(line.get_gid())?,
            mode: line.get_mode().to_string(),
            size: i64::try_from(line.get_size())?,
        })
    }
}
