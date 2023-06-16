use serde::Serialize;

use crate::RecordField;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Hash)]
pub struct RecordDescriptor(
    Vec<RecordField>,
);

impl RecordDescriptor {
    pub const fn from(fields: Vec<RecordField>) -> Self {
        Self(fields)
    }
}
