use syn::spanned::Spanned;

type Data = syn::Data;
type DeriveInput = syn::DeriveInput;
type ExprStream = proc_macro2::TokenStream;
type Fields = syn::Fields;
type Ident = proc_macro2::Ident;
type Lifetime = syn::Lifetime;
type Result<T> = syn::Result<T>;
type Variant = syn::Variant;

pub(super) fn derive(item: ExprStream) -> Result<ExprStream> {
    let mut input = syn::parse2::<DeriveInput>(item)?;
    debug_assert!(
        !input.ident.to_string().is_empty(),
        "derive input has an identifier"
    );

    validate_lifetimes(&input)?;
    let input_name = input.ident;
    let lifetime = derive_lifetime(&input.generics);

    match input.data {
        Data::Struct(struct_) => {
            struct_impl(&mut input.generics, &input_name, lifetime, struct_.fields)
        }
        Data::Enum(enum_) => enum_impl(&mut input.generics, &input_name, lifetime, enum_.variants),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "cannot derive `Decode` on unions",
        )),
    }
}

fn validate_lifetimes(input: &DeriveInput) -> Result<()> {
    if input.generics.lifetimes().count() <= 1 {
        return Ok(());
    }

    Err(syn::Error::new(
        input.generics.params.span(),
        "type deriving `Decode` must have no more than one lifetime",
    ))
}

fn derive_lifetime(generics: &syn::Generics) -> Lifetime {
    generics
        .lifetimes()
        .next()
        .map_or_else(|| syn::parse_quote!('a), |l| l.lifetime.clone())
}

fn struct_impl(
    generics: &mut syn::Generics,
    input_name: &Ident,
    lifetime: Lifetime,
    fields: Fields,
) -> Result<ExprStream> {
    debug_assert!(
        !input_name.to_string().is_empty(),
        "derive input has an identifier"
    );
    let body = struct_body(input_name, fields)?;
    crate::add_trait_bounds(
        generics,
        quote::quote!(::valence_protocol::__private::Decode<#lifetime>),
    );
    let (impl_generics, ty_generics, where_clause) =
        crate::decode_split_for_impl(generics.clone(), lifetime.clone());

    Ok(quote::quote! {
        #[allow(unused_imports)]
        impl #impl_generics ::valence_protocol::__private::Decode<#lifetime> for #input_name #ty_generics
        #where_clause
        {
            fn decode(_r: &mut &#lifetime [u8]) -> ::valence_protocol::__private::Result<Self> {
                use ::valence_protocol::__private::{Decode, Context, ensure};

                Ok(#body)
            }
        }
    })
}

fn struct_body(input_name: &Ident, fields: Fields) -> Result<ExprStream> {
    match fields {
        Fields::Named(fields) => {
            let init = fields
                .named
                .iter()
                .map(|field| named_field_init(input_name, field))
                .collect::<Result<ExprStream>>()?;
            Ok(quote::quote! {
                Self {
                    #init
                }
            })
        }
        Fields::Unnamed(fields) => {
            let init = unnamed_field_inits(input_name, fields.unnamed.len());
            Ok(quote::quote! {
                Self(#init)
            })
        }
        Fields::Unit => Ok(quote::quote!(Self)),
    }
}

fn enum_impl(
    generics: &mut syn::Generics,
    input_name: &Ident,
    lifetime: Lifetime,
    variants: syn::punctuated::Punctuated<Variant, syn::token::Comma>,
) -> Result<ExprStream> {
    debug_assert!(
        !input_name.to_string().is_empty(),
        "derive input has an identifier"
    );
    let arms = arms(input_name, variants)?;
    crate::add_trait_bounds(
        generics,
        quote::quote!(::valence_protocol::__private::Decode<#lifetime>),
    );
    let (impl_generics, ty_generics, where_clause) =
        crate::decode_split_for_impl(generics.clone(), lifetime.clone());

    Ok(quote::quote! {
        #[allow(unused_imports)]
        impl #impl_generics ::valence_protocol::__private::Decode<#lifetime> for #input_name #ty_generics
        #where_clause
        {
            fn decode(_r: &mut &#lifetime [u8]) -> ::valence_protocol::__private::Result<Self> {
                use ::valence_protocol::__private::{Decode, Context, VarInt, bail};

                let ctx = concat!("failed to decode enum discriminant in `", stringify!(#input_name), "`");
                let disc = VarInt::decode(_r).context(ctx)?.0;
                match disc {
                    #arms
                    n => bail!("unexpected enum discriminant {} in `{}`", disc, stringify!(#input_name)),
                }
            }
        }
    })
}

fn arms(
    input_name: &Ident,
    variants: syn::punctuated::Punctuated<Variant, syn::token::Comma>,
) -> Result<ExprStream> {
    crate::pair_variants_with_discriminants(variants)?
        .iter()
        .map(|(disc, variant)| arm(input_name, *disc, variant))
        .collect::<Result<_>>()
}

fn arm(input_name: &Ident, disc: i32, variant: &Variant) -> Result<ExprStream> {
    debug_assert!(
        !variant.ident.to_string().is_empty(),
        "variant identifiers are present"
    );
    let name = &variant.ident;

    match &variant.fields {
        Fields::Named(fields) => named_variant(input_name, name, fields, disc),
        Fields::Unnamed(fields) => Ok(unnamed_variant(
            input_name,
            name,
            fields.unnamed.len(),
            disc,
        )),
        Fields::Unit => Ok(quote::quote!(#disc => Ok(Self::#name),)),
    }
}

fn named_variant(
    input_name: &Ident,
    name: &Ident,
    fields: &syn::FieldsNamed,
    disc: i32,
) -> Result<ExprStream> {
    debug_assert!(
        !name.to_string().is_empty(),
        "variant identifiers are present"
    );
    let fields = fields
        .named
        .iter()
        .map(|field| variant_named_field(input_name, name, field))
        .collect::<Result<ExprStream>>()?;

    Ok(quote::quote! {
        #disc => Ok(Self::#name { #fields }),
    })
}

fn unnamed_variant(input_name: &Ident, name: &Ident, len: usize, disc: i32) -> ExprStream {
    let init = variant_unnamed_fields(input_name, name, len);
    quote::quote! {
        #disc => Ok(Self::#name(#init)),
    }
}

fn named_field_init(input_name: &Ident, field: &syn::Field) -> Result<ExprStream> {
    let Some(name) = field.ident.as_ref() else {
        return Err(syn::Error::new(
            field.span(),
            "missing named field identifier",
        ));
    };
    let ctx = format!("failed to decode field `{name}` in `{input_name}`");
    Ok(quote::quote! {
        #name: Decode::decode(_r).context(#ctx)?,
    })
}

fn unnamed_field_inits(input_name: &Ident, len: usize) -> ExprStream {
    (0..len)
        .map(|i| {
            let ctx = format!("failed to decode field `{i}` in `{input_name}`");
            quote::quote! {
                Decode::decode(_r).context(#ctx)?,
            }
        })
        .collect()
}

fn variant_named_field(
    input_name: &Ident,
    variant_name: &Ident,
    field: &syn::Field,
) -> Result<ExprStream> {
    let Some(field_name) = field.ident.as_ref() else {
        return Err(syn::Error::new(
            field.span(),
            "missing named field identifier",
        ));
    };
    let ctx = format!(
        "failed to decode field `{field_name}` in variant `{variant_name}` in `{input_name}`",
    );
    Ok(quote::quote! {
        #field_name: Decode::decode(_r).context(#ctx)?,
    })
}

fn variant_unnamed_fields(input_name: &Ident, variant_name: &Ident, len: usize) -> ExprStream {
    (0..len)
        .map(|i| {
            let ctx = format!(
                "failed to decode field `{i}` in variant `{variant_name}` in `{input_name}`",
            );
            quote::quote! {
                Decode::decode(_r).context(#ctx)?,
            }
        })
        .collect()
}
