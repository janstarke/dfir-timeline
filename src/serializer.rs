use std::io::Write;

use serde::Serializer;

use crate::DfirRecord;

const RECORDSTREAM_MAGIC: &[u8] = b"RECORDSTREAM\n";

pub struct DfirSerializer<W: Write> {
    ser: rmp_serde::Serializer<W>,
    has_header_written: bool,
    buffer: Vec<u8>,
}

impl<W> DfirSerializer<W>
where
    W: Write,
{
    pub fn new(writer: W) -> Self {
        Self {
            ser: rmp_serde::Serializer::new(writer),
            has_header_written: false,
            buffer: Vec::new(),
        }
    }

    pub fn serialize<R>(&mut self, record: &R) -> Result<(), rmp_serde::encode::Error> where R: DfirRecord {
        if ! self.has_header_written {
            self.print_magic()?;
            self.has_header_written = true;
        }

        record.serialize(&mut self.serializer())?;

        self.flush_buffer();

        Ok(())
    }

    fn serializer(&mut self) -> rmp_serde::Serializer<&mut Vec<u8>> {
        rmp_serde::Serializer::new(&mut self.buffer)
    }

    fn flush_buffer(&mut self) {
        let size = (self.buffer.len() as u32).to_be_bytes();
        self.ser.get_mut().write_all(&size).unwrap();
        self.ser.get_mut().write_all(&self.buffer).unwrap();
        self.buffer.clear();
    }

    pub fn print_magic(&mut self) -> Result<(), rmp_serde::encode::Error> {
        self.serializer().serialize_bytes(RECORDSTREAM_MAGIC)?;
        self.flush_buffer();

        Ok(())
    }
}
