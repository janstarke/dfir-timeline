use std::{collections::HashSet, io::{Read, Write}};

use binrw::{io::NoSeek, BinWrite};
use flow_record_common::Object;
use rmpv::Value;

use crate::{FlowRecord, Record};

const RECORDSTREAM_MAGIC: &[u8] = b"RECORDSTREAM\n";

pub struct DfirSerializer<W: Write> {
    writer: NoSeek<W>,
    has_header_written: bool,
    written_descriptor_hashes: HashSet<u32>,
}

impl<W> DfirSerializer<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        let writer = NoSeek::new(writer);
        Self {
            writer,
            has_header_written: false,
            written_descriptor_hashes: HashSet::new(),
        }
    }

    pub fn into_inner(self) -> W {
        self.writer.into_inner()
    }

    pub fn without_header(mut self) -> Self {
        self.has_header_written = true;
        self
    }

    pub fn serialize<R>(&mut self, record: R) -> Result<(), rmp_serde::encode::Error>
    where
        R: Record,
    {
        if !self.has_header_written {
            FlowRecord::from(Value::Binary(RECORDSTREAM_MAGIC.to_vec()))
                .write_be(&mut self.writer)
                .unwrap();
            self.has_header_written = true;
        }

        let descriptor_hash = R::descriptor_hash();
        if !self.written_descriptor_hashes.contains(&descriptor_hash) {
            FlowRecord::from(R::descriptor().clone())
                .write_be(&mut self.writer)
                .unwrap();
            self.written_descriptor_hashes.insert(descriptor_hash);
        }
        FlowRecord::from(Value::try_from(Object::with_record(record))?)
            .write_be(&mut self.writer)
            .unwrap();

        Ok(())
    }
}
