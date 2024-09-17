use flow_record_common::{FieldType, RecordField};
use proc_macro2::TokenStream;
use quote::quote;
use sha2::{Digest, Sha256};
use std::io::Write;
use syn::Type;

use crate::field_info::FieldInfo;

pub fn expand_derive_serialize(ast: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let name_as_string = name.to_string();

    let descriptor;
    let hash;
    let values: Vec<_>;

    let from_parameter_name = quote! {self};

    match &ast.data {
        syn::Data::Struct(s) => {
            descriptor = struct_descriptor(&name_as_string, &s);
            hash = calc_descriptor_hash(&name_as_string, &s);
            values = record_value_tokens(&s, quote! {#from_parameter_name}).collect();
        }

        syn::Data::Enum(_) => panic!("no support for enums yet"),
        syn::Data::Union(_) => panic!("no support for unions yet"),
    }

    let gen = quote!(
        use rmpv::Value;

        impl Record for #name {
            fn name() -> &'static str {
                #name_as_string
            }
            fn descriptor() -> &'static Value {
                static D: std::sync::LazyLock<Value> = std::sync::LazyLock::new(|| Value::from(#descriptor));
                &*D
            }
            fn descriptor_hash() -> u32 {
                #hash
            }
            fn into_value(self) -> Value {
                Value::Array(vec![
                    Value::Array(vec![
                        Value::String(Self::name().into()),
                        Value::Integer(Self::descriptor_hash().into()),
                    ]),
                    Value::Array(vec![
                        #(#values),*
                    ])
                ])
            }
        }
    );
    Ok(gen.into())
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

fn struct_descriptor(name: &str, s: &syn::DataStruct) -> proc_macro2::TokenStream {
    let fields = record_field_tokens(s);
    quote! {
        flow_record_common::RecordDescriptor::new(#name.into(), vec![#(#fields),*])
    }
}

fn record_field_tokens(s: &syn::DataStruct) -> impl Iterator<Item = proc_macro2::TokenStream> {
    record_fields_from(s)
        .unwrap_or_default()
        .into_iter()
        .map(|rf| {
            let (field_name, field_type) = rf.dissolve();
            quote! {flow_record_common::RecordField::from((#field_name.into(), #field_type))}
        })
}

fn record_value_tokens(
    s: &syn::DataStruct,
    from_parameter_name: proc_macro2::TokenStream,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    fn value_expr(expr: proc_macro2::TokenStream, ft: FieldType) -> proc_macro2::TokenStream {
        match ft {
            flow_record_common::FieldType::Datetime => quote! {#expr.timestamp().into()},
            _ => quote!{#expr.into()}
        }
    }

    record_fields(s).map(move |(ident, _, info)| {
        let dst_type = match info.ft {
            flow_record_common::FieldType::Bool => quote! {rmpv::Value::Boolean},
            flow_record_common::FieldType::UInt16 => quote! {rmpv::Value::Integer},
            flow_record_common::FieldType::UInt32 => quote! {rmpv::Value::Integer},
            flow_record_common::FieldType::VarInt => quote! {rmpv::Value::Integer},
            flow_record_common::FieldType::Float => quote! {rmpv::Value::F64},
            flow_record_common::FieldType::String => quote! {rmpv::Value::String},
            flow_record_common::FieldType::Bin => quote! {rmpv::Value::Binary},
            flow_record_common::FieldType::Datetime => quote! {rmpv::Value::Integer},
        };

        if info.is_nullable {
            let var_name = quote! {val};
            let expr = value_expr(var_name.clone(), info.ft);
            quote! {
                match #from_parameter_name.#ident {
                    None => Value::Nil,
                    Some(#var_name) => #dst_type(#expr)
                }
            }
        } else {
            let expr = value_expr(quote! {#from_parameter_name.#ident}, info.ft);
            quote! {#dst_type(#expr)}
        }
    })
}

fn record_fields_from(s: &syn::DataStruct) -> Option<Vec<RecordField>> {
    match &s.fields {
        syn::Fields::Named(n) => {
            let fields: Vec<_> = n
                .named
                .iter()
                .map(|f| {
                    let field_name = f.ident.as_ref().unwrap().to_string();
                    let field_type = FieldInfo::from(&f.ty).ft;
                    flow_record_common::RecordField::from((field_name, field_type))
                })
                .collect();
            Some(fields)
        }
        _ => None,
    }
}

fn record_fields(
    s: &syn::DataStruct,
) -> impl Iterator<Item = (proc_macro2::Ident, Type, FieldInfo)> + '_ {
    match &s.fields {
        syn::Fields::Named(n) => n.named.iter().map(|f| {
            (
                f.ident.as_ref().unwrap().clone(),
                f.ty.clone(),
                FieldInfo::from(&f.ty),
            )
        }),
        _ => unimplemented!(),
    }
}
