use flow_record_common::FieldType;
use syn::Type;

pub struct FieldInfo {
    pub ft: FieldType,
    pub is_nullable: bool,
}

impl From<&Type> for FieldInfo {
    fn from(ty: &Type) -> Self {
        let mut ty = ty;
        let mut is_nullable = false;
        if let Some(inner) = extract_type_from_option(ty) {
            ty = inner;
            is_nullable = true;
        }

        if let Type::Path(type_path) = ty {
            if type_path.path.segments.len() == 1 {
                if let Some(type_name) = type_path.path.segments.iter().next() {
                    let ft = match &type_name.ident.to_string()[..] {
                        "String" => FieldType::String,
                        "u8" | "u16" => FieldType::UInt16,
                        "u32" => FieldType::UInt32,
                        "u64" | "u128" => FieldType::VarInt,
                        "i8" | "i16" | "i32" | "i64" | "i128" => FieldType::VarInt,
                        "DateTime" => FieldType::Datetime,
                        "Vec<u8>" => FieldType::Bin,
                        "f32" | "f64" => FieldType::Float,
                        _ => unimplemented!("no implementation for type '{type_name:?}' yet"),
                    };
                    return Self {
                        is_nullable,
                        ft
                    }
                }
            }
        }
        unimplemented!("no implementation for type '{ty:?}' yet")
    }
}

// inspired by <https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn>
fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    use syn::{GenericArgument, Path, PathArguments, PathSegment};

    fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
        match *ty {
            syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
            _ => None,
        }
    }

    // TODO store (with lazy static) the vec of string
    // TODO maybe optimization, reverse the order of segments
    fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
        let idents_of_path = path
            .segments
            .iter()
            .into_iter()
            .fold(String::new(), |mut acc, v| {
                acc.push_str(&v.ident.to_string());
                acc.push('|');
                acc
            });
        vec!["Option|", "std|option|Option|", "core|option|Option|"]
            .into_iter()
            .find(|s| &idents_of_path == *s)
            .and_then(|_| path.segments.last())
    }

    extract_type_path(ty)
        .and_then(|path| extract_option_segment(path))
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}
