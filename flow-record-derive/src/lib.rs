use derive_record::expand_derive_serialize;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derive_record;
mod field_info;
mod struct_info;

#[proc_macro_derive(Record, attributes(flow_record))]
pub fn record_derive(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    expand_derive_serialize(&mut input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
