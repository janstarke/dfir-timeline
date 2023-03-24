use apache_avro::AvroSchema;
use bodyfile::Bodyfile3Line;
use serde::{Serialize, Deserialize};

use crate::EventData;

#[derive(Debug, Serialize, Deserialize, AvroSchema)]
pub struct PosixFile {
    file_name: String,
    user_id: i64,
    group_id: i64,
    mode: String,
    size: i64,
}

impl EventData for PosixFile {

}

impl TryFrom<&Bodyfile3Line> for PosixFile {
    type Error = std::num::TryFromIntError;
    fn try_from(line: &Bodyfile3Line) -> Result<Self, Self::Error> {
        Ok(Self {
            file_name: line.get_name().to_string(),
            user_id: i64::try_from(line.get_uid())?,
            group_id: i64::try_from(line.get_gid())?,
            mode: line.get_mode().to_string(),
            size: i64::try_from(line.get_size())?
        })
    }
}