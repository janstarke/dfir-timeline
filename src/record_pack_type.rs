pub enum RecordPackType {
    RecordPackExtType = 0xE,
    RecordPackTypeRecord = 0x1,
    RecordPackTypeDescriptor = 0x2,
    RecordPackTypeFieldtype = 0x3,
    RecordPackTypeDatetime = 0x10,
    RecordPackTypeVarint = 0x11,
    RecordPackTypeGroupedrecord = 0x12,
}
