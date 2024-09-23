use std::io::stdout;

use bodyfile::Bodyfile3Line;
use flow_record::{artifacts::posix::FileRecord, Serializer};

fn main() {
    let sample_line = "0|/Users/Administrator ($FILE_NAME)|93552-48-2|d/drwxr-xr-x|0|0|92|1577092511|1577092511|1577092511|-1";
    let bf_line = Bodyfile3Line::try_from(sample_line).unwrap();
    let record = FileRecord::try_from(&bf_line).unwrap();
    let mut ser = Serializer::new(stdout());

    ser.serialize(record).unwrap();
}
