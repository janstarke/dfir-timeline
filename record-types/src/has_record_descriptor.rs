use crate::RecordDescriptor;

pub trait HasRecordDescriptor {
    fn descriptor() -> RecordDescriptor;
}