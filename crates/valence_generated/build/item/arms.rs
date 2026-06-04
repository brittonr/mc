use heck::ToPascalCase;

type Item = super::Item;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn methods(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let item_kind_from_raw_id_arms = raw_id_from_arms(items);
    let item_kind_to_raw_id_arms = raw_id_to_arms(items);
    let item_kind_from_str_arms = str_from_arms(items);
    let item_kind_to_str_arms = str_to_arms(items);
    let item_kind_to_translation_key_arms = translation_key_arms(items);
    let item_kind_to_max_stack_arms = super::props::max_stack_arms(items);
    let item_kind_to_food_component_arms = super::props::food_component_arms(items);
    let item_kind_to_max_durability_arms = super::props::max_durability_arms(items);
    let item_kind_to_enchantability_arms = super::props::enchantability_arms(items);
    let item_kind_to_fireproof_arms = super::props::fireproof_arms(items);

    quote::quote! {
        #item_kind_from_raw_id_arms
        #item_kind_to_raw_id_arms
        #item_kind_from_str_arms
        #item_kind_to_str_arms
        #item_kind_to_translation_key_arms
        #item_kind_to_max_stack_arms
        #item_kind_to_food_component_arms
        #item_kind_to_max_durability_arms
        #item_kind_to_enchantability_arms
        #item_kind_to_fireproof_arms
    }
}

fn raw_id_from_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .map(|item| {
            let id = &item.id;
            let name = variant_name(item);
            quote::quote! {
                #id => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Constructs a item kind from a raw item ID."]
        #[doc = ""]
        #[doc = "If the given ID is invalid, `None` is returned."]
        pub const fn from_raw(id: u16) -> Option<Self> {
            match id {
                #arms
                _ => None
            }
        }
    }
}

fn raw_id_to_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .map(|item| {
            let id = &item.id;
            let name = variant_name(item);
            quote::quote! {
                Self::#name => #id,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the raw item ID from the item kind"]
        pub const fn to_raw(self) -> u16 {
            match self {
                #arms
            }
        }
    }
}

fn str_from_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .map(|item| {
            let str_name = &item.name;
            let name = variant_name(item);
            quote::quote! {
                #str_name => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Construct an item kind for its `snake_case` name."]
        #[doc = ""]
        #[doc = "Returns `None` if the name is invalid."]
        #[allow(clippy::should_implement_trait)]
        pub fn from_str(name: &str) -> Option<ItemKind> {
            match name {
                #arms
                _ => None
            }
        }
    }
}

fn str_to_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .map(|item| {
            let str_name = &item.name;
            let name = variant_name(item);
            quote::quote! {
                Self::#name => #str_name,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the `snake_case` name of this item kind."]
        pub const fn to_str(self) -> &'static str {
            match self {
                #arms
            }
        }
    }
}

fn translation_key_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .map(|item| {
            let name = variant_name(item);
            let translation_key = &item.translation_key;
            quote::quote! {
                Self::#name => #translation_key,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the translation key of this item kind."]
        pub const fn translation_key(self) -> &'static str {
            match self {
                #arms
            }
        }
    }
}

pub(super) fn variants(items: &[Item]) -> Vec<proc_macro2::Ident> {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    items.iter().map(variant_name).collect()
}

pub(super) fn variant_name(item: &Item) -> proc_macro2::Ident {
    valence_build_utils::ident(item.name.to_pascal_case())
}
