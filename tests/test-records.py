import sys

from flow.record import RecordDescriptor, RecordStreamWriter

TestRecord1 = RecordDescriptor(
    "test/csv_test1",
    [
        ("string", "field11"),
        ("string", "field12"),
        ("string", "field13"),
    ]
)

TestRecord2 = RecordDescriptor(
    "test/csv_test2",
    [
        ("string", "field21"),
        ("string", "field22"),
        ("string", "field23"),
    ]
)

rec1 = TestRecord1(
    field11 = "AB",
    field12 = "CD",
    field13 = "EF",
)

rec2 = TestRecord2(
    field21 = "PQ",
    field22 = "RS",
    field23 = "TU",
)

rec3 = TestRecord1(
    field11 = "K\\L",
    field12 = "M\\\\N",
    field13 = "O\\\"'PQR",
)

record_writer = RecordStreamWriter(sys.stdout.buffer)

record_writer.write(rec1)
record_writer.write(rec2)
record_writer.write(rec3)
