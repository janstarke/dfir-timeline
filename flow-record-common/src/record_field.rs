use getset::Getters;
use rmpv::Value;

use crate::FieldType;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Getters)]
#[getset(get = "pub")]
pub struct RecordField {
    field_name: String,
    field_type: FieldType,
}

impl RecordField {
    pub fn dissolve(self) -> (String, FieldType) {
        (self.field_name, self.field_type)
    }
}

impl From<(String, FieldType)> for RecordField {
    fn from(value: (String, FieldType)) -> Self {
        Self {
            field_name: value.0,
            field_type: value.1,
        }
    }
}

impl From<RecordField> for Value {
    fn from(value: RecordField) -> Self {
        Value::Array(vec![
            value.field_type.into(),
            Value::String(value.field_name.into()),
        ])
    }
}
