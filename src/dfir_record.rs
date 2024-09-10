use super::HasRecordDescriptor;
use serde::Serialize;

pub trait DfirRecord: Serialize + HasRecordDescriptor {}
