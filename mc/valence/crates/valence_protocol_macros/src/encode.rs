use syn::spanned::Spanned;

type Data = syn::Data;
type DeriveInput = syn::DeriveInput;
type ExprStream = proc_macro2::TokenStream;
type Fields = syn::Fields;
type Ident = proc_macro2::Ident;
type LitInt = syn::LitInt;
type Result<T> = syn::Result<T>;
type Span = proc_macro2::Span;
type Variant = syn::Variant;

pub(super) fn derive(item: ExprStream) -> Result<ExprStream> {
    let mut input = syn::parse2::<DeriveInput>(item)?;
    debug_assert!(
        !input.ident.to_string().is_empty(),
        "derive input has an identifier"
    );

    let input_name = input.ident;
    crate::add_trait_bounds(
        &mut input.generics,
        quote::quote!(::valence_protocol::__private::Encode),
    );
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    match input.data {
        Data::Struct(struct_) => struct_impl(
            &input_name,
            &struct_.fields,
            &impl_generics,
            &ty_generics,
            &where_clause,
        ),
        Data::Enum(enum_) => enum_impl(
            &input_name,
            enum_.variants,
            &impl_generics,
            &ty_generics,
            &where_clause,
        ),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "cannot derive `Encode` on unions",
        )),
    }
}

fn struct_impl(
    input_name: &Ident,
    fields: &Fields,
    impl_generics: &impl quote::ToTokens,
    ty_generics: &impl quote::ToTokens,
    where_clause: &impl quote::ToTokens,
) -> Result<ExprStream> {
    let body = struct_body(input_name, fields)?;
    Ok(quote::quote! {
        #[allow(unused_imports)]
        impl #impl_generics ::valence_protocol::__private::Encode for #input_name #ty_generics
        #where_clause
        {
            fn encode(&self, mut _w: impl ::std::io::Write) -> ::valence_protocol::__private::Result<()> {
                use ::valence_protocol::__private::{Encode, Context};

                #body

                Ok(())
            }
        }
    })
}

fn struct_body(input_name: &Ident, fields: &Fields) -> Result<ExprStream> {
    match fields {
        Fields::Named(fields) => fields
            .named
            .iter()
            .map(|field| {
                let Some(name) = field.ident.as_ref() else {
                    return Err(syn::Error::new(
                        field.span(),
                        "missing named field identifier",
                    ));
                };
                let ctx = format!("failed to encode field `{name}` in `{input_name}`");
                Ok(quote::quote! {
                    self.#name.encode(&mut _w).context(#ctx)?;
                })
            })
            .collect::<Result<_>>(),
        Fields::Unnamed(fields) => Ok((0..fields.unnamed.len())
            .map(|i| {
                let lit = LitInt::new(&i.to_string(), Span::call_site());
                let ctx = format!("failed to encode field `{lit}` in `{input_name}`");
                quote::quote! {
                    self.#lit.encode(&mut _w).context(#ctx)?;
                }
            })
            .collect()),
        Fields::Unit => Ok(ExprStream::new()),
    }
}

fn enum_impl(
    input_name: &Ident,
    variants: syn::punctuated::Punctuated<Variant, syn::token::Comma>,
    impl_generics: &impl quote::ToTokens,
    ty_generics: &impl quote::ToTokens,
    where_clause: &impl quote::ToTokens,
) -> Result<ExprStream> {
    let arms = enum_arms(input_name, variants)?;
    Ok(quote::quote! {
        #[allow(unused_imports, unreachable_code)]
        impl #impl_generics ::valence_protocol::__private::Encode for #input_name #ty_generics
        #where_clause
        {
            fn encode(&self, mut _w: impl ::std::io::Write) -> ::valence_protocol::__private::Result<()> {
                use ::valence_protocol::__private::{Encode, VarInt, Context};

                match self {
                    #arms
                    _ => unreachable!(),
                }
            }
        }
    })
}

fn enum_arms(
    input_name: &Ident,
    variants: syn::punctuated::Punctuated<Variant, syn::token::Comma>,
) -> Result<ExprStream> {
    crate::pair_variants_with_discriminants(variants)?
        .iter()
        .map(|(disc, variant)| variant_arm(input_name, *disc, variant))
        .collect::<Result<_>>()
}

fn variant_arm(input_name: &Ident, disc: i32, variant: &Variant) -> Result<ExprStream> {
    debug_assert!(
        !variant.ident.to_string().is_empty(),
        "variant identifiers are present"
    );
    let variant_name = &variant.ident;
    let disc_ctx = format!(
        "failed to encode enum discriminant {disc} for variant `{variant_name}` in `{input_name}`",
    );

    match &variant.fields {
        Fields::Named(fields) => {
            named_variant_arm(input_name, variant_name, fields, disc, &disc_ctx)
        }
        Fields::Unnamed(fields) => {
            unnamed_variant_arm(input_name, variant_name, fields, disc, &disc_ctx)
        }
        Fields::Unit => Ok(quote::quote! {
            Self::#variant_name => Ok(
                VarInt(#disc)
                    .encode(&mut _w)
                    .context(#disc_ctx)?
            ),
        }),
    }
}

fn named_variant_arm(
    input_name: &Ident,
    variant_name: &Ident,
    fields: &syn::FieldsNamed,
    disc: i32,
    disc_ctx: &str,
) -> Result<ExprStream> {
    debug_assert!(
        !variant_name.to_string().is_empty(),
        "variant identifiers are present"
    );
    let field_names = fields
        .named
        .iter()
        .map(|field| {
            let Some(name) = field.ident.as_ref() else {
                return Err(syn::Error::new(
                    field.span(),
                    "missing named field identifier",
                ));
            };
            Ok(name)
        })
        .collect::<Result<Vec<_>>>()?;
    let body = field_names
        .iter()
        .map(|name| field_body(input_name, variant_name, name))
        .collect::<ExprStream>();

    Ok(quote::quote! {
        Self::#variant_name { #(#field_names,)* } => {
            VarInt(#disc).encode(&mut _w).context(#disc_ctx)?;

            #body
            Ok(())
        }
    })
}

fn unnamed_variant_arm(
    input_name: &Ident,
    variant_name: &Ident,
    fields: &syn::FieldsUnnamed,
    disc: i32,
    disc_ctx: &str,
) -> Result<ExprStream> {
    let field_names = (0..fields.unnamed.len())
        .map(|i| Ident::new(&format!("_{i}"), Span::call_site()))
        .collect::<Vec<_>>();
    let body = field_names
        .iter()
        .map(|name| field_body(input_name, variant_name, name))
        .collect::<ExprStream>();

    Ok(quote::quote! {
        Self::#variant_name(#(#field_names,)*) => {
            VarInt(#disc).encode(&mut _w).context(#disc_ctx)?;

            #body
            Ok(())
        }
    })
}

fn field_body(input_name: &Ident, variant_name: &Ident, name: &Ident) -> ExprStream {
    let ctx =
        format!("failed to encode field `{name}` in variant `{variant_name}` in `{input_name}`",);
    quote::quote! {
        #name.encode(&mut _w).context(#ctx)?;
    }
}
