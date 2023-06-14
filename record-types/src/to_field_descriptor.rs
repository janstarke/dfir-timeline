use crate::FieldDescriptor;
use chrono::{DateTime, Utc};


pub trait ToFieldDescriptor {
    fn to_field_descriptor() -> FieldDescriptor;
}

impl ToFieldDescriptor for u32 {
    fn to_field_descriptor() -> FieldDescriptor {
        FieldDescriptor::Int
    }
}

impl ToFieldDescriptor for i64 {
    fn to_field_descriptor() -> FieldDescriptor {
        FieldDescriptor::Int
    }
}

impl ToFieldDescriptor for String {
    fn to_field_descriptor() -> FieldDescriptor{
        FieldDescriptor::String
    }
}

impl ToFieldDescriptor for Option<DateTime<Utc>> {
    fn to_field_descriptor() -> FieldDescriptor{
        FieldDescriptor::Timestamp
    }
}
