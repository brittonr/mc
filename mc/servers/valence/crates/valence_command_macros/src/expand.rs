type Data = syn::Data;
type Fields = syn::Fields;
type Ident = proc_macro2::Ident;
type Result<T> = syn::Result<T>;
type Route = crate::path::Route;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn run(input: syn::DeriveInput) -> Result<proc_macro::TokenStream> {
    let input_name = input.ident;
    let attrs = input.attrs;
    let outer_scopes = attrs
        .iter()
        .find_map(|attr| crate::path::lit_list(attr, "scopes"))
        .unwrap_or_default();

    let expansion = match input.data {
        Data::Enum(data_enum) => enum_impl(&input_name, &attrs, data_enum, &outer_scopes)?,
        Data::Struct(data_struct) => struct_impl(&input_name, &attrs, data_struct, &outer_scopes)?,
        Data::Union(data_union) => {
            return Err(syn::Error::new_spanned(
                data_union.union_token,
                "Command enum must be an enum, not a union",
            ))
        }
    };

    Ok(proc_macro::TokenStream::from(expansion))
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
        return Err(syn::Error::new_spanned(
            input_name,
            "No paths attribute found for command enum",
        ));
    };
    let mut alias_iter = alias_routes.into_iter();
    let Some(base_route) = alias_iter.next() else {
        return Err(syn::Error::new_spanned(
            input_name,
            "No paths attribute found for command enum",
        ));
    };

    let variant_routes = collect_variant_routes(data_enum);
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

fn collect_variant_routes(data_enum: syn::DataEnum) -> Vec<(Vec<Route>, Fields, Ident)> {
    let mut routes = Vec::with_capacity(data_enum.variants.len());
    for variant in data_enum.variants {
        for attr in &variant.attrs {
            if let Some(attr_routes) = crate::path::parse(attr) {
                routes.push((attr_routes, variant.fields.clone(), variant.ident.clone()));
            }
        }
    }
    routes
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
    let routes = collect_struct_routes(attrs);
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

fn collect_struct_routes(attrs: &[syn::Attribute]) -> Vec<Vec<Route>> {
    let mut routes = Vec::with_capacity(attrs.len());
    for attr in attrs {
        if let Some(attr_routes) = crate::path::parse(attr) {
            routes.push(attr_routes);
        }
    }
    routes
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
