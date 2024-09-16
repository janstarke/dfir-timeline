# flow-record
Library for the creation of DFIR timelines, to be used by [`rdump`](https://docs.dissect.tools/en/latest/tools/rdump.html)

## Usage

```rust
use serde::Serialize;
use chrono::prelude::*;
use flow_record::{Serializer, Record};
use flow_record_derive::Record;

#[derive(Serialize, Record)]
struct SampleStruct {
    int_value: u32,
    str_value: String,
    dtm_value: DateTime<Utc>
}

let sample_struct = SampleStruct {
    int_value: 42,
    str_value: "forty two".into(),
    dtm_value: Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap(),
};

let mut ser = Serializer::new(Vec::new());
ser.serialize(&sample_struct).unwrap();

let result = ser.into_inner();
assert_eq!(result, vec![
    // header length
    0x0,0x0,0x0,0xf,

    // header
    0xc4,0xd,0x52,0x45,0x43,0x4f,0x52,0x44,0x53,0x54,0x52,0x45,0x41,0x4d,0xa,

    // length of record descriptor
    0x0,0x0,0x0,0x4c,

    0xc7,0x49,0xe,      // ext 8 record of type 0x0e
        0x92,           // array of length 2
            0x2,        // 2 (record desciptor)
            0x92,       // array of length 2
                0xac,   // string of length 12
                // "SampleStruct"
                0x53,0x61,0x6d,0x70,0x6c,0x65,0x53,0x74,0x72,0x75,0x63,0x74,
                0x93,   // array of length 3
                    0x92,   // array of length 2
                        0xa6, // string of length 6
                            0x75,0x69,0x6e,0x74,0x33,0x32, // "uint32"
                        0xa9, // string of length 9
                            0x69,0x6e,0x74,0x5f,0x76,0x61,0x6c,0x75,0x65, // "int_value"
                    0x92,
                        0xa6, // string of length 6
                            0x73,0x74,0x72,0x69,0x6e,0x67,
                        0xa9,0x73,0x74,0x72,0x5f,0x76,0x61,0x6c,0x75,0x65,
                    0x92,
                        0xa8,
                            0x64,0x61,0x74,0x65,0x74,0x69,0x6d,0x65,
                        0xa9,
                            0x64,0x74,0x6d,0x5f,0x76,0x61,0x6c,0x75,0x65,
    0x0,0x0,0x0,0x3a, // length of record
    0xc7,0x37,0xe,
        0x92,
            0x1,
            0x92,
                0x92,
                    0xac,
                        0x53,0x61,0x6d,0x70,0x6c,0x65,0x53,0x74,0x72,0x75,0x63,0x74,
                    0xce, // uint32
                        0x6,0xd6,0x49,0xca,
                    0x93, // array of length 3
                        0x2a,
                        0xa9,   // string of length 9
                            0x66,0x6f,0x72,0x74,0x79,0x20,0x74,0x77,0x6f,
                        0xb4,   // string of length 20
                                // "2020-01-01T00:00:00Z"
                            0x32,0x30,0x32,0x30,0x2d,0x30,0x31,0x2d,0x30,0x31,0x54,0x30,0x30,0x3a,0x30,0x30,0x3a,0x30,0x30,0x5a,]);
```

## Record format

Basically, the [record format](https://github.com/fox-it/flow.record) uses [MessagePack](https://github.com/msgpack/msgpack), with some extensions:

 - a list of records is preceded by a header
 - every record is preceded by its size (as 32bit integer in network byte order)

### Header

The header is formed by the serialized version of the string `RECORDSTREAM\n`, preceded by the header size:

```
       length of the string "RECORDSTREAM\n" ----+
                                                 |
                  msgpack type bin8 ----+        |
                                        |        |
0                                   31  v        v                     63
┌───────────────────────────────────┬────────┬────────┬────────┬────────┐
│   0x0000000f (header size in BE)  │  0xc4  │  0x0d  │   R    │   E    │
├────────┬────────┬────────┬────────┼────────┼────────┼────────┼────────┤
│   C    │   O    │   R    │   D    │   S    │   T    │   R    │   E    │
├────────┼────────┼────────┼────────┴────────┴────────┴────────┴────────┘
│   A    │   M    │  0x0a  │
└────────┴────────┴────────┘
```

In the following description I omit the fact that every distinct record and every descriptor must be preceded by its length.

### Objects

All data in the record format are specified as an *object*, which is simply a tuple (a *fixarray* of length 2) consisting of an object type and the object data. The following object ids are known:

|Object ID|Raw value|Description|
|-|-|-|
|RecordPackTypeRecord | `0x1` | |
|RecordPackTypeDescriptor | `0x2` | a [record descriptor](#descriptor) |
|RecordPackTypeFieldtype | `0x3` | |
|RecordPackTypeDatetime | `0x10` | |
|RecordPackTypeVarint | `0x11` | |
|RecordPackTypeGroupedrecord | `0x12` | |

### Descriptor

Every record must have some certain type, which must be specified using a *record descriptor* first. A record descriptor is an [object](#objects) of type `RecordPackTypeDescriptor`, which is wrapped as an msgpack `ext8` (depending on its size). The msgpack type id is `0x0e`.

Consider the following type:

```rust
struct test_csv_test1 {
       field11: String,
       field12: String,
       field13: String
}
```

which will the following msgpack encoding:

| raw value | explanation |
|-|-|
| `0xc7` | This is an ext8 record |
| `0x43` | length of the containing data |
| `0x0e` | marker for `rdump` that this contains an [object](#objects)

The object itself will be the msgpack encoded equivalent of the following data:

```json
[
	2,
	[
		"test_csv_test1",
		[
			[
				"string",
				"field11"
			],
			[
				"string",
				"field12"
			],
			[
				"string",
				"field13"
			]
		]
	]
]
```

It is important to note that every field is encoded as a tuple where the first entry is the datatype, and the second is the field name. The following datatypes are supported:

| Datatype | Mapped from Rust type | Explanation |
|-|-|-|
|`boolean`| `bool`
|`command`|
|`dynamic`|
|`datetime`| `DateTime<Utc>`
|`filesize`|
|`uint16`| `u8`, `u16`
|`uint32`|`u32`
|`float`|
|`string`|`String`
|`stringlist`|
|`dictlist`|
|`unix_file_mode`|
|`varint`| `i64` 
|`wstring`|
|`net.ipv4.Address`|
|`net.ipv4.Subnet`|
|`net.tcp.Port`|
|`net.udp.Port`|
|`uri`|
|`digest`|
|`bytes`|
|`record`|
|`net.ipaddress`|
|`net.ipnetwork`|
|`net.IPAddress`|
|`net.IPNetwork`|
|`path`|


### Record data# flow-record



License: GPL-3.0
