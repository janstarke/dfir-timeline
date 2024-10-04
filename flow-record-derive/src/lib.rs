use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use record_attributes::RecordAttributes;
use struct_info::StructInfo;
use syn::{parse_macro_input, DeriveInput};

mod field_info;
mod record_attributes;
mod struct_info;
mod without_lifetimes;

#[proc_macro_derive(FlowRecord, attributes(flow_record))]
pub fn derive_flow_record(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    FromDeriveInput::from_derive_input(&input)
        .and_then(|attrs| expand(&mut input, attrs))
        .map(Into::into)
        // Error handling
        .unwrap_or_else(|e| e.write_errors().into())
}

fn expand(ast: &mut syn::DeriveInput, attrs: RecordAttributes) -> darling::Result<TokenStream> {
    let name = &ast.ident;
    let name_as_string = name.to_string();

    let descriptor;
    let child_descriptors;
    let hash;
    let values: Vec<_>;

    let from_parameter_name = quote! {self};

    match &ast.data {
        syn::Data::Struct(s) => {
            let struct_info = StructInfo::new(name_as_string.clone(), s.clone(), attrs);
            descriptor = struct_info.descriptor();
            child_descriptors = struct_info.child_descriptors();
            hash = struct_info.descriptor_hash();
            values = struct_info.values(quote! {#from_parameter_name}).collect();
        }

        syn::Data::Enum(_) => panic!("no support for enums yet"),
        syn::Data::Union(_) => panic!("no support for unions yet"),
    }

    let children = child_descriptors.into_iter().map(|c| {
        let (hash, descriptor) = (c.hash, c.descriptor);
        quote! {#hash, #descriptor}
    });

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote!(
        use flow_record::prelude::rmpv::Value;

        impl #impl_generics FlowRecord for #name #ty_generics #where_clause {
            fn name() -> &'static str {
                #name_as_string
            }
            fn descriptor() -> &'static Value {
                static D: std::sync::LazyLock<Value> = std::sync::LazyLock::new(|| Value::from( #descriptor ) );
                &*D
            }
            fn descriptor_hash() -> u32 {
                static H: std::sync::LazyLock<u32> = std::sync::LazyLock::new(|| #hash);
                *H
            }
            fn into_value(self) -> Value {
                Value::Array(vec![
                    #(#values),*,
                ])
            }
            fn child_descriptors() -> &'static ::std::collections::HashMap<u32, Value> {
                static D: std::sync::LazyLock<::std::collections::HashMap<u32, Value>> = std::sync::LazyLock::new(||
                    {
                        let mut d = ::std::collections::HashMap::new();
                        #( d.insert( #children ); )*
                        d
                    }
                );
                &*D
            }
        }
    );
    Ok(gen.into())
}

#[proc_macro_attribute]
pub fn has_descriptor(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    annotated_item
}
