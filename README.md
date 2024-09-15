# dfir-timeline
Library for the creation of DFIR timelines

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

|Datatype | Explanation |
|-|-|
|`boolean`|
|`command`|
|`dynamic`|
|`datetime`|
|`filesize`|
|`uint16`|
|`uint32`|
|`float`|
|`string`|
|`stringlist`|
|`dictlist`|
|`unix_file_mode`|
|`varint`|
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


### Record data