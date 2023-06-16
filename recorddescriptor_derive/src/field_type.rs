use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Hash)]
#[allow(dead_code)]
pub enum FieldType {
    Bool,
    Int,
    Float,
    String,
    Bin,
    Timestamp
}

