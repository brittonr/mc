use std::collections::BTreeSet;

use quote::ToTokens;

type Arg = crate::path::Arg;
type Data = syn::Data;
type Fields = syn::Fields;
type Ident = proc_macro2::Ident;
type Result<T> = syn::Result<T>;
type Route = crate::path::Route;
type TokenStream = proc_macro2::TokenStream;

const NO_PATHS_ERROR: &str = "No paths attribute found for command";
const MISSING_VARIANT_PATH_ERROR: &str =
    "Command enum variant is missing a paths attribute and would not register a handler";
const SUGGESTIONS_UNSUPPORTED_ERROR: &str =
    "Command derive does not support suggestions attributes yet; use manual CommandGraphBuilder construction for suggestion metadata";
const INVALID_SCOPE_ERROR: &str =
    "Command scopes must be non-empty, contain no whitespace, and use non-empty dot-separated segments";
const DUPLICATE_ROUTE_ERROR: &str =
    "Duplicate command path would register an ambiguous command graph route";
const ROOT_PATH_MARKER: &str = "{/}";

pub(super) fn run(input: syn::DeriveInput) -> Result<proc_macro::TokenStream> {
    Ok(proc_macro::TokenStream::from(expand_input(input)?))
}

fn expand_input(input: syn::DeriveInput) -> Result<TokenStream> {
    let input_name = input.ident;
    let attrs = input.attrs;
    ensure_no_suggestion_attrs(&attrs)?;
    let outer_scopes = attrs
        .iter()
        .find_map(|attr| crate::path::lit_list(attr, "scopes"))
        .unwrap_or_default();
    validate_scopes(&input_name, &outer_scopes)?;

    match input.data {
        Data::Enum(data_enum) => enum_impl(&input_name, &attrs, data_enum, &outer_scopes),
        Data::Struct(data_struct) => struct_impl(&input_name, &attrs, data_struct, &outer_scopes),
        Data::Union(data_union) => Err(syn::Error::new_spanned(
            data_union.union_token,
            "Command enum must be an enum, not a union",
        )),
    }
}

fn enum_impl(
    input_name: &Ident,
    attrs: &[syn::Attribute],
    data_enum: syn::DataEnum,
    outer_scopes: &[String],
) -> Result<TokenStream> {
    debug_assert!(
        !input_name.to_string().is_empty(),
        "derive input has an identifier"
    );
    let Some(alias_routes) = attrs.iter().find_map(crate::path::parse) else {
        return Err(syn::Error::new_spanned(input_name, NO_PATHS_ERROR));
    };
    ensure_unique_routes(input_name, &alias_routes)?;
    let mut alias_iter = alias_routes.into_iter();
    let Some(base_route) = alias_iter.next() else {
        return Err(syn::Error::new_spanned(input_name, NO_PATHS_ERROR));
    };

    let variant_routes = collect_variant_routes(data_enum)?;
    let mut expanded_nodes = Vec::with_capacity(variant_routes.len());
    for (routes, fields, variant_ident) in variant_routes {
        let processed = crate::node::enum_routes(
            routes,
            crate::node::EnumSpec {
                enum_name: input_name,
                fields: &fields,
                variant_ident: &variant_ident,
                can_execute: true,
                outer_scopes,
            },
        )?;
        expanded_nodes.push(quote::quote! { #processed; });
    }

    let root_ident = quote::format_ident!("{}Root", input_name);
    let base = base_expansion(input_name, root_ident.clone(), base_route, outer_scopes)?;
    let alias_routes = alias_iter.collect::<Vec<_>>();
    let aliases = alias_expansion(input_name, root_ident, alias_routes, outer_scopes)?;

    Ok(quote::quote! {
        impl valence::command::Command for #input_name {
            fn assemble_graph(command_graph: &mut valence::command::graph::CommandGraphBuilder<Self>) {
                use valence::command::parsers::CommandArg;
                #base

                #aliases

                #(#expanded_nodes)*
            }
        }
    })
}

fn collect_variant_routes(data_enum: syn::DataEnum) -> Result<Vec<(Vec<Route>, Fields, Ident)>> {
    let mut routes = Vec::with_capacity(data_enum.variants.len());
    let mut route_keys = BTreeSet::new();
    for variant in data_enum.variants {
        ensure_no_suggestion_attrs(&variant.attrs)?;
        let mut variant_routes = Vec::new();
        for attr in &variant.attrs {
            let Some(attr_routes) = crate::path::parse(attr) else {
                continue;
            };
            if attr_routes.is_empty() {
                return Err(syn::Error::new_spanned(attr, NO_PATHS_ERROR));
            }
            record_unique_routes(attr, &attr_routes, &mut route_keys)?;
            variant_routes.extend(attr_routes);
        }
        if variant_routes.is_empty() {
            return Err(syn::Error::new_spanned(
                variant.ident,
                MISSING_VARIANT_PATH_ERROR,
            ));
        }
        routes.push((
            variant_routes,
            variant.fields.clone(),
            variant.ident.clone(),
        ));
    }
    Ok(routes)
}

fn base_expansion(
    input_name: &Ident,
    root_ident: Ident,
    base_route: Route,
    outer_scopes: &[String],
) -> Result<TokenStream> {
    debug_assert!(
        !input_name.to_string().is_empty(),
        "derive input has an identifier"
    );
    let processed = crate::node::enum_routes(
        vec![base_route],
        crate::node::EnumSpec {
            enum_name: input_name,
            fields: &Fields::Unit,
            variant_ident: &root_ident,
            can_execute: false,
            outer_scopes,
        },
    )?;
    let command_root = with_scopes(
        quote::quote! {
            let command_root_node = #processed
        },
        outer_scopes,
    );

    Ok(quote::quote! {
        #command_root.id();
    })
}

fn alias_expansion(
    input_name: &Ident,
    root_ident: Ident,
    alias_routes: Vec<Route>,
    outer_scopes: &[String],
) -> Result<TokenStream> {
    debug_assert!(
        !input_name.to_string().is_empty(),
        "derive input has an identifier"
    );
    let mut aliases = Vec::with_capacity(alias_routes.len());
    for route in alias_routes {
        let processed = crate::node::enum_routes(
            vec![route],
            crate::node::EnumSpec {
                enum_name: input_name,
                fields: &Fields::Unit,
                variant_ident: &root_ident,
                can_execute: false,
                outer_scopes,
            },
        )?;
        let alias = with_scopes(
            quote::quote! {
                #processed
                    .redirect_to(command_root_node)
            },
            outer_scopes,
        );
        aliases.push(alias);
    }

    Ok(quote::quote! {
        #(#aliases;)*
    })
}

fn struct_impl(
    input_name: &Ident,
    attrs: &[syn::Attribute],
    data_struct: syn::DataStruct,
    outer_scopes: &[String],
) -> Result<TokenStream> {
    debug_assert!(
        !input_name.to_string().is_empty(),
        "derive input has an identifier"
    );
    let routes = collect_struct_routes(input_name, attrs)?;
    let mut expanded_nodes = Vec::with_capacity(routes.len());
    for route in routes {
        let processed = crate::node::struct_routes(
            route,
            crate::node::StructSpec {
                struct_name: input_name,
                fields: &data_struct.fields,
                outer_scopes,
            },
        )?;
        expanded_nodes.push(quote::quote! { #processed; });
    }

    Ok(quote::quote! {
        impl valence::command::Command for #input_name {
            fn assemble_graph(command_graph: &mut valence::command::graph::CommandGraphBuilder<Self>) {
                use valence::command::parsers::CommandArg;
                #(#expanded_nodes)*
            }
        }
    })
}

fn collect_struct_routes(input_name: &Ident, attrs: &[syn::Attribute]) -> Result<Vec<Vec<Route>>> {
    let mut routes = Vec::with_capacity(attrs.len());
    let mut route_keys = BTreeSet::new();
    for attr in attrs {
        let Some(attr_routes) = crate::path::parse(attr) else {
            continue;
        };
        if attr_routes.is_empty() {
            return Err(syn::Error::new_spanned(attr, NO_PATHS_ERROR));
        }
        record_unique_routes(attr, &attr_routes, &mut route_keys)?;
        routes.push(attr_routes);
    }
    if routes.is_empty() {
        return Err(syn::Error::new_spanned(input_name, NO_PATHS_ERROR));
    }
    Ok(routes)
}

fn with_scopes(inner: TokenStream, outer_scopes: &[String]) -> TokenStream {
    if outer_scopes.is_empty() {
        return inner;
    }

    quote::quote! {
        #inner
            .with_scopes(vec![#(#outer_scopes),*])
    }
}

fn ensure_no_suggestion_attrs(attrs: &[syn::Attribute]) -> Result<()> {
    for attr in attrs {
        if attr.path().is_ident("suggestions") {
            return Err(syn::Error::new_spanned(attr, SUGGESTIONS_UNSUPPORTED_ERROR));
        }
    }
    Ok(())
}

fn validate_scopes(input_name: &Ident, scopes: &[String]) -> Result<()> {
    for scope in scopes {
        if !valid_scope(scope) {
            return Err(syn::Error::new_spanned(
                input_name,
                format!("{INVALID_SCOPE_ERROR}: `{scope}`"),
            ));
        }
    }
    Ok(())
}

fn valid_scope(scope: &str) -> bool {
    !scope.is_empty()
        && scope.trim() == scope
        && !scope.chars().any(char::is_whitespace)
        && scope.split('.').all(|segment| !segment.is_empty())
}

fn ensure_unique_routes(owner: &Ident, routes: &[Route]) -> Result<()> {
    let mut route_keys = BTreeSet::new();
    record_unique_routes(owner, routes, &mut route_keys)
}

fn record_unique_routes(
    owner: impl ToTokens,
    routes: &[Route],
    route_keys: &mut BTreeSet<String>,
) -> Result<()> {
    for route in routes {
        let key = route_key(route);
        if !route_keys.insert(key.clone()) {
            return Err(syn::Error::new_spanned(
                owner,
                format!("{DUPLICATE_ROUTE_ERROR}: `{key}`"),
            ));
        }
    }
    Ok(())
}

fn route_key(route: &Route) -> String {
    let mut parts = Vec::new();
    if route.is_at_root {
        parts.push(ROOT_PATH_MARKER.to_owned());
    }
    for arg in &route.args {
        parts.push(arg_key(arg));
    }
    parts.join(" ")
}

fn arg_key(arg: &Arg) -> String {
    match arg {
        Arg::Required(ident) => format!("{{{ident}}}"),
        Arg::Optional(ident) => format!("{{{ident}?}}"),
        Arg::Literal(literal) => literal.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEED_LITERAL: &str = "speed";
    const AMOUNT_ARGUMENT: &str = "amount";
    const VALID_SCOPE: &str = "valence.command.speed";
    const INVALID_SCOPE: &str = "valence command speed";

    fn expanded_tokens(input: syn::DeriveInput) -> String {
        expand_input(input).unwrap().to_string()
    }

    fn expand_error(input: syn::DeriveInput) -> String {
        expand_input(input).unwrap_err().to_string()
    }

    #[test]
    fn enum_expansion_contains_literal_argument_scope_and_handler() {
        let expanded = expanded_tokens(syn::parse_quote! {
            #[paths("command")]
            #[scopes(#VALID_SCOPE)]
            enum SpeedCommand {
                #[paths("speed {amount}")]
                Speed { amount: i32 },
            }
        });

        assert!(expanded.contains("SpeedCommand"));
        assert!(expanded.contains("literal"));
        assert!(expanded.contains(&format!("\"{SPEED_LITERAL}\"")));
        assert!(expanded.contains("argument"));
        assert!(expanded.contains(&format!("\"{AMOUNT_ARGUMENT}\"")));
        assert!(expanded.contains("with_parser"));
        assert!(expanded.contains("i32"));
        assert!(expanded.contains(&format!("\"{VALID_SCOPE}\"")));
        assert!(expanded.contains("with_executable"));
    }

    #[test]
    fn duplicate_literal_route_is_rejected() {
        let error = expand_error(syn::parse_quote! {
            #[paths("command")]
            enum DuplicateCommand {
                #[paths("speed")]
                First,
                #[paths("speed")]
                Second,
            }
        });

        assert!(error.contains(DUPLICATE_ROUTE_ERROR));
        assert!(error.contains(SPEED_LITERAL));
    }

    #[test]
    fn enum_variant_without_path_is_rejected_as_missing_handler() {
        let error = expand_error(syn::parse_quote! {
            #[paths("command")]
            enum MissingHandlerCommand {
                #[paths("speed")]
                Speed,
                NoHandler,
            }
        });

        assert!(error.contains(MISSING_VARIANT_PATH_ERROR));
    }

    #[test]
    fn missing_parser_field_is_rejected() {
        let error = expand_error(syn::parse_quote! {
            #[paths("command")]
            enum MissingFieldCommand {
                #[paths("speed {amount}")]
                Speed { value: i32 },
            }
        });

        assert!(error.contains("command argument field not found"));
    }

    #[test]
    fn invalid_scope_is_rejected() {
        let error = expand_error(syn::parse_quote! {
            #[paths("command")]
            #[scopes(#INVALID_SCOPE)]
            enum InvalidScopeCommand {
                #[paths("speed")]
                Speed,
            }
        });

        assert!(error.contains(INVALID_SCOPE_ERROR));
        assert!(error.contains(INVALID_SCOPE));
    }

    #[test]
    fn suggestion_attribute_is_rejected_for_manual_fallback() {
        let error = expand_error(syn::parse_quote! {
            #[paths("command")]
            #[suggestions(amount = "minecraft:ask_server")]
            enum SuggestedCommand {
                #[paths("speed {amount}")]
                Speed { amount: i32 },
            }
        });

        assert!(error.contains(SUGGESTIONS_UNSUPPORTED_ERROR));
    }

    #[test]
    fn optional_followed_by_literal_is_rejected() {
        let error = expand_error(syn::parse_quote! {
            #[paths("command")]
            enum OptionalOrderCommand {
                #[paths("speed {amount?} later")]
                Speed { amount: Option<i32> },
            }
        });

        assert!(error.contains("Only optional args can follow an optional arg"));
    }

    #[test]
    fn struct_without_path_is_rejected() {
        let error = expand_error(syn::parse_quote! {
            struct MissingPathCommand {
                amount: i32,
            }
        });

        assert!(error.contains(NO_PATHS_ERROR));
    }
}
