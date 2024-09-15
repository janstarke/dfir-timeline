use serde::{ser::SerializeTuple, Serialize};

#[derive(Serialize, Clone, Copy)]
#[repr(i8)]
#[allow(unused)]
pub enum ObjectType {
    RecordTypeExt = 0x0e,

    RecordPackTypeRecord = 0x1,
    RecordPackTypeDescriptor = 0x2,
    RecordPackTypeFieldtype = 0x3,
    RecordPackTypeDatetime = 0x10,
    RecordPackTypeVarint = 0x11,
    RecordPackTypeGroupedrecord = 0x12,
}

impl ObjectType {
    pub fn serialize<D, S>(&self, serializer: S, data: &D) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
        D: Serialize
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&(*self as i8))?;
        tuple.serialize_element(data)?;
        tuple.end()
    }
}
