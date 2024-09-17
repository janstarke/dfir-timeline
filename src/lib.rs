//!
//! # Usage
//! 
//! ```rust
//! use chrono::prelude::*;
//! use flow_record::{Record, Serializer};
//! use flow_record_derive::Record;
//! 
//! #[derive(Record)]
//! struct SampleStruct {
//!     int_value: u32,
//!     str_value: String,
//!     dtm_value: DateTime<Utc>
//! }
//! 
//! let sample_struct = SampleStruct {
//!     int_value: 42,
//!     str_value: "forty two".into(),
//!     dtm_value: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
//! };
//! 
//! let mut ser = Serializer::new(Vec::new());
//! ser.serialize(sample_struct).unwrap();
//! 
//! let result = ser.into_inner();
//! ```
pub mod artifacts;
mod serializer;
mod record_pack_type;
mod flow_record;

pub use record_pack_type::*;
pub use serializer::DfirSerializer as Serializer;

pub use flow_record_common::*;
pub use flow_record::*;