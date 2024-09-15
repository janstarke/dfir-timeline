use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

use crate::{Object, ObjectType};

pub struct ExtData(pub Object);

impl Serialize for ExtData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut ser = rmp_serde::Serializer::new(Vec::new())
            .with_bytes(rmp_serde::config::BytesMode::ForceAll);
        self.0.serialize(&mut ser).unwrap();
        SerializableExtType((
            ObjectType::RecordTypeExt as i8,
            ByteBuf::from(ser.into_inner()),
        ))
        .serialize(serializer)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "_ExtStruct")]
struct SerializableExtType((i8, serde_bytes::ByteBuf));
