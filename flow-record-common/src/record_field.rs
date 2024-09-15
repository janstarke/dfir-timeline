use getset::Getters;
use serde::{ser::SerializeTuple, Serialize};

use crate::FieldType;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Getters)]
#[getset(get="pub")]
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
        let mut tuple = serializer.serialize_tuple(2)?;

        // keep in mind: the order is important here
        tuple.serialize_element(&self.field_type)?;
        tuple.serialize_element(&self.field_name)?;
        tuple.end()
    }
}
