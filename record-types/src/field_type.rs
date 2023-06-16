use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Hash)]
pub enum FieldType {
    Bool,
    Int,
    Float,
    String,
    Bin,
    Timestamp
}

