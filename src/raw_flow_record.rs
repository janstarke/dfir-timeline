use std::io::Cursor;

use binrw::{helpers::count, BinRead, BinReaderExt, BinWrite};
use rmpv::Value;

pub struct RawFlowRecord(Value);

impl BinWrite for RawFlowRecord {
    type Args<'a> = ();

    fn write_options<W: std::io::Write + std::io::Seek>(
        &self,
        writer: &mut W,
        _endian: binrw::Endian,
        _args: Self::Args<'_>,
    ) -> binrw::BinResult<()> {
        let mut data = Vec::new();

        if let Err(why) = rmpv::encode::write_value(&mut data, &self.0) {
            return Err(binrw::Error::Custom {
                pos: writer.stream_position()?,
                err: Box::new(why),
            });
        }

        let length: u32 = data.len().try_into().unwrap();
        length.write_be(writer)?;
        data.write_be(writer)?;
        Ok(())
    }
}

impl BinRead for RawFlowRecord {
    type Args<'a> = ();

    fn read_options<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
        endian: binrw::Endian,
        args: Self::Args<'_>,
    ) -> binrw::BinResult<Self> {
        let length: u32 = reader.read_be()?;
        let data: Vec<u8> = count(length.try_into().unwrap())(reader, endian, args)?;

        match rmpv::decode::read_value(&mut Cursor::new(data)) {
            Ok(value) => Ok(Self(value)),
            Err(why) => Err(binrw::Error::Custom {
                pos: reader.stream_position()?,
                err: Box::new(why),
            }),
        }
    }
}

impl From<Value> for RawFlowRecord {
    fn from(data: Value) -> Self {
        Self(data)
    }
}

impl From<RawFlowRecord> for Value {
    fn from(value: RawFlowRecord) -> Self {
        value.0
    }
}