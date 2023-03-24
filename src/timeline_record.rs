use std::collections::HashSet;

use apache_avro::{schema::derive::AvroSchemaComponent, AvroSchema, Schema};
use bodyfile::Bodyfile3Line;
use chrono::{TimeZone, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{artifacts::PosixFile, Action, Artifact, Timestamp};

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineRecord<A: Artifact + AvroSchemaComponent> {
    artifact: A,
    timestamps: HashSet<Timestamp>,
}

lazy_static! {
    pub static ref TIMELINE_RECORD_POSIXFILE_SCHEMA: Schema = match Schema::parse(&json!({
        "type": "record",
        "name": "TimelineRecord_PosixFile",
        "namespace": "dfir-timeline",
        "fields" : [
            {"name": "artifact", "type": PosixFile::get_schema()},
            {"name": "timestamps", "type": "array", "items": Timestamp::get_schema()}
        ]
    })) {
        Ok(schema) => schema,
        Err(why) => panic!("unable to compile schema: {why}")
    };
}

impl AvroSchema for TimelineRecord<PosixFile>
{
    fn get_schema() -> apache_avro::Schema {
        TIMELINE_RECORD_POSIXFILE_SCHEMA.clone()
    }
}

pub trait AddBodyfileTimestamp {
    fn add_bodyfile_timestamp(&mut self, action: Action, timestamp: i64);
}

impl<A> TimelineRecord<A>
where
    A: Artifact + AvroSchemaComponent,
{
    pub fn add_timestamp(&mut self, ts: Timestamp) {
        self.timestamps.insert(ts);
    }
}

impl<A> From<A> for TimelineRecord<A>
where
    A: Artifact + AvroSchemaComponent,
{
    fn from(artifact: A) -> Self {
        Self {
            artifact,
            timestamps: Default::default(),
        }
    }
}

impl AddBodyfileTimestamp for TimelineRecord<PosixFile> {
    fn add_bodyfile_timestamp(&mut self, action: Action, timestamp: i64) {
        if let Ok(seconds) = u32::try_from(timestamp) {
            match Utc.timestamp_opt(seconds.into(), 0) {
                chrono::LocalResult::None => {
                    log::warn!("unable to convert '{seconds}' into a timestamp")
                }
                chrono::LocalResult::Single(ts) => self.add_timestamp((action, ts).into()),
                chrono::LocalResult::Ambiguous(ts1, ts2) => {
                    log::warn!("the conversion of '{seconds}' leads to two different timestamps: '{ts1}' and '{ts2}'. We will use '{ts2}'");
                    self.add_timestamp((action, ts2).into())
                }
            }
        }
    }
}

impl TryFrom<Bodyfile3Line> for TimelineRecord<PosixFile> {
    type Error = std::num::TryFromIntError;
    fn try_from(line: Bodyfile3Line) -> Result<Self, Self::Error> {
        // FIXME: this could be faster when omitting the call to from(), which copies two strings internally
        let posix_file = PosixFile::try_from(&line)?;

        let mut record = TimelineRecord::from(posix_file);
        record.add_bodyfile_timestamp(Action::Modified, line.get_mtime());
        record.add_bodyfile_timestamp(Action::Accessed, line.get_atime());
        record.add_bodyfile_timestamp(Action::Changed, line.get_ctime());
        record.add_bodyfile_timestamp(Action::Created, line.get_crtime());
        Ok(record)
    }
}
