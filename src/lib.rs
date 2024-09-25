#![doc = include_str!("../README.md")]

//! # Usage
//!
//! ```rust
//! use binrw::BinReaderExt;
//! use chrono::prelude::*;
//! use flow_record::prelude::*;
//! use flow_record::derive::*;
//! use std::io::{Cursor,Seek,SeekFrom};
//! 
//! #[derive(FlowRecord)]
//! #[flow_record(version = 1, source = "Sample", classification = "file", skip_meta=true)]
//! struct SampleStruct {
//!     int_value: u32,
//!     str_value: String,
//!     dtm_value: DateTime<Utc>
//! }
//!
//! let now = Utc::now();
//! let sample_struct = SampleStruct {
//!     int_value: 42,
//!     str_value: "forty two".into(),
//!     dtm_value: now,
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
//!# use flow_record::prelude::*;
//!# use flow_record::derive::FlowRecord;
//!# use std::io::{Cursor,Seek,SeekFrom};
//!# #[derive(FlowRecord)]
//!# #[flow_record(version = 1, source = "Sample", classification = "file", skip_meta=true)]
//!# struct SampleStruct {
//!#     int_value: u32,
//!#     str_value: String,
//!#     dtm_value: DateTime<Utc>
//!# }
//! let now = Utc::now();
//!# let sample_struct = SampleStruct {
//!#     int_value: 42,
//!#     str_value: "forty two".into(),
//!#     dtm_value: now,
//!# };
//!# let mut ser = Serializer::new(Vec::new());
//!# ser.serialize(sample_struct).unwrap();
//!# let mut raw_data = Cursor::new(ser.into_inner());
//! // omit the header
//! raw_data.seek(SeekFrom::Start((4+2+RECORDSTREAM_MAGIC.len()).try_into().unwrap()));
//!
//! let descriptor_record: RawFlowRecord = raw_data.read_be().unwrap();
//! let data_record: RawFlowRecord = raw_data.read_be().unwrap();
//!
//! let descriptor = RecordPack::try_from(Value::from(descriptor_record)).unwrap();
//! let data = RecordPack::try_from(Value::from(data_record)).unwrap()
//!                 .inner().clone();
//!
//! assert_eq!(data,
//!        Value::Array(vec![
//!            1.into(), // record pack type
//!            Value::Array(vec![
//!                Value::Array(vec![    // reference to record descriptor
//!                    "SampleStruct".into(),  // struct name
//!                    114706890.into()        // struct hash
//!                ]),
//!                Value::Array(vec![   // actual data
//!                    42.into(),
//!                    "forty two".into(),
//!                    now.timestamp().into()
//!                ])
//!            ])
//!        ]));
//! ```
//! 

pub mod artifacts;

mod raw_flow_record;
mod record_pack_type;
mod serializer;

pub mod prelude {
    pub use super::record_pack_type::*;
    pub use super::serializer::DfirSerializer as Serializer;
    
    pub use super::raw_flow_record::*;
    pub use flow_record_common::*;
    pub use super::serializer::RECORDSTREAM_MAGIC;
}

pub mod derive {
    pub use flow_record_derive::FlowRecord;
}