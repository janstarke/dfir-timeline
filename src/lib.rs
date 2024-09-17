//!
//! # Usage
//! 
//! ```rust
//! use binrw::BinReaderExt;
//! use chrono::prelude::*;
//! use flow_record::{FlowRecord, Object, Record, Serializer, RECORDSTREAM_MAGIC};
//! use flow_record_derive::Record;
//! use std::io::{Cursor,Seek,SeekFrom};
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
//! let mut raw_data = Cursor::new(ser.into_inner());
//! 
//! // omit the header
//! raw_data.seek(SeekFrom::Start((4+2+RECORDSTREAM_MAGIC.len()).try_into().unwrap()));
//! 
//! let descriptor_record: FlowRecord = raw_data.read_be().unwrap();
//! let data_record: FlowRecord = raw_data.read_be().unwrap();
//! 
//! let descriptor = Object::try_from(Value::from(descriptor_record)).unwrap();
//! let data = Object::try_from(Value::from(data_record)).unwrap();
//! ```
pub mod artifacts;
mod serializer;
mod record_pack_type;
mod flow_record;

pub use record_pack_type::*;
pub use serializer::DfirSerializer as Serializer;

pub use flow_record_common::*;
pub use flow_record::*;
pub use serializer::RECORDSTREAM_MAGIC;