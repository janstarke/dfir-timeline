use serde::{ser::SerializeTuple, Deserialize, Serialize};

use crate::RecordField;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct RecordDescriptor {
    name: String,
    fields: Vec<RecordField>,
}

impl RecordDescriptor {
    pub const fn new(name: String, fields: Vec<RecordField>) -> Self {
        Self { name, fields }
    }
}

impl Serialize for RecordDescriptor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.name)?;
        tuple.serialize_element(&self.fields)?;
        tuple.end()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "_ExtStruct")]
struct DescriptorExtType((i8, serde_bytes::ByteBuf));
