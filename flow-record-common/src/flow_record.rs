use rmpv::Value;


pub trait FlowRecord {
    fn name() -> &'static str;
    fn descriptor() -> &'static Value;
    fn descriptor_hash() -> u32;
    fn into_value(self) -> Value;
}