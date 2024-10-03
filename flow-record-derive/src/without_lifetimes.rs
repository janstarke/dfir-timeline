use syn::{
    punctuated::Punctuated, AngleBracketedGenericArguments, GenericArgument, Path, PathArguments,
    PathSegment, Token, Type, TypePath,
};

pub trait WithoutLifetimes {
    fn without_lifetimes(&self) -> Self;
}

impl WithoutLifetimes for Type {
    fn without_lifetimes(&self) -> Self {
        match self {
            Type::Path(type_path) => Type::Path(type_path.without_lifetimes()),
            _ => unimplemented!("no support for this type yet: '{:?}'", self),
        }
    }
}

impl WithoutLifetimes for TypePath {
    fn without_lifetimes(&self) -> Self {
        Self {
            qself: self.qself.clone(),
            path: self.path.without_lifetimes(),
        }
    }
}

impl WithoutLifetimes for Path {
    fn without_lifetimes(&self) -> Self {
        let segments = self.segments.without_lifetimes();
        let leading_colon = self.leading_colon;
        Self {
            leading_colon,
            segments,
        }
    }
}

impl WithoutLifetimes for Punctuated<PathSegment, Token![::]> {
    fn without_lifetimes(&self) -> Self {
        Self::from_iter(self.iter().map(|s| s.without_lifetimes()))
    }
}

impl WithoutLifetimes for PathSegment {
    fn without_lifetimes(&self) -> Self {
        match &self.arguments {
            PathArguments::AngleBracketed(args) => Self {
                ident: self.ident.clone(),
                arguments: PathArguments::AngleBracketed(args.without_lifetimes()),
            },
            _ => self.clone(),
        }
    }
}

impl WithoutLifetimes for AngleBracketedGenericArguments {
    fn without_lifetimes(&self) -> Self {
        let args = self.args.without_lifetimes();
        //let colon2_token = Some(Token![::](args.span()));
        let colon2_token = self.colon2_token;
        Self {
            colon2_token,
            lt_token: self.lt_token,
            args,
            gt_token: self.gt_token,
        }
    }
}

impl WithoutLifetimes for Punctuated<GenericArgument, Token![,]> {
    fn without_lifetimes(&self) -> Self {
        Self::from_iter(self.iter().filter_map(|arg| match arg {
            // continue recursively
            GenericArgument::Type(ty) => Some(GenericArgument::Type(ty.without_lifetimes())),
            // this is the place where we filter out the lifetimes
            GenericArgument::Lifetime(_) => None,
            // forward all other arguments
            x => Some(x.clone()),
        }))
    }
}
