use proc_macro2::TokenStream;

pub struct FieldInfo {
    pub name: String,
    pub field_type_expr: TokenStream,
}

impl FieldInfo {
    pub fn new(name: String, field_type_expr: TokenStream) -> Self {
        Self {
            name,
            field_type_expr
        }
    }
}