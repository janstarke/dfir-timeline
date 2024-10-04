use std::collections::HashSet;

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::Type;

use crate::field_info::FieldInfo;
use crate::record_attributes::RecordAttributes;
use crate::without_lifetimes::WithoutLifetimes;

pub struct DescriptorCode {
    pub hash: TokenStream,
    pub descriptor: TokenStream,
}

pub struct StructInfo {
    name: String,
    fields: Vec<FieldInfo>,
    attrs: RecordAttributes,

    children_types: HashSet<Type>,
}

impl StructInfo {
    pub fn new(name: String, s: syn::DataStruct, attrs: RecordAttributes) -> Self {
        match s.fields {
            syn::Fields::Named(n) => {
                let mut fields = Vec::new();
                let mut children_types = HashSet::new();

                for field in n.named.iter() {
                    let field_type = field.ty.clone().without_lifetimes();
                    let field_type_expr = quote! {
                        <#field_type as ::flow_record::prelude::ToMsgPackValue>::field_type()
                    };
                    fields.push(FieldInfo::new(
                        field.ident.as_ref().unwrap().to_string(),
                        field_type_expr,
                    ));

                    if field
                        .attrs
                        .iter()
                        .any(|attr| attr.path().is_ident("has_descriptor"))
                    {
                        children_types.insert(field_type.clone());
                    }
                }

                Self {
                    name,
                    fields,
                    attrs,
                    children_types,
                }
            }
            _ => Self {
                name,
                fields: vec![],
                attrs,
                children_types: HashSet::new(),
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

    pub fn child_descriptors(&self) -> Vec<DescriptorCode> {
        self.children_types.iter().map(|ty| {
            let ty = ty.clone();
            let hash = quote! {
                <#ty as ::flow_record::prelude::ToMsgPackValue>::descriptor_hash()
            };
            let descriptor = quote! {
                <#ty as ::flow_record::prelude::ToMsgPackValue>::descriptor()
            };
            DescriptorCode { hash, descriptor }
        }).collect()
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
