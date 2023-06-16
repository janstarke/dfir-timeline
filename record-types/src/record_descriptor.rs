use serde::Serialize;

use crate::RecordField;

#[derive(Clone, Eq, PartialEq, Debug, Serialize)]
pub struct RecordDescriptor(
    Vec<RecordField>,
);

impl FromIterator<RecordField> for RecordDescriptor {
    fn from_iter<T: IntoIterator<Item = RecordField>>(iter: T) -> Self {
        let v = Vec::from_iter(iter);
        Self(v)
    }
}
