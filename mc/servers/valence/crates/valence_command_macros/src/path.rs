type Attribute = syn::Attribute;
type Expr = syn::Expr;
type Ident = proc_macro2::Ident;
type Meta = syn::Meta;
type TokenTree = proc_macro2::TokenTree;

#[derive(Debug, Clone)]
pub(super) enum Arg {
    Required(Ident),
    Optional(Ident),
    Literal(String),
}

#[derive(Debug, Clone)]
pub(super) struct Route {
    pub args: Vec<Arg>,
    pub is_at_root: bool,
}

pub(super) fn parse(attr: &Attribute) -> Option<Vec<Route>> {
    let values = lit_list(attr, "paths")?;
    let mut routes = Vec::with_capacity(values.len());

    for value in values {
        routes.push(parse_value(&value));
    }

    Some(routes)
}

pub(super) fn lit_list(attr: &Attribute, ident: &str) -> Option<Vec<String>> {
    match &attr.meta {
        Meta::NameValue(key_value) => name_value_strings(key_value, ident),
        Meta::List(list) => list_strings(list, ident),
        Meta::Path(_) => None,
    }
}

fn name_value_strings(key_value: &syn::MetaNameValue, ident: &str) -> Option<Vec<String>> {
    if !key_value.path.is_ident(ident) {
        return None;
    }

    match &key_value.value {
        Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(lit_str) => Some(vec![lit_str.value()]),
            _ => None,
        },
        _ => None,
    }
}

fn list_strings(list: &syn::MetaList, ident: &str) -> Option<Vec<String>> {
    if !list.path.is_ident(ident) {
        return None;
    }
    debug_assert!(list.path.is_ident(ident), "list attr guard checked");

    let tokens = list.tokens.clone().into_iter().collect::<Vec<_>>();
    let mut values = Vec::with_capacity(tokens.len());
    let mut needs_comma = false;

    for token in tokens {
        match token {
            TokenTree::Literal(lit) => {
                if needs_comma {
                    return None;
                }
                values.push(literal_string(lit)?);
                needs_comma = true;
            }
            TokenTree::Punct(punct) => {
                if punct.as_char() != ',' || !needs_comma {
                    return None;
                }
                needs_comma = false;
            }
            _ => return None,
        }
    }

    Some(values)
}

fn literal_string(lit: proc_macro2::Literal) -> Option<String> {
    let lit = lit.to_string();
    Some(lit.strip_prefix('"')?.strip_suffix('"')?.to_owned())
}

fn parse_value(value: &str) -> Route {
    let is_at_root = value.starts_with("{/}");
    let word_count = value.split_whitespace().count();
    let mut args = Vec::with_capacity(word_count);
    let mut words = value.split_whitespace();

    if is_at_root {
        let _root_marker = words.next();
    }

    for word in words {
        args.push(parse_word(word));
    }

    Route { args, is_at_root }
}

fn parse_word(word: &str) -> Arg {
    if let Some(inner) = word
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix("?}"))
    {
        return Arg::Optional(quote::format_ident!("{}", inner));
    }

    if let Some(inner) = word
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
    {
        return Arg::Required(quote::format_ident!("{}", inner));
    }

    Arg::Literal(word.to_owned())
}
