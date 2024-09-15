use serde::{ser::SerializeTuple, Serialize};

use crate::RecordDescriptor;

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

#[allow(unused)]
pub enum Object {
    Record,
    Descriptor(RecordDescriptor),
    Fieldtype,
    Datetime,
    Varint,
    GroupedRecord,
}

impl Object {
    pub fn type_id(&self) -> ObjectType {
        match self {
            Object::Record => ObjectType::RecordPackTypeRecord,
            Object::Descriptor(_) => ObjectType::RecordPackTypeDescriptor,
            Object::Fieldtype => ObjectType::RecordPackTypeFieldtype,
            Object::Datetime => ObjectType::RecordPackTypeDatetime,
            Object::Varint => ObjectType::RecordPackTypeVarint,
            Object::GroupedRecord => ObjectType::RecordPackTypeGroupedrecord,
        }
    }
}

impl Serialize for Object {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&(self.type_id() as i8))?;
        match self {
            Object::Record => todo!(),
            Object::Descriptor(d) => tuple.serialize_element(d)?,
            Object::Fieldtype => todo!(),
            Object::Datetime => todo!(),
            Object::Varint => todo!(),
            Object::GroupedRecord => todo!(),
        }
        tuple.end()
    }
}
