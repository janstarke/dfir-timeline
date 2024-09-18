//!
//! # Usage
//!
//! ```rust
//! use binrw::BinReaderExt;
//! use chrono::prelude::*;
//! use flow_record::{FlowRecord, RecordPack, Record, Serializer, RECORDSTREAM_MAGIC};
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
//! ```
//! 
//! That's basically all. The next steps are only necessary to validate
//! if all data were written correctly. You can ignore this, if you just want
//! to export the binary data.
//! 
//! ```rust
//!# use binrw::BinReaderExt;
//!# use chrono::prelude::*;
//!# use flow_record::{FlowRecord, RecordPack, Record, Serializer, RECORDSTREAM_MAGIC};
//!# use flow_record_derive::Record;
//!# use std::io::{Cursor,Seek,SeekFrom};
//!# #[derive(Record)]
//!# struct SampleStruct {
//!#     int_value: u32,
//!#     str_value: String,
//!#     dtm_value: DateTime<Utc>
//!# }
//!# let sample_struct = SampleStruct {
//!#     int_value: 42,
//!#     str_value: "forty two".into(),
//!#     dtm_value: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
//!# };
//!# let mut ser = Serializer::new(Vec::new());
//!# ser.serialize(sample_struct).unwrap();
//!# let mut raw_data = Cursor::new(ser.into_inner());
//! // omit the header
//! raw_data.seek(SeekFrom::Start((4+2+RECORDSTREAM_MAGIC.len()).try_into().unwrap()));
//!
//! let descriptor_record: FlowRecord = raw_data.read_be().unwrap();
//! let data_record: FlowRecord = raw_data.read_be().unwrap();
//!
//! let descriptor = RecordPack::try_from(Value::from(descriptor_record)).unwrap();
//! let data = RecordPack::try_from(Value::from(data_record)).unwrap()
//!                 .inner().clone();
//!
//! assert_eq!(data,
//!        Value::Array(vec![
//!            Value::Integer(1.into()), // record pack type
//!            Value::Array(vec![
//!                Value::Array(vec![    // reference to record descriptor
//!                    Value::String("SampleStruct".into()),  // struct name
//!                    Value::Integer(114706890.into())       // struct hash
//!                ]),
//!                Value::Array(vec![   // actual data
//!                    Value::Integer(42.into()),
//!                    Value::String("forty two".into()),
//!                    Value::Integer(1577836800.into())
//!                ])
//!            ])
//!        ]));
//! ```
pub mod artifacts;
mod flow_record;
mod record_pack_type;
mod serializer;

pub use record_pack_type::*;
pub use serializer::DfirSerializer as Serializer;

pub use flow_record::*;
pub use flow_record_common::*;
pub use serializer::RECORDSTREAM_MAGIC;