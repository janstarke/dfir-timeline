use apache_avro::AvroSchema;
use apache_avro::schema::{Name, Schema, Aliases};
use chrono::{serde::ts_milliseconds, DateTime, Utc};
use chrono_tz::Tz;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::json;
mod ser_tz {
    use std::str::FromStr;

    use chrono_tz::Tz;
    use serde::{Serializer, Deserializer, Deserialize};
    
    pub (crate) fn serialize<S>(tz: &Tz, ser: S) -> Result<S::Ok, S::Error> where S: Serializer {
        ser.serialize_str(tz.name())
    }
    
    pub (crate) fn deserialize<'de, D>(des: D)  -> Result<Tz, D::Error> where D: Deserializer<'de> {
        Tz::from_str(&String::deserialize(des)?).map_err(serde::de::Error::custom)
    }
}

use crate::Action;

#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct Timestamp {
    action: Action,

    #[serde(with = "ts_milliseconds")]
    ts: DateTime<Utc>,

    #[serde(with = "ser_tz")]
    source_tz: Tz,
}

lazy_static! {
    pub static ref TIMESTAMP_SCHEMA: Schema = match Schema::parse(&json!({
        "name": "Timestamp",
        "type": "record",
        "namespace": "dfir-timeline",
        "fields" : [
            {"name": "ts", "type": "long"},
            {"name": "tz", "type": "string"}
        ]
    })) {
    Ok(schema) => schema,
    Err(why) => panic!("unable to compile schema: {why}")
};
}

impl AvroSchema for Timestamp {
    fn get_schema() -> apache_avro::Schema {
        TIMESTAMP_SCHEMA.clone()
    }
}

impl From<(Action, DateTime<Tz>)> for Timestamp
{
    fn from(value: (Action, DateTime<Tz>)) -> Self {
        Self {
            action: value.0,
            ts: value.1.with_timezone(&Utc),
            source_tz: value.1.timezone(),
        }
    }
}

impl From<(Action, DateTime<Utc>)> for Timestamp
{
    fn from(value: (Action, DateTime<Utc>)) -> Self {
        Self {
            action: value.0,
            ts: value.1.with_timezone(&Utc),
            source_tz: chrono_tz::UTC,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono_tz::{Europe::Berlin, Tz, UTC};

    use crate::{Timestamp, Action};

    fn create_ts(tz: Tz) -> Timestamp {
        match tz.with_ymd_and_hms(2016, 2, 10, 12, 0, 0) {
        chrono::LocalResult::None => {
            unreachable!()
        },
        chrono::LocalResult::Single(time) => {
            Timestamp::from((Action::Created, time))
        }
        chrono::LocalResult::Ambiguous(_, _) => {
            unreachable!()
        }
    }
    }

    #[test]
    fn test_from_utc() {
        let ts = create_ts(UTC);
        assert_eq!(ts.source_tz, UTC);
        assert_eq!(ts.source_tz.name(), "UTC");
        assert_eq!(ts.ts.timestamp_millis(), 1455105600 * 1000);
    }

    #[test]
    fn test_from_berlin() {
        let ts = create_ts(Berlin);
        assert_eq!(ts.source_tz, Berlin);
        assert_eq!(ts.source_tz.name(), "Europe/Berlin");
        assert_eq!(ts.ts.timestamp_millis(), 1455102000 * 1000);
    }
}
