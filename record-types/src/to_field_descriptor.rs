use crate::FieldType;
use chrono::{DateTime, Utc};


pub trait ToFieldType {
    fn to_field_type() -> FieldType;
}

impl ToFieldType for u32 {
    fn to_field_type() -> FieldType {
        FieldType::Int
    }
}

impl ToFieldType for i64 {
    fn to_field_type() -> FieldType {
        FieldType::Int
    }
}

impl ToFieldType for String {
    fn to_field_type() -> FieldType{
        FieldType::String
    }
}

impl ToFieldType for Option<DateTime<Utc>> {
    fn to_field_type() -> FieldType{
        FieldType::Timestamp
    }
}
