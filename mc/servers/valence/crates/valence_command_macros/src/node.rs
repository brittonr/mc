mod state;

type Arg = crate::path::Arg;
type Fields = syn::Fields;
type Ident = proc_macro2::Ident;
type Result<T> = syn::Result<T>;
type Route = crate::path::Route;
type TokenStream = proc_macro2::TokenStream;

pub(super) struct EnumSpec<'a> {
    pub enum_name: &'a Ident,
    pub fields: &'a Fields,
    pub variant_ident: &'a Ident,
    pub can_execute: bool,
    pub outer_scopes: &'a [String],
}

pub(super) struct StructSpec<'a> {
    pub struct_name: &'a Ident,
    pub fields: &'a Fields,
    pub outer_scopes: &'a [String],
}

struct StartMode {
    is_first_route: bool,
    should_continue_root: bool,
}

pub(super) fn enum_routes(routes: Vec<Route>, spec: EnumSpec<'_>) -> Result<TokenStream> {
    debug_assert!(
        !routes.is_empty(),
        "routes are parsed from at least one path"
    );
    let mut inner = quote::quote! {};
    let mut is_first_route = true;

    for route in routes {
        let mode = StartMode {
            is_first_route,
            should_continue_root: spec.can_execute && !route.is_at_root,
        };
        inner = route_start(inner, mode);
        inner = enum_route(inner, route, &spec)?;
        is_first_route = false;
    }

    Ok(inner)
}

pub(super) fn struct_routes(routes: Vec<Route>, spec: StructSpec<'_>) -> Result<TokenStream> {
    debug_assert!(
        !routes.is_empty(),
        "routes are parsed from at least one path"
    );
    let mut inner = quote::quote! {};
    let mut is_first_route = true;

    for route in routes {
        inner = struct_route_start(inner, is_first_route);
        inner = struct_route(inner, route, &spec)?;
        is_first_route = false;
    }

    Ok(inner)
}

fn route_start(previous: TokenStream, mode: StartMode) -> TokenStream {
    debug_assert!(
        mode.is_first_route || !previous.is_empty(),
        "previous route is built"
    );
    if mode.is_first_route && mode.should_continue_root {
        return quote::quote! {
            command_graph.at(command_root_node)
        };
    }
    if mode.is_first_route {
        return quote::quote! {
            command_graph.root()
        };
    }
    if mode.should_continue_root {
        return quote::quote! {
            #previous;

            command_graph.at(command_root_node)
        };
    }
    quote::quote! {
        #previous;

        command_graph.root()
    }
}

fn struct_route_start(previous: TokenStream, is_first_route: bool) -> TokenStream {
    if is_first_route {
        return quote::quote! {
            command_graph.root()
        };
    }

    quote::quote! {
        #previous;

        command_graph.root()
    }
}

fn enum_route(inner: TokenStream, route: Route, spec: &EnumSpec<'_>) -> Result<TokenStream> {
    let target = state::Target::Enum {
        ty: spec.enum_name,
        variant: spec.variant_ident,
    };
    let mut builder = state::Builder::new(target, spec.fields, &route.args, spec.outer_scopes);
    apply_route(inner, &mut builder, spec.can_execute)
}

fn struct_route(inner: TokenStream, route: Route, spec: &StructSpec<'_>) -> Result<TokenStream> {
    let target = state::Target::Struct {
        ty: spec.struct_name,
    };
    let mut builder = state::Builder::new(target, spec.fields, &route.args, spec.outer_scopes);
    apply_route(inner, &mut builder, false)
}

fn apply_route(
    mut inner: TokenStream,
    builder: &mut state::Builder<'_>,
    can_execute_literal: bool,
) -> Result<TokenStream> {
    let path_len = builder.args.len();
    debug_assert!(path_len == builder.args.len(), "path length is stable");
    let mut is_first_arg = true;

    for (index, arg) in builder.args.iter().enumerate() {
        let step = state::Step {
            index,
            is_last: index.checked_add(1).is_some_and(|next| next == path_len),
        };
        inner = match arg {
            Arg::Literal(lit) => literal(inner, lit, builder, step, can_execute_literal),
            Arg::Required(ident) => required(inner, ident, builder, step)?,
            Arg::Optional(ident) => optional(inner, ident, builder, step)?,
        };

        if matches!(builder.target, state::Target::Struct { .. }) && is_first_arg {
            inner = with_scopes(inner, builder.outer_scopes);
            is_first_arg = false;
        }
    }

    Ok(inner)
}

fn literal(
    inner: TokenStream,
    lit: &str,
    builder: &state::Builder<'_>,
    step: state::Step,
    can_execute_literal: bool,
) -> TokenStream {
    let mut expansion = quote::quote! {
        #inner.literal(#lit)
    };
    if matches!(builder.target, state::Target::Enum { .. }) {
        expansion = with_scopes(expansion, builder.outer_scopes);
    }

    if !(can_execute_literal && step.is_last) {
        return expansion;
    }

    builder.target.finish(expansion, &builder.final_fields)
}

fn required(
    inner: TokenStream,
    ident: &Ident,
    builder: &mut state::Builder<'_>,
    step: state::Step,
) -> Result<TokenStream> {
    let field_type = state::find_field_type(builder.fields, ident)?;
    let ident_string = ident.to_string();
    let mut expansion = argument(inner, &ident_string, field_type);
    builder.final_fields.push(quote::quote! {
        #ident: #field_type::parse_arg(s).unwrap()
    });

    if step.is_last {
        expansion = builder.target.finish(expansion, &builder.final_fields);
    }

    Ok(expansion)
}

fn optional(
    inner: TokenStream,
    ident: &Ident,
    builder: &mut state::Builder<'_>,
    step: state::Step,
) -> Result<TokenStream> {
    debug_assert!(
        step.index < builder.args.len(),
        "step comes from route args"
    );
    let field_type = state::find_field_type(builder.fields, ident)?;
    let parts = state::OptionalParts {
        node_ident: quote::format_ident!("graph_til_{}", ident),
        field_ident: ident,
        next_optional_args: state::trailing_optional_args(
            builder.args,
            step.index,
            builder.target.owner(),
        )?,
        option_inner: state::option_inner_type(field_type)?,
    };
    let ident_string = ident.to_string();
    let option_inner = parts.option_inner;
    let mut expansion = builder
        .target
        .checkpoint(inner, &builder.final_fields, &parts);
    expansion = argument(expansion, &ident_string, option_inner);

    builder.final_fields.push(quote::quote! {
        #ident: Some(#option_inner::parse_arg(s).unwrap())
    });

    if step.is_last {
        expansion = builder.target.finish(expansion, &builder.final_fields);
    }

    Ok(expansion)
}

fn argument(inner: TokenStream, ident_string: &str, field_type: &syn::Type) -> TokenStream {
    quote::quote! {
        #inner
            .argument(#ident_string)
            .with_parser::<#field_type>()
    }
}

fn with_scopes(inner: TokenStream, outer_scopes: &[String]) -> TokenStream {
    if outer_scopes.is_empty() {
        return inner;
    }

    quote::quote! {
        #inner.with_scopes(vec![#(#outer_scopes),*])
    }
}
