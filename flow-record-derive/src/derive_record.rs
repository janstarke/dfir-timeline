use proc_macro2::TokenStream;
use quote::quote;

use crate::struct_info::StructInfo;

pub fn expand_derive_serialize(ast: &mut syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let name_as_string = name.to_string();

    let descriptor;
    let hash;
    let values: Vec<_>;

    let from_parameter_name = quote! {self};

    match &ast.data {
        syn::Data::Struct(s) => {
            let struct_info = StructInfo::new(name_as_string.clone(), s);
            descriptor = struct_info.descriptor();
            hash = struct_info.descriptor_hash();
            values = struct_info.values(quote! {#from_parameter_name}).collect();
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
                static H: std::sync::LazyLock<u32> = std::sync::LazyLock::new(|| #hash);
                *H
            }
            fn into_value(self) -> Value {
                Value::Array(vec![
                    #(#values),*,
                ])
            }
        }
    );
    Ok(gen.into())
}
