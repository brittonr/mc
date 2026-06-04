type KindCode = super::super::kind::KindCode;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn tokens(kind: &KindCode) -> TokenStream {
    debug_assert!(
        !kind.block_kind_variants.is_empty(),
        "block kind variants are non-empty"
    );
    debug_assert_eq!(
        kind.block_kind_count,
        kind.block_kind_variants.len(),
        "block kind count matches variants"
    );
    let block_kind_variants = &kind.block_kind_variants;
    let string_methods = string_methods(kind);
    let state_methods = state_methods(kind);
    let item_methods = item_methods(kind);
    let raw_methods = raw_methods(kind);
    let block_kind_count = kind.block_kind_count;

    quote::quote! {
        #[doc = "An enumeration of all block kinds."]
        #[allow(unknown_lints, acronym_style)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum BlockKind { #(#block_kind_variants,)* }

        impl BlockKind {
            #string_methods
            #state_methods
            #item_methods
            #raw_methods
            #[doc = "An array of all block kinds."]
            pub const ALL: [Self; #block_kind_count] = [#(Self::#block_kind_variants,)*];
        }

        #[doc = "The default block kind is `air`."]
        impl Default for BlockKind {
            fn default() -> Self { Self::Air }
        }
    }
}

fn string_methods(kind: &KindCode) -> TokenStream {
    debug_assert!(
        !kind.block_kind_from_str_arms.is_empty(),
        "string arms are non-empty"
    );
    let block_kind_from_str_arms = &kind.block_kind_from_str_arms;
    let block_kind_to_str_arms = &kind.block_kind_to_str_arms;
    let kind_to_translation_key_arms = &kind.kind_to_translation_key_arms;

    quote::quote! {
        #[doc = "Construct a block kind from its `snake_case` name."]
        #[doc = ""]
        #[doc = "Returns `None` if the name is invalid."]
        pub fn from_str(name: &str) -> Option<Self> {
            match name { #block_kind_from_str_arms _ => None }
        }

        #[doc = "Get the `snake_case` name of this block kind."]
        pub const fn to_str(self) -> &'static str {
            match self { #block_kind_to_str_arms }
        }

        pub const fn translation_key(self) -> &'static str {
            match self { #kind_to_translation_key_arms }
        }
    }
}

fn state_methods(kind: &KindCode) -> TokenStream {
    debug_assert!(
        !kind.block_kind_props_arms.is_empty(),
        "property arms are non-empty"
    );
    let block_kind_props_arms = &kind.block_kind_props_arms;

    quote::quote! {
        #[doc = "Returns the default block state for a given block kind."]
        pub const fn to_state(self) -> BlockState { BlockState::from_kind(self) }

        #[doc = "Returns a slice of all properties this block kind has."]
        pub const fn props(self) -> &'static [PropName] {
            match self { #block_kind_props_arms _ => &[] }
        }
    }
}

fn item_methods(kind: &KindCode) -> TokenStream {
    debug_assert!(
        !kind.block_kind_to_item_kind_arms.is_empty(),
        "item arms are non-empty"
    );
    let block_kind_to_item_kind_arms = &kind.block_kind_to_item_kind_arms;
    let block_kind_from_item_kind_arms = &kind.block_kind_from_item_kind_arms;

    quote::quote! {
        #[doc = "Converts a block kind to its corresponding item kind."]
        #[doc = ""]
        #[doc = "[`ItemKind::Air`] is used to indicate the absence of an item."]
        pub const fn to_item_kind(self) -> crate::item::ItemKind {
            let id = match self { #block_kind_to_item_kind_arms };
            match crate::item::ItemKind::from_raw(id) {
                Some(kind) => kind,
                None => unreachable!(),
            }
        }

        #[doc = "Constructs a block kind from an item kind."]
        #[doc = ""]
        #[doc = "If the given item does not have a corresponding block, `None` is returned."]
        pub const fn from_item_kind(item: crate::item::ItemKind) -> Option<Self> {
            #[allow(unreachable_patterns)]
            match item.to_raw() { #block_kind_from_item_kind_arms _ => None }
        }
    }
}

fn raw_methods(kind: &KindCode) -> TokenStream {
    debug_assert!(
        !kind.block_kind_from_raw_arms.is_empty(),
        "raw ID arms are non-empty"
    );
    let block_kind_from_raw_arms = &kind.block_kind_from_raw_arms;

    quote::quote! {
        #[doc = "Constructs a block kind from a raw block kind ID."]
        #[doc = ""]
        #[doc = "If the given ID is invalid, `None` is returned."]
        pub const fn from_raw(id: u16) -> Option<Self> {
            match id { #block_kind_from_raw_arms _ => None }
        }

        #[doc = "Converts this block kind to its underlying raw block state ID."]
        #[doc = ""]
        #[doc = "The original block kind can be recovered with [`BlockKind::from_raw`]."]
        pub const fn to_raw(self) -> u16 { self as u16 }
    }
}
