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
                quote! {rmpv::Value::from(#source)},
                quote! {rmpv::Value::from(#classification)},
                quote! {rmpv::Value::from(chrono::Utc::now().timestamp())},
                quote! {rmpv::Value::from(#version)},
            ]
        };
        values.into_iter()
    }
}
