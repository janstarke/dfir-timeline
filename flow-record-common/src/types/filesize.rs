use crate::ToMsgPackValue;

#[derive(Debug)]
pub struct Filesize(u64);

impl From<u64> for Filesize {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Filesize> for u64 {
    fn from(value: Filesize) -> Self {
        value.0
    }
}

impl ToMsgPackValue for Filesize {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::Integer(self.0.into())
    }

    fn field_type() -> crate::FieldType {
        crate::FieldType::Filesize
    }
}