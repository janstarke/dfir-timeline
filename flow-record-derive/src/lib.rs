use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use serde::Serialize;

use proc_macro::TokenStream;
use quote::quote;
use syn::Type;

mod ext_data;
mod field_type;
mod object;
mod record_descriptor;
mod record_field;

use field_type::*;
use object::*;
use record_descriptor::*;
use record_field::*;
use ext_data::*;

fn to_field_type(ty: &Type) -> FieldType {
    let type_name = quote!(#ty).to_string().replace(' ', "");
    match &type_name[..] {
        "String" => FieldType::String,
        "u8" | "u16" => FieldType::UInt16,
        "u32" => FieldType::UInt32,
        "i64" => FieldType::VarInt,
        "DateTime<Utc>" => FieldType::Datetime,
        "Option<DateTime<Utc>>" => FieldType::Datetime,
        type_id => unimplemented!("no implementation for type '{type_id}' yet"),
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
            ExtData(Object::Descriptor(RecordDescriptor::new(name, fields)))
            .serialize(&mut ser)
            .unwrap();
            buffer
        }
        syn::Fields::Unnamed(_) => unimplemented!(),
        syn::Fields::Unit => unimplemented!(),
    }
}
