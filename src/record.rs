use serde::Serialize;

pub trait Record: Serialize {
    fn descriptor() -> &'static [u8];
    fn descriptor_hash() -> u64;
}