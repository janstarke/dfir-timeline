use record_types::RecordDescriptor;
use serde::{Serialize};

pub trait DfirRecord: Serialize {
    fn descriptor() -> &'static RecordDescriptor;
}