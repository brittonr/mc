type Arg = crate::path::Arg;
type Fields = syn::Fields;
type Ident = proc_macro2::Ident;
type Result<T> = syn::Result<T>;
type TokenStream = proc_macro2::TokenStream;

#[derive(Clone, Copy)]
pub(super) enum Target<'a> {
    Enum { ty: &'a Ident, variant: &'a Ident },
    Struct { ty: &'a Ident },
}

pub(super) struct Builder<'a> {
    pub target: Target<'a>,
    pub fields: &'a Fields,
    pub args: &'a [Arg],
    pub outer_scopes: &'a [String],
    pub final_fields: Vec<TokenStream>,
}

#[derive(Clone, Copy)]
pub(super) struct Step {
    pub index: usize,
    pub is_last: bool,
}

pub(super) struct OptionalParts<'a> {
    pub node_ident: Ident,
    pub field_ident: &'a Ident,
    pub next_optional_args: Vec<&'a Ident>,
    pub option_inner: &'a syn::Type,
}

impl<'a> Builder<'a> {
    pub(super) fn new(
        target: Target<'a>,
        fields: &'a Fields,
        args: &'a [Arg],
        outer_scopes: &'a [String],
    ) -> Self {
        Self {
            target,
            fields,
            args,
            outer_scopes,
            final_fields: Vec::with_capacity(args.len()),
        }
    }
}

impl<'a> Target<'a> {
    pub(super) fn owner(self) -> &'a Ident {
        match self {
            Self::Enum { variant, .. } => variant,
            Self::Struct { ty } => ty,
        }
    }

    pub(super) fn finish(
        self,
        expansion: TokenStream,
        final_fields: &[TokenStream],
    ) -> TokenStream {
        match self {
            Self::Enum { ty, variant } => quote::quote! {
                #expansion
                    .with_executable(|s| {
                        #ty::#variant {
                            #(#final_fields,)*
                        }
                    })
            },
            Self::Struct { ty } => quote::quote! {
                #expansion
                    .with_executable(|s| {
                        #ty {
                            #(#final_fields,)*
                        }
                    })
            },
        }
    }

    pub(super) fn checkpoint(
        self,
        inner: TokenStream,
        final_fields: &[TokenStream],
        parts: &OptionalParts<'_>,
    ) -> TokenStream {
        let node_ident = &parts.node_ident;
        let field_ident = parts.field_ident;
        let next_optional_args = &parts.next_optional_args;
        match self {
            Self::Enum { ty, variant } => quote::quote! {
                let #node_ident = {#inner
                    .with_executable(|s| {
                        #ty::#variant {
                            #(#final_fields,)*
                            #field_ident: None,
                            #(#next_optional_args: None,)*
                        }
                    })
                    .id()};

                command_graph.at(#node_ident)
            },
            Self::Struct { ty } => quote::quote! {
                let #node_ident = {#inner
                    .with_executable(|s| {
                        #ty {
                            #(#final_fields,)*
                            #field_ident: None,
                            #(#next_optional_args: None,)*
                        }
                    })
                    .id()};

                command_graph.at(#node_ident)
            },
        }
    }
}

pub(super) fn find_field_type<'a>(fields: &'a Fields, ident: &Ident) -> Result<&'a syn::Type> {
    for field in fields {
        let Some(field_ident) = field.ident.as_ref() else {
            continue;
        };
        if field_ident == ident {
            return Ok(&field.ty);
        }
    }

    Err(syn::Error::new_spanned(
        ident,
        "command argument field not found",
    ))
}

pub(super) fn option_inner_type(field_type: &syn::Type) -> Result<&syn::Type> {
    debug_assert!(
        !quote::quote!(#field_type).is_empty(),
        "option field type must have tokens"
    );
    let syn::Type::Path(type_path) = field_type else {
        return Err(syn::Error::new_spanned(
            field_type,
            "Option type must be a single path segment",
        ));
    };

    let mut segments = type_path.path.segments.iter();
    let Some(segment) = segments.next() else {
        return Err(syn::Error::new_spanned(
            &type_path.path,
            "Option type must be a single path segment",
        ));
    };
    if segments.next().is_some() || segment.ident != "Option" {
        return Err(syn::Error::new_spanned(
            &type_path.path,
            "Option type must be a single path segment",
        ));
    }

    angle_bracket_type(segment)
}

fn angle_bracket_type(segment: &syn::PathSegment) -> Result<&syn::Type> {
    debug_assert!(segment.ident == "Option", "caller checks Option segment");
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return Err(syn::Error::new_spanned(
            segment,
            "Option type must have a single generic argument",
        ));
    };
    let mut args_iter = args.args.iter();
    let Some(syn::GenericArgument::Type(generic_type)) = args_iter.next() else {
        return Err(syn::Error::new_spanned(
            args,
            "Option type must have a single generic argument",
        ));
    };
    if args_iter.next().is_some() {
        return Err(syn::Error::new_spanned(
            args,
            "Option type must have a single generic argument",
        ));
    }

    Ok(generic_type)
}

pub(super) fn trailing_optional_args<'a>(
    args: &'a [Arg],
    index: usize,
    owner: &Ident,
) -> Result<Vec<&'a Ident>> {
    let start = index.saturating_add(1);
    let mut optional_args = Vec::with_capacity(args.len().saturating_sub(start));
    for arg in args.iter().skip(start) {
        match arg {
            Arg::Optional(ident) => optional_args.push(ident),
            _ => {
                return Err(syn::Error::new_spanned(
                    owner,
                    "Only optional args can follow an optional arg",
                ))
            }
        }
    }
    Ok(optional_args)
}
