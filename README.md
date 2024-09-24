<img width="100%" src="https://raw.githubusercontent.com/janstarke/flow-record/main/docs/img/flow-record-header.png"></img> 

[![Crates.io](https://img.shields.io/crates/v/flow-record)](https://crates.io/crates/flow-record)
![Crates.io](https://img.shields.io/crates/l/flow-record)
![Crates.io (latest)](https://img.shields.io/crates/dv/flow-record)

# flow-record
Library for the creation of DFIR timelines, to be used by [`rdump`](https://docs.dissect.tools/en/latest/tools/rdump.html)



## Usage

```rust
use binrw::BinReaderExt;
use chrono::prelude::*;
use flow_record::{FlowRecord, RecordPack, Record, Serializer, RECORDSTREAM_MAGIC};
use flow_record_derive::Record;
use std::io::{Cursor,Seek,SeekFrom};

#[derive(Record)]
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
ser.serialize(sample_struct).unwrap();

```

That's basically all. The next steps are only necessary to validate
if all data were written correctly. You can ignore this, if you just want
to export the binary data.

```rust
// omit the header
raw_data.seek(SeekFrom::Start((4+2+RECORDSTREAM_MAGIC.len()).try_into().unwrap()));

let descriptor_record: FlowRecord = raw_data.read_be().unwrap();
let data_record: FlowRecord = raw_data.read_be().unwrap();

let descriptor = RecordPack::try_from(Value::from(descriptor_record)).unwrap();
let data = RecordPack::try_from(Value::from(data_record)).unwrap()
                .inner().clone();

assert_eq!(data,
       Value::Array(vec![
           Value::Integer(1.into()), // record pack type
           Value::Array(vec![
               Value::Array(vec![    // reference to record descriptor
                   Value::String("SampleStruct".into()),  // struct name
                   Value::Integer(114706890.into())       // struct hash
               ]),
               Value::Array(vec![   // actual data
                   Value::Integer(42.into()),
                   Value::String("forty two".into()),
                   Value::Integer(1577836800.into())
               ])
           ])
       ]));
```


# Record flow format

Basically, the [record format](https://github.com/fox-it/flow.record) uses [MsgPack](https://github.com/msgpack/msgpack). A record stream is a sequence of tuples, each containing of a 4 byte size field and a msgpack encoded [record pack](#record-packs) (see below).

The very first of those tuples is a special case; it is some kind of a header.

```text
                                ┌────────[record size] bytes───────┐ 
                               /                                    \
┌──────────────────────────────┬────────────────────────────────────┐
│record size (32bit big endian)│     msgpack encoded content        │
└──────────────────────────────┴────────────────────────────────────┘
```

# Header

The header is formed by the serialized version of the string `RECORDSTREAM\n`, encoded as `bin8`:

```text
   ┌───────────────msgpack type: bin8
   │                                 
   │    ┌──────────length: 13 bytes  
   │    │                            
   │    │    ┌─────13 bytes of data  
   ▼    ▼    ▼                       
┌────┬────┬──────────────┐           
│0xc4│0x0d│RECORDSTREAM\n│           
└────┴────┴──────────────┘           
```

In the following description I omit the fact that every distinct record and every descriptor must be preceded by its length.

# Record packs

All data in the record format are specified as a *record pack*, which is simply a tuple (a *fixarray* of length 2) consisting of an record pack type and the record pack data.

```
   ┌──────────────────────────── msgpack type ext8/ext16/ext32
   │    ┌─────────────────────── length of content            
   │    │    ┌────────────────── type id must be 0x0e         
   │    │    │     ┌──────────── array of length 2            
   │    │    │     │    ┌─────── record pack type             
   ▼    ▼    ▼     │    │    ┌── record pack data                      
┌────┬────┬────┬───┼────┼────┼──────────────────────────      
│    │    │    │   ▼    ▼    ▼                                
│    │    │    │┌────┬────┬──────────────────                 
│0xc7│    │0x0e││0x92│    │                                   
│    │    │    │└────┴────┴──────────────────                 
│    │    │    │                                              
└────┴────┴────┴────────────────────────────────────────      
```

The following record pack types are known:

|Object ID|Raw value|Description|
|-|-|-|
|RecordPackTypeRecord | `0x1` | |
|RecordPackTypeDescriptor | `0x2` | a [record descriptor](#descriptor) |
|RecordPackTypeFieldtype | `0x3` | |
|RecordPackTypeDatetime | `0x10` | |
|RecordPackTypeVarint | `0x11` | |
|RecordPackTypeGroupedrecord | `0x12` | |

## Descriptor

Every record must have some certain type, which must be specified using a *record descriptor* first. A record descriptor is a [record pack](#record-packs) of type `RecordPackTypeDescriptor`, which is wrapped as an msgpack `ext8` (depending on its size). The msgpack type id is `0x0e`.

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

The record pack itself will be the msgpack encoded equivalent of the following data:

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
|`datetime`| `DateTime<TZ: TimeZone>` | UNIX timestamp, encoding as integer in msgpack|
|`filesize`|`flow-record-common::Filesize`|
|`uint16`| `u8`, `u16`|
|`uint32`| `u32`, `u64`|
|`float`|`f32`, `f64`|
|`string`|`String`|
|`stringlist`|
|`dictlist`|
|`unix_file_mode`| `String` | the `chmod` formatted string will be converted to u16 internally
|`varint`| `i8`,`i16`, `i32`, `i64` |
|`wstring`|
|`net.ipv4.Address`|
|`net.ipv4.Subnet`|
|`net.tcp.Port`|
|`net.udp.Port`|
|`uri`|
|`digest`|
|`bytes`|`Vec<u8>`|
|`record`|
|`net.ipaddress`|
|`net.ipnetwork`|
|`net.IPAddress`|
|`net.IPNetwork`|
|`path`|`PathBuf`|

### Identifier

A record descriptor is identified by

- the name of the record type
- a hash, which equals the first 32 bit of a SHA256-Hash of the record type name and the names and types (in that order) of the record fields. For example, the above struct would have the following input for the hash function:
  `test_csv_test1field11stringfield12stringfield13string`, which would result in the hash `12a9d8d90aa34e5068dbf6692b82baf6fff0143eeaa84d7b2a9c92021f7747c2`. Here we take the first 4 bytes `12a9d8d9`, interpret them as byte endian integer `313120985` and use this as hash.

Every remaining record can refer to a record descriptor using the name and hash of it.

## Record data

A record contains a reference to the record descriptor and a list of values, in the order of fields like specified in the descriptor.

```
        ┌───────────────────────────────────────────────────────────── record pack type 1           
        │                     ┌─────────────────────────────────────── name of the descriptor       
        │                     │                   ┌─────────────────── hash of the descriptor       
        │                     │                   │       ┌─────────── array of values              
        ▼                     │                   │       │   ┌─────── value of the first data field
┌────┬────┬───────────────────┼───────────────────┼───────┼───┼───────────────────────────          
│    │    │                   │                   │       │   │                                     
│    │    │┌────┬─────────────┼───────────────────┼───┬───┼───┼───────────────────────────          
│    │    ││    │             ▼                   ▼   │   ▼   ▼                                     
│    │    ││    │┌────┬────┬─────────────┬────┬──────┐│┌────┬─────────┬─────────┬─────────          
│0x92│0x01││0x92││0x92│0xa?│<struct name>│0xce│<hash>│││0x9?│<field 1>│<field 2>│...                
│    │    ││    │└────┴────┴─────────────┴────┴──────┘│└────┴─────────┴─────────┴─────────          
│    │    ││    │                                     │                                             
│    │    │└────┴─────────────────────────────────────┴───────────────────────────────────          
│    │    │                                                                                         
└────┴────┴───────────────────────────────────────────────────────────────────────────────          
```
License: GPL-3.0
