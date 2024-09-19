use crate::ToMsgPackValue;

#[derive(Debug)]
pub struct UnixFileMode(String);

impl From<String> for UnixFileMode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<UnixFileMode> for String {
    fn from(value: UnixFileMode) -> Self {
        value.0
    }
}

impl ToMsgPackValue for UnixFileMode {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::String(self.0.into())
    }

    fn field_type() -> crate::FieldType {
        crate::FieldType::UnixFileMode
    }
}