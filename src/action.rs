use apache_avro::AvroSchema;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, AvroSchema)]
pub enum Action {
    Modified,

    Accessed,

    Changed,

    /// AKA birth
    Created, 

    Deleted,

    Renamed,
}