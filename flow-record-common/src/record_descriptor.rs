use rmpv::Value;

use crate::RecordField;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct RecordDescriptor {
    name: String,
    fields: Vec<RecordField>,
}

impl RecordDescriptor {
    pub const fn new(name: String, fields: Vec<RecordField>) -> Self {
        Self { name, fields }
    }
}

impl From<RecordDescriptor> for Value {
    fn from(value: RecordDescriptor) -> Self {
        Value::Array(vec![
            Value::String(value.name.into()),
            Value::Array(value.fields.into_iter().map(Value::from).collect()),
        ])
    }
}
