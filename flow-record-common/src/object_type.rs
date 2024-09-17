use rmpv::Value;

#[derive(Clone, Copy)]
#[repr(i8)]
#[allow(unused)]
pub enum ObjectType {
    RecordTypeExt = 0x0e,

    RecordPackTypeRecord = 0x1,
    RecordPackTypeDescriptor = 0x2,
    RecordPackTypeFieldtype = 0x3,
    RecordPackTypeDatetime = 0x10,
    RecordPackTypeVarint = 0x11,
    RecordPackTypeGroupedrecord = 0x12,
}

impl From<ObjectType> for Value {
    fn from(value: ObjectType) -> Self {
        Value::Integer((value as i8).into())
    }
}
