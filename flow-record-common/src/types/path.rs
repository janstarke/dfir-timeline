use std::path::PathBuf;

use crate::ToMsgPackValue;

#[repr(u8)]
pub enum PathType {
    Posix = 0,
    Windows = 1,
}

pub struct Path {
    path: PathBuf,
    path_type: PathType
}

impl Path {
    pub fn new(path: PathBuf, path_type: PathType) -> Self {
        Self {
            path,
            path_type
        }
    }
}

impl ToMsgPackValue for Path {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::Array(vec![
            self.path.to_string_lossy().into(),
            (self.path_type as u8).to_string().into()
        ])
    }

    fn field_type() -> crate::FieldType {
        crate::FieldType::Path
    }
}
