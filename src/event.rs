use apache_avro::{AvroSchema};

pub trait Event: AvroSchema {}
pub trait EventData: AvroSchema {
    type EventType: Event;
}