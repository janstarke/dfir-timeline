use std::{collections::HashSet, io::Write};

use binrw::{io::NoSeek, BinResult, BinWrite};
use flow_record_common::{Error, RecordPack};
use rmpv::Value;

use crate::{FlowRecord, Record};

pub const RECORDSTREAM_MAGIC: &[u8] = b"RECORDSTREAM\n";

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

    pub fn serialize<R>(&mut self, record: R) -> Result<(), Error>
    where
        R: Record,
    {
        if !self.has_header_written {
            self.write_header()?;
        }

        if !self
            .written_descriptor_hashes
            .contains(&R::descriptor_hash())
        {
            self.write_descriptor::<R>()?;
        }

        self.write_flow_record(Value::try_from(RecordPack::with_record(record))?.into())?;

        Ok(())
    }

    fn write_descriptor<R>(&mut self) -> Result<(), Error>
    where
        R: Record,
    {
        self.write_flow_record(
            Value::try_from(RecordPack::with_descriptor(R::descriptor().clone()))?.into(),
        )?;
        self.written_descriptor_hashes.insert(R::descriptor_hash());
        Ok(())
    }

    fn write_header(&mut self) -> Result<(), Error> {
        self.write_flow_record(Value::Binary(RECORDSTREAM_MAGIC.to_vec()).into())?;
        self.has_header_written = true;
        Ok(())
    }

    fn write_flow_record(&mut self, fr: FlowRecord) -> BinResult<()> {
        fr.write_be(&mut self.writer)
    }
}
