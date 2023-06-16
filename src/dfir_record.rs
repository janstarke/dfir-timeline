use serde::{Serialize};
use super::HasRecordDescriptor;

pub trait DfirRecord: Serialize + HasRecordDescriptor {
}