use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

use crate::{ObjectType, Record, RecordDescriptor};

pub struct Object(ByteBuf);

impl Object {
    fn serializer() -> Serializer<Vec<u8>> {
        rmp_serde::Serializer::new(Vec::new()).with_bytes(rmp_serde::config::BytesMode::ForceAll)
    }
    pub fn with_descriptor(descriptor: &RecordDescriptor) -> Self {
        let mut ser = Self::serializer();
        ObjectType::RecordPackTypeDescriptor.serialize(&mut ser, descriptor).unwrap();
        Self(ByteBuf::from(ser.into_inner()))
    }

    pub fn with_record<R>(record: &R) -> Self where R: Record {
        let mut ser = Self::serializer();
        let metadata = (
            R::name(),
            R::descriptor_hash(),
        );
        let ser_data = (
            metadata,
            record
        );
        ObjectType::RecordPackTypeRecord.serialize(&mut ser, &ser_data).unwrap();
        Self(ByteBuf::from(ser.into_inner()))
    }
}

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializableExtType((ObjectType::RecordTypeExt as i8, self.0.clone())).serialize(serializer)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "_ExtStruct")]
struct SerializableExtType((i8, serde_bytes::ByteBuf));
