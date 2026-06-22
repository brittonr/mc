use heck::ToPascalCase;

type TokenStream = proc_macro2::TokenStream;
type StatusEffect = super::StatusEffect;

pub(super) fn methods(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let effect_from_raw_id_arms = raw_id_from_arms(effects);
    let effect_to_raw_id_arms = raw_id_to_arms(effects);
    let effect_from_ident_arms = ident_from_arms(effects);
    let effect_to_ident_arms = ident_to_arms(effects);
    let effect_to_translation_key_arms = translation_key_arms(effects);
    let effect_to_category_arms = super::props::category_arms(effects);
    let effect_to_color_arms = super::props::color_arms(effects);
    let effect_to_instant_arms = super::props::instant_arms(effects);
    let effect_to_attribute_modifiers_arms = super::props::attribute_modifier_arms(effects);

    quote::quote! {
        #effect_from_raw_id_arms
        #effect_to_raw_id_arms
        #effect_from_ident_arms
        #effect_to_ident_arms
        #effect_to_translation_key_arms
        #effect_to_category_arms
        #effect_to_color_arms
        #effect_to_instant_arms
        #effect_to_attribute_modifiers_arms
    }
}

fn raw_id_from_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .map(|effect| {
            let id = &effect.id;
            let name = variant_name(effect);
            quote::quote! {
                #id => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Constructs a effect from a raw item ID."]
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

fn raw_id_to_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .map(|effect| {
            let id = &effect.id;
            let name = variant_name(effect);
            quote::quote! {
                Self::#name => #id,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the raw effect ID from the effect"]
        pub const fn to_raw(self) -> u16 {
            match self {
                #arms
            }
        }
    }
}

fn ident_from_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .map(|effect| {
            let ident_name = format!("minecraft:{}", &effect.name);
            let name = variant_name(effect);
            quote::quote! {
                #ident_name => Some(Self::#name),
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Construct a effect from its `snake_case` name."]
        #[doc = ""]
        #[doc = "Returns `None` if the name is invalid."]
        pub fn from_ident(id: valence_ident::Ident<&str>) -> Option<Self> {
            match id.as_str() {
                #arms
                _ => None
            }
        }
    }
}

fn ident_to_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .map(|effect| {
            let str_name = &effect.name;
            let name = variant_name(effect);
            quote::quote! {
                Self::#name => valence_ident::ident!(#str_name),
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the identifier of this effect."]
        pub const fn to_ident(self) -> valence_ident::Ident<&'static str> {
            match self {
                #arms
            }
        }

        #[doc = "Gets the name of this effect."]
        #[doc = "Same as [`StatusEffect::to_ident`], but doesn't take ownership."]
        pub const fn name(&self) -> valence_ident::Ident<&'static str> {
            match self {
                #arms
            }
        }
    }
}

fn translation_key_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .map(|effect| {
            let str_name = &effect.translation_key;
            let name = variant_name(effect);
            quote::quote! {
                Self::#name => #str_name,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the translation key of this effect."]
        pub const fn translation_key(&self) -> &'static str {
            match self {
                #arms
            }
        }
    }
}

pub(super) fn variants(effects: &[StatusEffect]) -> Vec<proc_macro2::Ident> {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    effects.iter().map(variant_name).collect()
}

pub(super) fn variant_name(effect: &StatusEffect) -> proc_macro2::Ident {
    valence_build_utils::ident(effect.name.to_pascal_case())
}
