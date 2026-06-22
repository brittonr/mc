#![doc = include_str!("../README.md")]

#[proc_macro]
pub fn parse_ident_str(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    parse_ident_str_inner(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn parse_ident_str_inner(item: proc_macro2::TokenStream) -> syn::Result<proc_macro2::TokenStream> {
    let ident_lit: syn::LitStr = syn::parse2(item)?;
    let mut ident = ident_lit.value();

    match ident.split_once(':') {
        Some((namespace, path)) if check_namespace(namespace) && check_path(path) => {}
        None if check_path(&ident) => {
            ident = format!("minecraft:{ident}");
        }
        _ => {
            return Err(syn::Error::new(
                ident_lit.span(),
                "string cannot be parsed as a resource identifier",
            ));
        }
    }

    Ok(quote::quote!(#ident))
}

fn check_namespace(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'))
}

fn check_path(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'))
}
