use std::io::Cursor;

use rmpv::Value;

use crate::{Error, ObjectType, Record, RecordDescriptor};

pub struct RecordPack(Value);

impl RecordPack {
    pub fn with_descriptor(descriptor: RecordDescriptor) -> Self {
        Self(Value::Array(vec![
            ObjectType::RecordPackTypeDescriptor.into(),
            descriptor.into()
        ]))
    }

    pub fn with_record<R>(record: R) -> Self
    where
        R: Record,
    {
        Self(Value::Array(vec![
            ObjectType::RecordPackTypeRecord.into(),
            Value::Array(vec![
                Value::Array(vec![
                    Value::String(R::name().into()),
                    Value::Integer(R::descriptor_hash().into()),
                ]),
                record.into_value(),
            ]),
        ]))
    }

    pub fn inner(&self) -> &Value {
        &self.0
    }
}

impl TryFrom<RecordPack> for Value {
    type Error = rmpv::encode::Error;

    fn try_from(value: RecordPack) -> Result<Self, Self::Error> {
        let mut buffer = Vec::new();
        rmpv::encode::write_value(&mut buffer, &value.0)?;

        Ok(Value::Ext(ObjectType::RecordTypeExt as i8, buffer))
    }
}

impl TryFrom<Value> for RecordPack {
    type Error = crate::Error;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Ext(type_id, vec) => {
                if type_id == ObjectType::RecordTypeExt as i8 {
                    let payload = rmpv::decode::read_value(&mut Cursor::new(vec))?;
                    Ok(Self(payload))
                } else {
                    Err(Error::InvalidExtTypeId(type_id))
                }
            }
            _ => Err(Error::ExpectedExtValue),
        }
    }
}
