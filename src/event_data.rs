use apache_avro::{AvroSchema};

pub trait EventData: AvroSchema {}