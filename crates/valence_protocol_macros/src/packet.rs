use heck::ToShoutySnakeCase;
use syn::spanned::Spanned;

type Expr = syn::Expr;
type Ident = proc_macro2::Ident;
type LitInt = syn::LitInt;
type LitStr = syn::LitStr;
type Span = proc_macro2::Span;
type Attribute = syn::Attribute;
type Meta<'a> = syn::meta::ParseNestedMeta<'a>;
type Result<T> = syn::Result<T>;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn derive(item: TokenStream) -> Result<TokenStream> {
    let mut input = syn::parse2::<syn::DeriveInput>(item)?;
    debug_assert!(
        !input.ident.to_string().is_empty(),
        "derive input has an identifier"
    );

    let attr = parse_helper_attr(&input.attrs)?.unwrap_or_default();
    let name = input.ident.clone();
    let name_str = name_from_attr(&name, attr.name.as_ref());
    debug_assert!(!name_str.is_empty(), "packet names cannot be empty");

    let id = id_expr(&name_str, &attr)?;
    crate::add_trait_bounds(&mut input.generics, quote::quote!(::std::fmt::Debug));

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let side = side_expr(&name_str, &attr)?;
    let state = attr
        .state
        .unwrap_or_else(|| syn::parse_quote!(::valence_protocol::PacketState::Play));
    let packet_trait = quote::quote!(::valence_protocol::__private::Packet);

    Ok(quote::quote! {
        impl #impl_generics #packet_trait for #name #ty_generics
        #where_clause
        {
            const ID: i32 = #id;
            const NAME: &'static str = #name_str;
            const SIDE: ::valence_protocol::PacketSide = #side;
            const STATE: ::valence_protocol::PacketState = #state;
        }
    })
}

fn name_from_attr(name: &Ident, attr_name: Option<&LitStr>) -> String {
    if let Some(attr_name) = attr_name {
        return attr_name.value();
    }
    name.to_string()
}

fn id_expr(name_str: &str, attr: &Attr) -> Result<Expr> {
    debug_assert!(!name_str.is_empty(), "packet names cannot be empty");
    if let Some(expr) = attr.id.clone() {
        return Ok(expr);
    }

    let Ok(ident) = syn::parse_str::<Ident>(&name_str.to_shouty_snake_case()) else {
        return Err(syn::Error::new(
            attr.span,
            "missing valid `id = ...` value from `packet` attr",
        ));
    };
    Ok(syn::parse_quote!(::valence_protocol::packet_id::#ident))
}

fn side_expr(name_str: &str, attr: &Attr) -> Result<Expr> {
    debug_assert!(!name_str.is_empty(), "packet names cannot be empty");
    if let Some(side_attr) = attr.side.clone() {
        return Ok(side_attr);
    }

    let lower_name = name_str.to_lowercase();
    if lower_name.ends_with("s2c") {
        return Ok(syn::parse_quote!(
            ::valence_protocol::PacketSide::Clientbound
        ));
    }
    if lower_name.ends_with("c2s") {
        return Ok(syn::parse_quote!(
            ::valence_protocol::PacketSide::Serverbound
        ));
    }

    Err(syn::Error::new(
        attr.span,
        "missing `side = PacketSide::...` value from `packet` attribute",
    ))
}

struct Attr {
    span: Span,
    id: Option<Expr>,
    tag: Option<i32>,
    name: Option<LitStr>,
    side: Option<Expr>,
    state: Option<Expr>,
}

impl Default for Attr {
    fn default() -> Self {
        Self {
            span: Span::call_site(),
            id: None,
            tag: None,
            name: None,
            side: None,
            state: None,
        }
    }
}

fn parse_helper_attr(attrs: &[Attribute]) -> Result<Option<Attr>> {
    for attr in attrs {
        if !attr.path().is_ident("packet") {
            continue;
        }

        debug_assert!(attr.path().is_ident("packet"), "packet attr guard checked");
        let mut res = Attr {
            span: attr.span(),
            id: None,
            tag: None,
            name: None,
            side: None,
            state: None,
        };

        attr.parse_nested_meta(|meta| parse_meta(&mut res, meta))?;
        return Ok(Some(res));
    }

    Ok(None)
}

fn parse_meta(res: &mut Attr, meta: Meta<'_>) -> Result<()> {
    debug_assert!(!meta.path.segments.is_empty(), "meta paths are non-empty");
    if meta.path.is_ident("id") {
        res.id = Some(meta.value()?.parse::<Expr>()?);
        return Ok(());
    }
    if meta.path.is_ident("tag") {
        let tag: LitInt = meta.value()?.parse()?;
        res.tag = Some(tag.base10_parse::<i32>()?);
        return Ok(());
    }
    if meta.path.is_ident("name") {
        res.name = Some(meta.value()?.parse::<LitStr>()?);
        return Ok(());
    }
    if meta.path.is_ident("side") {
        res.side = Some(meta.value()?.parse::<Expr>()?);
        return Ok(());
    }
    if meta.path.is_ident("state") {
        res.state = Some(meta.value()?.parse::<Expr>()?);
        return Ok(());
    }

    Err(meta.error("unrecognized packet argument"))
}
