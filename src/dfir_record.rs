use record_types::HasRecordDescriptor;
use serde::{Serialize};

pub trait DfirRecord: Serialize + HasRecordDescriptor {
}