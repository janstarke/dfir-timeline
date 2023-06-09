# dfir-timeline
Library for the creation of DFIR timelines

## Record format

Basically, the [record format](https://github.com/fox-it/flow.record) uses [MessagePack](https://github.com/msgpack/msgpack), with some extensions:

 - a list of records is preceded by a header
 - every record is preceded by its size (as 32bit integer in network byte order)

### Header

The header is formed by the serialized version of the string `RECORDSTREAM\n`, preceded by the header size:

```
 63                                 31               15                0
┌───────────────────────────────────┬───────────────────────────────────┐
│   0x0000000f (header size in BE)  │  0xc4  │  0x0d  │   R    │   E    │
├────────┼────────┼────────┼────────┼────────┬────────┬────────┬────────┤
│   C    │   O    │   R    │   D    │   S    │   T    │   R    │   E    │
├────────┼────────┼────────┼────────┴────────┴────────┴────────┴────────┘
│   A    │   M    │  0x0a  │
└────────┴────────┴────────┘
```

### Descriptor

### Record data