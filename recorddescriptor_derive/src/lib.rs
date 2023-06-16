use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_derive(HasRecordDescriptor)]
pub fn recorddescriptor_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let descriptor = match ast.data {
        syn::Data::Struct(s) => struct_descriptor(&s),
        syn::Data::Enum(_) => panic!("no support for enums yet"),
        syn::Data::Union(_) => panic!("no support for unions yet"),
    };

    let gen = quote!(
        impl HasRecordDescriptor for #name {
            fn descriptor() -> record_types::RecordDescriptor {
                record_types::RecordDescriptor::from_iter(#descriptor.into_iter())
            }
        }
    );
    gen.into()
}

fn struct_descriptor(s: &syn::DataStruct) -> proc_macro2::TokenStream {
    match &s.fields {
        syn::Fields::Named(n) => {
            let recurse = n.named.iter().map(|f| {
                let field_name = f.ident.as_ref().unwrap().to_string();
                let field_type = &f.ty;
                quote_spanned!(f.span()=>
                    RecordField::from((#field_name,
                    <#field_type>::to_field_type()))
                )
            });
            quote! {
                vec![ #(#recurse , )* ]
            }
        }
        syn::Fields::Unnamed(_) => unimplemented!(),
        syn::Fields::Unit => unimplemented!(),
    }
}
