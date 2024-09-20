use file_mode::{Mode, ModeParseError};

use crate::ToMsgPackValue;

#[derive(Debug)]
pub struct UnixFileMode(Mode);

impl TryFrom<&str> for UnixFileMode {
    type Error = ModeParseError;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut mode = Mode::empty();
        let mut value = value;

        if value.chars().nth(1) == Some('/') {
            value = &value[3..];
        }

        let value = format!("={value}");

        mode.set_str(&value)?;
        Ok(Self(mode))
    }
}

impl ToMsgPackValue for UnixFileMode {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::Integer(self.0.mode().into())
    }

    fn field_type() -> crate::FieldType {
        crate::FieldType::UnixFileMode
    }
}