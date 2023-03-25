use apache_avro::{AvroSchema, Schema};
use bodyfile::Bodyfile3Line;
use chrono::{serde::ts_milliseconds, DateTime, TimeZone, Utc};
use chrono_tz::Tz;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{artifacts::{PosixFile, PosixFileEvent}, EventData};

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineRecord<D: EventData> {
    #[serde(with = "ts_milliseconds")]
    ts: DateTime<Utc>,

    #[serde(with = "crate::ser_tz")]
    tz: Option<Tz>,

    ev: D::EventType,

    ed: D,
}

lazy_static! {
    pub static ref TIMELINE_RECORD_POSIXFILE_SCHEMA: Schema = match Schema::parse(&json!({
        "type": "record",
        "name": "TimelineRecord_PosixFile",
        "namespace": crate::AVRO_NAMESPACE,
        "fields" : [
            {"name": "ts", "type": "long", "order": "ascending"},
            {"name": "tz", "type": "string"},
            {"name": "ev", "type": <PosixFile as EventData>::EventType::get_schema()},
            {"name": "ed", "type": PosixFile::get_schema()}
        ]
    })) {
        Ok(schema) => schema,
        Err(why) => panic!("unable to compile schema: {why}"),
    };
}

impl AvroSchema for TimelineRecord<PosixFile> {
    fn get_schema() -> apache_avro::Schema {
        TIMELINE_RECORD_POSIXFILE_SCHEMA.clone()
    }
}

impl<D> TimelineRecord<D>
where
    D: EventData,
{
    pub fn from(
        ts: DateTime<Utc>,
        original_tz: Option<Tz>,
        event: D::EventType,
        event_data: D,
    ) -> Self {
        Self {
            ts,
            tz: original_tz,
            ev: event,
            ed: event_data,
        }
    }
}

impl TimelineRecord<PosixFile> {
    pub fn iter_records_from(
        line: Bodyfile3Line,
    ) -> Result<impl Iterator<Item = Self>, std::num::TryFromIntError> {
        let mut records = Vec::new();
        if let Some(r) = Self::create_record(&line, PosixFileEvent::Modified, line.get_mtime())? {
            records.push(r)
        }
        if let Some(r) = Self::create_record(&line, PosixFileEvent::Accessed, line.get_atime())? {
            records.push(r)
        }
        if let Some(r) = Self::create_record(&line, PosixFileEvent::Changed, line.get_ctime())? {
            records.push(r)
        }
        if let Some(r) = Self::create_record(&line, PosixFileEvent::Created, line.get_crtime())? {
            records.push(r)
        }
        Ok(records.into_iter())
    }

    fn create_record(
        line: &Bodyfile3Line,
        event: PosixFileEvent,
        timestamp: i64,
    ) -> Result<Option<Self>, std::num::TryFromIntError> {
        if let Ok(seconds) = u32::try_from(timestamp) {
            let ts = match Utc.timestamp_opt(seconds.into(), 0) {
                chrono::LocalResult::None => {
                    panic!("unable to convert '{seconds}' into a timestamp");
                }
                chrono::LocalResult::Single(ts) => ts,
                chrono::LocalResult::Ambiguous(ts1, ts2) => {
                    log::warn!("the conversion of '{seconds}' leads to two different timestamps: '{ts1}' and '{ts2}'. We will use '{ts2}'");
                    ts2
                }
            };
            let posix_file = PosixFile::try_from(line)?;
            Ok(Some(Self::from(ts, None, event, posix_file)))
        } else {
            Ok(None)
        }
    }
}
