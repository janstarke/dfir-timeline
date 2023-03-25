pub const AVRO_NAMESPACE: &str = "dfir_timeline";

pub mod artifacts;
mod timeline_record;
mod event;
pub (crate) mod ser_tz;

pub use event::*;
pub use timeline_record::*;