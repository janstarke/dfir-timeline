use record_types::{RecordDescriptor, HasRecordDescriptor};
use serde::{Serialize};

pub trait DfirRecord: Serialize + HasRecordDescriptor {
    fn descriptor() -> &'static RecordDescriptor;
}