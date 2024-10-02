use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use rmpv::Value;
use strum::Display;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Display)]
#[allow(dead_code)]
#[strum(serialize_all = "snake_case")]
pub enum FieldType {
    Bool,

    #[strum(to_string="uint16")]
    UInt16,

    #[strum(to_string="uint32")]

    UInt32,
    #[strum(to_string="varint")]
    VarInt,
    Float,
    String,
    Bin,
    Datetime,
    Filesize,
    Path,
    UnixFileMode,
    Record
}

impl From<FieldType> for Value {
    fn from(value: FieldType) -> Self {
        Value::String(value.to_string().into())
    }
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(quote! {flow_record::prelude::#self});
    }
}
