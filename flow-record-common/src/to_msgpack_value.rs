use chrono::{DateTime, TimeZone};

pub trait ToMsgPackValue {
    fn to_msgpack_value(self) -> rmpv::Value;
}

impl<T> ToMsgPackValue for Option<T> where T: ToMsgPackValue {
    fn to_msgpack_value(self) -> rmpv::Value {
        match self {
            Some(v) => v.to_msgpack_value(),
            None => rmpv::Value::Nil,
        }
    }
}

impl<Tz> ToMsgPackValue for DateTime<Tz> where Tz: TimeZone {
    fn to_msgpack_value(self) -> rmpv::Value {
        self.timestamp().into()
    }
}

impl ToMsgPackValue for String {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::String(self.into())
    }
}

impl ToMsgPackValue for &str {
    fn to_msgpack_value(self) -> rmpv::Value {
        rmpv::Value::String(self.into())
    }
}

macro_rules! to_msgpack_value_for_integer {
    ($type: ty) => {
        impl ToMsgPackValue for $type {
            fn to_msgpack_value(self) -> rmpv::Value {
                rmpv::Value::Integer(self.into())
            }
        }
        impl ToMsgPackValue for &$type {
            fn to_msgpack_value(self) -> rmpv::Value {
                rmpv::Value::Integer((*self).into())
            }
        }
    };
}

to_msgpack_value_for_integer!(u8);
to_msgpack_value_for_integer!(u16);
to_msgpack_value_for_integer!(u32);
to_msgpack_value_for_integer!(u64);

to_msgpack_value_for_integer!(i8);
to_msgpack_value_for_integer!(i16);
to_msgpack_value_for_integer!(i32);
to_msgpack_value_for_integer!(i64);