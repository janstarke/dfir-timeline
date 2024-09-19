use chrono::{DateTime, TimeZone};

use crate::FieldType;

pub trait ToMsgPackValue {
    fn to_msgpack_value(self) -> rmpv::Value;
    fn field_type() -> FieldType;
}

impl<T> ToMsgPackValue for Option<T> where T: ToMsgPackValue {
    fn to_msgpack_value(self) -> rmpv::Value {
        match self {
            Some(v) => v.to_msgpack_value(),
            None => rmpv::Value::Nil,
        }
    }
    
    fn field_type() -> FieldType {
        T::field_type()
    }
}

impl<Tz> ToMsgPackValue for DateTime<Tz> where Tz: TimeZone {
    fn to_msgpack_value(self) -> rmpv::Value {
        self.timestamp().into()
    }
    
    fn field_type() -> FieldType {
        FieldType::Datetime
    }
}

impl ToMsgPackValue for String {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::String(self.into())
    }
    
    fn field_type() -> FieldType {
        FieldType::String
    }
}

impl ToMsgPackValue for &str {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::String(self.into())
    }
    
    fn field_type() -> FieldType {
        FieldType::String
    }
}

impl ToMsgPackValue for Vec<u8> {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::Binary(self.into())
    }

    fn field_type() -> FieldType {
        FieldType::Bin
    }
}

macro_rules! to_msgpack_value_for {
    ($dst: expr, $type: ty, $field_type: expr) => {
        impl ToMsgPackValue for $type {
            fn to_msgpack_value(self) -> rmpv::Value {
                $dst(self.into())
            }

            fn field_type() -> FieldType {
                $field_type
            }
        }
        impl ToMsgPackValue for &$type {
            fn to_msgpack_value(self) -> rmpv::Value {
                $dst((*self).into())
            }

            fn field_type() -> FieldType {
                $field_type
            }
        }
    };
}

macro_rules! to_msgpack_value_for_integer {
    ($type: ty, $field_type: expr) => {to_msgpack_value_for!(rmpv::Value::Integer, $type, $field_type);}
}

to_msgpack_value_for_integer!(u8, FieldType::UInt16);
to_msgpack_value_for_integer!(u16, FieldType::UInt16);
to_msgpack_value_for_integer!(u32, FieldType::UInt32);
to_msgpack_value_for_integer!(u64, FieldType::VarInt);

to_msgpack_value_for_integer!(i8, FieldType::VarInt);
to_msgpack_value_for_integer!(i16, FieldType::VarInt);
to_msgpack_value_for_integer!(i32, FieldType::VarInt);
to_msgpack_value_for_integer!(i64, FieldType::VarInt);

to_msgpack_value_for!(rmpv::Value::F32, f32, FieldType::Float);
to_msgpack_value_for!(rmpv::Value::F64, f64, FieldType::Float);