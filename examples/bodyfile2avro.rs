use std::io::{stdout, Write};

use bodyfile::Bodyfile3Line;
use apache_avro::{schema::AvroSchema, Writer};
use dfir_timeline::{artifacts::PosixFile, TimelineRecord};

fn main() {
    let sample_line = "0|/wusagedl.exe|0|6|33279|-/-rwxrwxrwx|1|0|0|0|3827200|1220846400|1216831874|1216831874|512|0";
    let bf_line = Bodyfile3Line::try_from(sample_line).unwrap();
    let record = TimelineRecord::try_from(bf_line).unwrap();
    let schema = TimelineRecord::<PosixFile>::get_schema();

    let mut writer = Writer::new(&schema, Vec::new());
    writer.append_ser(record).unwrap();
    let encoded = writer.into_inner().unwrap();
    let _ = stdout().write(&encoded[..]);
}