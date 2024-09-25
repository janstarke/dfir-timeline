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
    UnixFileMode
}
/*
impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FieldType::Bool => "boolean",
                FieldType::UInt16 => "uint16",
                FieldType::UInt32 => "uint32",
                FieldType::VarInt => "varint",
                FieldType::Float => "float",
                FieldType::String => "string",
                FieldType::Bin => "bin",
                FieldType::Datetime => "datetime",
                FieldType::Filesize => "filesize",
                FieldType::Path => "path",
                FieldType::UnixFileMode => "unix_file_mode",
            }
        )
    }
}
     */

impl From<FieldType> for Value {
    fn from(value: FieldType) -> Self {
        Value::String(value.to_string().into())
    }
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let token = match self {
            FieldType::Bool => quote! {FieldType::Bool},
            FieldType::UInt16 => quote! {FieldType::UInt16},
            FieldType::UInt32 => quote! {FieldType::UInt32},
            FieldType::VarInt => quote! {FieldType::VarInt},
            FieldType::Float => quote! {FieldType::Float},
            FieldType::String => quote! {FieldType::String},
            FieldType::Bin => quote! {FieldType::Bin},
            FieldType::Datetime => quote! {FieldType::Datetime},
            FieldType::Filesize => quote! {FieldType::Filesize},
            FieldType::Path => quote! {FieldType::Path},
            FieldType::UnixFileMode => quote! {FieldType::UnixFileMode},
        };
        tokens.append_all(quote! {flow_record_common::#token});
    }
}
