pub trait HasRecordDescriptor {
    fn descriptor() -> &'static [u8];
    fn descriptor_hash() -> u64;
}