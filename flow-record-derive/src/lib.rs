use flow_record_common::{FieldType, Object, RecordDescriptor, RecordField};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::Write;

use proc_macro::TokenStream;
use quote::quote;
use syn::Type;

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
    let name_as_string = name.to_string();

    let descriptor;
    let hash;

    match ast.data {
        syn::Data::Struct(s) => {
            descriptor = struct_descriptor(&name_as_string, &s);
            hash = calc_descriptor_hash(&name_as_string, &s);
        }

        syn::Data::Enum(_) => panic!("no support for enums yet"),
        syn::Data::Union(_) => panic!("no support for unions yet"),
    }

    let length = descriptor.len();

    let gen = quote!(
        impl Record for #name {
            fn name() -> &'static str {
                #name_as_string
            }
            fn descriptor() -> &'static [u8] {
                static d: [u8; #length] = [ #(#descriptor),* ];
                &d
            }
            fn descriptor_hash() -> u32 {
                #hash
            }
        }
    );
    gen.into()
}

fn calc_descriptor_hash(name: &str, s: &syn::DataStruct) -> u32 {
    let mut hasher = Sha256::new();
    hasher.write_all(name.as_bytes()).unwrap();

    if let Some(fields) = record_fields_from(s) {
        for field in fields {
            hasher.write_all(field.field_name().as_bytes()).unwrap();
            hasher
                .write_all(field.field_type().to_string().as_bytes())
                .unwrap();
        }
    }
    let hash = hasher.finalize();
    u32::from_be_bytes(hash[0..4].try_into().unwrap())
}

fn struct_descriptor(name: &str, s: &syn::DataStruct) -> Vec<u8> {
    if let Some(fields) = record_fields_from(s) {
        let mut buffer: Vec<u8> = Vec::new();
        let mut ser = rmp_serde::Serializer::new(&mut buffer);
        Object::with_descriptor(&RecordDescriptor::new(name.to_owned(), fields))
            .serialize(&mut ser)
            .unwrap();
        buffer
    } else {
        unimplemented!()
    }
}

fn record_fields_from(s: &syn::DataStruct) -> Option<Vec<RecordField>> {
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
            Some(fields)
        }
        _ => None,
    }
}
