#![doc = include_str!("../README.md")]

use quote::ToTokens;

type Attribute = syn::Attribute;
type GenericParam = syn::GenericParam;
type Generics = syn::Generics;
type Lifetime = syn::Lifetime;
type LifetimeParam = syn::LifetimeParam;
type LitInt = syn::LitInt;
type Result<T> = syn::Result<T>;
type TokenStream = proc_macro2::TokenStream;
type Variant = syn::Variant;

mod decode;
mod encode;
mod packet;

#[proc_macro_derive(Encode, attributes(packet))]
pub fn derive_encode(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match encode::derive(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(Decode, attributes(packet))]
pub fn derive_decode(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match decode::derive(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_derive(Packet, attributes(packet))]
pub fn derive_packet(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match packet::derive(item.into()) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

fn pair_variants_with_discriminants(
    variants: impl IntoIterator<Item = Variant>,
) -> Result<Vec<(i32, Variant)>> {
    let mut discriminant = 0;
    variants
        .into_iter()
        .map(|v| {
            if let Some(i) = parse_tag_attr(&v.attrs)? {
                discriminant = i;
            }

            let pair = (discriminant, v);
            discriminant += 1;
            Ok(pair)
        })
        .collect::<Result<_>>()
}

fn parse_tag_attr(attrs: &[Attribute]) -> Result<Option<i32>> {
    for attr in attrs {
        if !attr.path().is_ident("packet") {
            continue;
        }

        let mut res = 0;
        attr.parse_nested_meta(|meta| parse_tag_meta(&mut res, meta))?;
        return Ok(Some(res));
    }

    Ok(None)
}

fn parse_tag_meta(res: &mut i32, meta: syn::meta::ParseNestedMeta<'_>) -> Result<()> {
    if meta.path.is_ident("tag") {
        let tag: LitInt = meta.value()?.parse()?;
        *res = tag.base10_parse::<i32>()?;
        return Ok(());
    }

    Err(meta.error("unrecognized argument"))
}

/// Adding our lifetime to the generics before calling `.split_for_impl()` would
/// also add it to the resulting `ty_generics`, which we don't want. So I'm
/// doing this hack.
fn decode_split_for_impl(
    mut generics: Generics,
    lifetime: Lifetime,
) -> (TokenStream, TokenStream, TokenStream) {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut impl_generics = impl_generics.to_token_stream();
    let ty_generics = ty_generics.to_token_stream();
    let where_clause = where_clause.to_token_stream();

    if generics.lifetimes().next().is_none() {
        generics
            .params
            .push(GenericParam::Lifetime(LifetimeParam::new(lifetime)));

        impl_generics = generics.split_for_impl().0.to_token_stream();
    }

    (impl_generics, ty_generics, where_clause)
}

fn add_trait_bounds(generics: &mut Generics, trait_: TokenStream) {
    for param in &mut generics.params {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(syn::parse_quote!(#trait_))
        }
    }
}
