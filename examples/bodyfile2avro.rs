use std::io::{stdout, Write};

use bodyfile::Bodyfile3Line;
use apache_avro::{schema::AvroSchema, Writer};
use dfir_timeline::{artifacts::PosixFile, TimelineRecord};

fn main() {
    let sample_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
    let bf_line = Bodyfile3Line::try_from(sample_line).unwrap();
    let schema = TimelineRecord::<PosixFile>::get_schema();

    let mut writer = Writer::new(&schema, Vec::new());
    for record in TimelineRecord::<PosixFile>::iter_records_from(bf_line).unwrap() {
        writer.append_ser(record).unwrap();
    }
    let encoded = writer.into_inner().unwrap();
    let _ = stdout().write(&encoded[..]);
}