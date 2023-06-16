use crate::RecordDescriptor;

pub trait HasRecordDescriptor {
    fn descriptor() -> &'static RecordDescriptor;
}