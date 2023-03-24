
use std::str::FromStr;

use chrono_tz::Tz;
use serde::{Deserialize, Deserializer, Serializer};

pub(crate) fn serialize<S>(tz: &Option<Tz>, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.serialize_str(tz.map(|tz| tz.name()).unwrap_or(""))
}

pub(crate) fn deserialize<'de, D>(des: D) -> Result<Option<Tz>, D::Error>
where
    D: Deserializer<'de>,
{
    let buffer = String::deserialize(des)?;
    if buffer.is_empty() {
        Ok(None)
    } else {
        let tz = Tz::from_str(&buffer).map_err(serde::de::Error::custom)?;
        Ok(Some(tz))
    }
}
