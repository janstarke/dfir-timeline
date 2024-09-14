use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use proc_macro::TokenStream;
use quote::quote;
use serde::Serialize;
use syn::Type;

mod field_type;
mod record_descriptor;
mod record_field;
mod ext_type;

use field_type::*;
use record_descriptor::*;
use record_field::*;

fn to_field_type(ty: &Type) -> FieldType {
    let type_name = quote!(#ty).to_string().replace(' ', "");
    match &type_name[..] {
        "String" => FieldType::String,
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" => FieldType::Int,
        "DateTime<Utc>" => FieldType::Timestamp,
        "Option<DateTime<Utc>>" => FieldType::Timestamp,
        _ => unimplemented!(),
    }
}

#[proc_macro_derive(Record)]
pub fn recorddescriptor_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let descriptor = match ast.data {
        syn::Data::Struct(s) => struct_descriptor(name.to_string(), &s),
        syn::Data::Enum(_) => panic!("no support for enums yet"),
        syn::Data::Union(_) => panic!("no support for unions yet"),
    };

    let length = descriptor.len();
    let mut hasher = DefaultHasher::new();
    descriptor.hash(&mut hasher);
    let hash = hasher.finish();

    let gen = quote!(
        impl Record for #name {
            fn descriptor() -> &'static [u8] {
                static d: [u8; #length] = [ #(#descriptor),* ];
                &d
            }
            fn descriptor_hash() -> u64 {
                #hash
            }
        }
    );
    gen.into()
}

fn struct_descriptor(name: String, s: &syn::DataStruct) -> Vec<u8> {
    match &s.fields {
        syn::Fields::Named(n) => {
            let fields: Vec<_> = n
                .named
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap().to_string();
                    let field_type = to_field_type(&f.ty);
                    RecordField::from((field_name, field_type))
                })
                .collect();
            let mut buffer: Vec<u8> = Vec::new();
            let mut ser = rmp_serde::Serializer::new(&mut buffer);
            RecordDescriptor::new(name, fields).serialize(&mut ser).unwrap();
            buffer
        }
        syn::Fields::Unnamed(_) => unimplemented!(),
        syn::Fields::Unit => unimplemented!(),
    }
}
