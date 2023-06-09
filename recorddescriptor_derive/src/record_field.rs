use serde::Serialize;

use crate::FieldType;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct RecordField {
    field_name: String,
    field_type: FieldType,
}

impl From<(String, FieldType)> for RecordField {
    fn from(value: (String, FieldType)) -> Self {
        Self {
            field_name: value.0,
            field_type: value.1,
        }
    }
}

impl Serialize for RecordField {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.field_name)
        //self.serialize(&self.field_type)
    }
}
