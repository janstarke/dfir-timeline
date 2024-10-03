use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;

use crate::field_info::FieldInfo;
use crate::record_attributes::RecordAttributes;
use crate::without_lifetimes::WithoutLifetimes;

pub struct StructInfo {
    name: String,
    fields: Vec<FieldInfo>,
    attrs: RecordAttributes,
}

impl StructInfo {
    pub fn new(name: String, s: syn::DataStruct, attrs: RecordAttributes) -> Self {
        match s.fields {
            syn::Fields::Named(n) => {
                let fields: Vec<_> = n
                    .named
                    .iter()
                    .map(|f| {
                        let field_type = f.ty.clone().without_lifetimes();

                        let field_type_expr = quote! {
                            <#field_type as ::flow_record::prelude::ToMsgPackValue>::field_type()
                        };

                        FieldInfo::new(f.ident.as_ref().unwrap().to_string(), field_type_expr)
                    })
                    .collect();
                Self {
                    name,
                    fields,
                    attrs,
                }
            }
            _ => Self {
                name,
                fields: vec![],
                attrs,
            },
        }
    }

    pub fn descriptor(&self) -> TokenStream {
        let name = &self.name;
        let fields = self.fields.iter().map(|field_info| {
            let field_name = &field_info.name;
            let field_type = &field_info.field_type_expr;
            quote! {
                flow_record::prelude::RecordField::from(
                    (#field_name.into(), (#field_type)))
            }
        });
        quote! {
            flow_record::prelude::RecordDescriptor::new(#name.into(), vec![#(#fields),*])
        }
    }

    pub fn descriptor_hash(&self) -> TokenStream {
        let name = &self.name;
        let hasher = quote!(hasher);

        let fields = self.fields.iter().map(|field_info| {
            let field_name = &field_info.name;
            let field_type = &field_info.field_type_expr;
            quote! {
                #hasher.write_all(#field_name.as_bytes()).unwrap();
                #hasher.write_all(#field_type.to_string().as_bytes()).unwrap();
            }
        });

        quote! { {
            use std::io::Write;
            use flow_record::prelude::sha2::Digest;
            let mut #hasher = flow_record::prelude::sha2::Sha256::new();
            #hasher.write_all(#name.as_bytes()).unwrap();
            #(#fields);*
            let hash = #hasher.finalize();
            u32::from_be_bytes(hash[0..4].try_into().unwrap())
        }}
    }

    pub fn values(
        &self,
        from_parameter_name: TokenStream,
    ) -> impl Iterator<Item = TokenStream> + '_ {
        self.fields
            .iter()
            .map(move |field_info| {
                let field_name = format_ident!("{}", field_info.name);
                quote! {
                    {
                        use flow_record::ToMsgPackValue;
                        #from_parameter_name.#field_name.to_msgpack_value()
                    }
                }
            })
            .chain(self.attrs.values())
    }
}
