use crate::FieldDescriptor;

pub enum RecordDescriptor {
    Unit,

    UnnamedFields(Vec<FieldDescriptor>),

    NamedFields(Vec<(String, FieldDescriptor)>),
}

impl FromIterator<FieldDescriptor> for RecordDescriptor {
    fn from_iter<T: IntoIterator<Item = FieldDescriptor>>(iter: T) -> Self {
        let v = Vec::from_iter(iter);
        Self::UnnamedFields(v)
    }
}
