use apache_avro::schema::AvroSchema;
use dfir_timeline::{artifacts::PosixFile, TimelineRecord};

fn main() {
    let schema = TimelineRecord::<PosixFile>::get_schema();
    println!("{}", schema.canonical_form());
}