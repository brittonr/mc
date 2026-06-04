type Item = super::Item;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn max_stack_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .map(|item| {
            let name = super::arms::variant_name(item);
            let max_stack = item.max_stack;
            quote::quote! {
                Self::#name => #max_stack,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Returns the maximum stack count."]
        pub const fn max_stack(self) -> i8 {
            match self {
                #arms
            }
        }
    }
}

pub(super) fn food_component_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .map(|item| match &item.food {
            Some(food_component) => food_component_arm(item, food_component),
            None => quote::quote! {},
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Returns a food component which stores hunger, saturation etc."]
        #[doc = ""]
        #[doc = "If the item kind can't be eaten, `None` will be returned."]
        pub const fn food_component(self) -> Option<FoodComponent> {
            match self {
                #arms
                _ => return None
            }
        }
    }
}

pub(super) fn max_durability_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .filter(|item| item.max_durability != 0)
        .map(|item| {
            let name = super::arms::variant_name(item);
            let max_durability = item.max_durability;
            quote::quote! {
                Self::#name => #max_durability,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Returns the maximum durability before the item will break."]
        #[doc = ""]
        #[doc = "If the item doesn't have durability, `0` is returned."]
        pub const fn max_durability(self) -> u16 {
            match self {
                #arms
                _ => return 0,
            }
        }
    }
}

pub(super) fn enchantability_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .filter(|item| item.enchantability != 0)
        .map(|item| {
            let name = super::arms::variant_name(item);
            let ench = item.enchantability;
            quote::quote! {
                Self::#name => #ench,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Returns the enchantability of the item kind."]
        #[doc = ""]
        #[doc = "If the item doesn't have durability, `0` is returned."]
        pub const fn enchantability(self) -> u8 {
            match self {
                #arms
                _ => return 0,
            }
        }
    }
}

pub(super) fn fireproof_arms(items: &[Item]) -> TokenStream {
    debug_assert!(!items.is_empty(), "generated item list is non-empty");
    let arms = items
        .iter()
        .filter(|item| item.fireproof)
        .map(|item| {
            let name = super::arms::variant_name(item);
            quote::quote! {
                Self::#name => true,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Returns if the item can survive in fire/lava."]
        pub const fn fireproof(self) -> bool {
            #[expect(clippy::match_like_matches_macro, reason = "generated item table keeps one match shape")]
            match self {
                #arms
                _ => return false
            }
        }
    }
}

fn food_component_arm(item: &Item, food_component: &super::FoodComponent) -> TokenStream {
    let name = super::arms::variant_name(item);
    let hunger = food_component.hunger;
    let saturation = food_component.saturation;
    let is_always_edible = food_component.always_edible;
    let is_meat = food_component.meat;
    let is_snack = food_component.snack;

    quote::quote! {
        Self::#name => Some(FoodComponent {
            hunger: #hunger,
            saturation: #saturation,
            always_edible: #is_always_edible,
            meat: #is_meat,
            snack: #is_snack,
        }
    ),
    }
}
