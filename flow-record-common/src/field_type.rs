use std::fmt::Display;

use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[allow(dead_code)]
pub enum FieldType {
    Bool,
    UInt16,
    UInt32,
    VarInt,
    Float,
    String,
    Bin,
    Datetime
}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            FieldType::Bool => "boolean",
            FieldType::UInt16 => "uint16",
            FieldType::UInt32 => "uint32",
            FieldType::VarInt => "varint",
            FieldType::Float => "float",
            FieldType::String => "string",
            FieldType::Bin => "bin",
            FieldType::Datetime => "datetime",
        })
    }
}

impl Serialize for FieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}