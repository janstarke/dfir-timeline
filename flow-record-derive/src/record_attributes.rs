use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::quote;

#[derive(FromDeriveInput)]
#[darling(supports(struct_named), attributes(flow_record))]
pub struct RecordAttributes {
    #[darling(default)]
    skip_meta: bool,
    source: String,
    classification: String,
    version: u8,
}

impl RecordAttributes {
    pub fn values(&self) -> impl Iterator<Item = TokenStream> {
        let (source, classification, version) = (&self.source, &self.classification, &self.version);
        let values = if self.skip_meta {
            Vec::new()
        } else {
            vec![
                quote! {flow_record::prelude::rmpv::Value::from(#source)},
                quote! {flow_record::prelude::rmpv::Value::from(#classification)},
                quote! {flow_record::prelude::rmpv::Value::from(chrono::Utc::now().timestamp())},
                quote! {flow_record::prelude::rmpv::Value::from(#version)},
            ]
        };
        values.into_iter()
    }
}
