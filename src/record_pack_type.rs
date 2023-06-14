pub enum RecordPackType {
    RecordPackExtType = 0x0E,
    RecordPackTypeRecord = 0x01,
    RecordPackTypeDescriptor = 0x02,
    RecordPackTypeFieldtype = 0x03,
    RecordPackTypeDatetime = 0x10,
    RecordPackTypeVarint = 0x11,
    RecordPackTypeGroupedrecord = 0x12,
}
