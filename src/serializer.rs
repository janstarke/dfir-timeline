use std::{collections::HashSet, io::Write};

use flow_record_common::Object;
use serde::Serialize;

use crate::Record;

const RECORDSTREAM_MAGIC: &[u8] = b"RECORDSTREAM\n";

pub struct DfirSerializer<W: Write> {
    writer: W,
    has_header_written: bool,
    buffer: Vec<u8>,
    written_descriptor_hashes: HashSet<u32>,
}

impl<W> DfirSerializer<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            has_header_written: false,
            buffer: Vec::new(),
            written_descriptor_hashes: HashSet::new(),
        }
    }

    pub fn without_header(mut self) -> Self {
        self.has_header_written = true;
        self
    }

    pub fn serialize<R>(&mut self, record: &R) -> Result<(), rmp_serde::encode::Error>
    where
        R: Record,
    {
        if !self.has_header_written {
            self.print_magic()?;
            self.has_header_written = true;
        }

        let descriptor_hash = R::descriptor_hash();
        if !self.written_descriptor_hashes.contains(&descriptor_hash) {
            self.buffer.extend(R::descriptor());
            self.flush_buffer();

            self.written_descriptor_hashes.insert(descriptor_hash);
        }

        Object::with_record(record).serialize(&mut self.serializer())?;

        self.flush_buffer();

        Ok(())
    }

    fn serializer(&mut self) -> rmp_serde::Serializer<&mut Vec<u8>> {
        rmp_serde::Serializer::new(&mut self.buffer)
            .with_bytes(rmp_serde::config::BytesMode::ForceAll)
    }

    fn flush_buffer(&mut self) {
        let size = (self.buffer.len() as u32).to_be_bytes();
        self.writer.write_all(&size).unwrap();
        self.writer.write_all(&self.buffer).unwrap();
        self.buffer.clear();
    }

    pub fn print_magic(&mut self) -> Result<(), rmp_serde::encode::Error> {
        RECORDSTREAM_MAGIC.serialize(&mut self.serializer())?;
        self.flush_buffer();
        Ok(())
    }
}
