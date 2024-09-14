use std::io::Write;

use rmp_serde::Serializer;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;

use crate::{ext_type::ExtType, RecordField};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Hash)]
pub struct RecordDescriptor{
    name: String, 
    fields: Vec<RecordField>
}

impl RecordDescriptor {
    pub const fn new(name: String, fields: Vec<RecordField>) -> Self {
        Self{name, fields}
    }

    fn to_ext_type(&self) -> DescriptorExtType {
        let mut buffer: Vec<u8> = Vec::new();
        let mut ser = rmp_serde::Serializer::new(&mut buffer);
        self.name.serialize(&mut ser).unwrap();
        self.fields.serialize(&mut ser).unwrap();
        DescriptorExtType((
            ExtType::RecordPackTypeDescriptor as i8,
            ByteBuf::from(buffer),
        ))
    }

    pub fn serialize<W>(&self, ser: &mut Serializer<W>) -> Result<(), rmp_serde::encode::Error>
    where
        W: Write,
    {
        self.to_ext_type().serialize(ser)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "_ExtStruct")]
struct DescriptorExtType((i8, serde_bytes::ByteBuf));
