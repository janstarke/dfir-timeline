use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
#[allow(dead_code)]
pub enum FieldType {
    Bool,
    Int,
    Float,
    String,
    Bin,
    Timestamp
}

impl Serialize for FieldType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let s = match self {
            FieldType::Bool => "bool",
            FieldType::Int => "int",
            FieldType::Float => "float",
            FieldType::String => "string",
            FieldType::Bin => "bin",
            FieldType::Timestamp => "timestamp",
        };
        serializer.serialize_str(s)
    }
}