use serde::Serialize;


pub trait Record: Serialize {
    fn name() -> &'static str;
    fn descriptor() -> &'static [u8];
    fn descriptor_hash() -> u32;
}
