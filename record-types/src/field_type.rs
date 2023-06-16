use serde::Serialize;

#[derive(Clone, Eq, PartialEq, Debug, Serialize)]
pub enum FieldType {
    Bool,
    Int,
    Float,
    String,
    Bin,
    Timestamp
}

