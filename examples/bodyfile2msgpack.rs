use std::io::{stdout, Write};

use bodyfile::Bodyfile3Line;
use flow_record::artifacts::PosixFileRecord;
use rmp_serde::Serializer;
use serde::Serialize;

fn main() {
    let sample_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxrwxrwx|0|0|92|1577092511|1577092511|1577092511|-1";
    let bf_line = Bodyfile3Line::try_from(sample_line).unwrap();
    let record = PosixFileRecord::try_from(&bf_line).unwrap();
    let mut ser = Serializer::new(stdout());

    std::io::stdout().write_all(b"\x00\x00\x00\x0f\xc4\x0dRECORDSTREAM\n").unwrap();
    record.serialize(&mut ser).unwrap();
}
