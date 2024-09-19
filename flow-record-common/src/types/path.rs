use std::path::PathBuf;

use crate::ToMsgPackValue;

impl ToMsgPackValue for PathBuf {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::String(self.to_string_lossy().into())
    }

    fn field_type() -> crate::FieldType {
        crate::FieldType::Path
    }
}
