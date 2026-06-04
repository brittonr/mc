type TokenStream = proc_macro2::TokenStream;

mod arms;
mod props;

#[derive(serde::Deserialize, Clone, Debug)]
struct Item {
    id: u16,
    name: String,
    translation_key: String,
    max_stack: i8,
    max_durability: u16,
    enchantability: u8,
    fireproof: bool,
    food: Option<FoodComponent>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct FoodComponent {
    hunger: u16,
    saturation: f32,
    always_edible: bool,
    meat: bool,
    snack: bool,
}

pub(crate) fn build() -> anyhow::Result<TokenStream> {
    valence_build_utils::rerun_if_changed(["extracted/items.json"]);

    let items = serde_json::from_str::<Vec<Item>>(include_str!("../extracted/items.json"))?;
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    Ok(tokens(&items))
}

fn tokens(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let item_kind_count = items.len();
    let item_kind_variants = arms::variants(items);
    let methods = arms::methods(items);

    quote::quote! {
        #[doc = "Represents an item from the game"]
        #[allow(unknown_lints, acronym_style)]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
        #[repr(u16)]
        pub enum ItemKind {
            #[default]
            #(#item_kind_variants,)*
        }

        #[doc = "Contains food information about an item."]
        #[doc = ""]
        #[doc = "Only food items have a food component."]
        #[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
        pub struct FoodComponent {
            pub hunger: u16,
            pub saturation: f32,
            pub always_edible: bool,
            pub meat: bool,
            pub snack: bool,
        }

        impl ItemKind {
            #methods

            #[doc = "An array of all item kinds."]
            pub const ALL: [Self; #item_kind_count] = [#(Self::#item_kind_variants,)*];
        }
    }
}
