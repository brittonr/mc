type TokenStream = proc_macro2::TokenStream;

mod arms;
mod props;

#[derive(serde::Deserialize, Debug)]
pub(crate) enum StatusEffectCategory {
    Beneficial,
    Harmful,
    Neutral,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct AttributeModifiers {
    attribute: u8,
    operation: u8,
    value: f64,
    uuid: String,
}

#[derive(serde::Deserialize, Debug)]
pub(crate) struct StatusEffect {
    id: u16,
    name: String,
    translation_key: String,
    category: StatusEffectCategory,
    color: u32,
    instant: bool,
    attribute_modifiers: Option<Vec<AttributeModifiers>>,
}

pub(crate) fn build() -> anyhow::Result<TokenStream> {
    valence_build_utils::rerun_if_changed(["extracted/effects.json"]);

    let effects =
        serde_json::from_str::<Vec<StatusEffect>>(include_str!("../../extracted/effects.json"))?;
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    Ok(tokens(&effects))
}

fn tokens(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let effect_count = effects.len();
    let effect_variants = arms::variants(effects);
    let methods = arms::methods(effects);

    quote::quote! {
        #[doc = "Represents an attribute modifier."]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct AttributeModifier {
            #[doc = "The attribute that this modifier modifies."]
            pub attribute: super::attributes::EntityAttribute,
            #[doc = "The operation that this modifier applies."]
            pub operation: super::attributes::EntityAttributeOperation,
            #[doc = "The value of this modifier."]
            pub value: f64,
            #[doc = "The UUID of this modifier."]
            pub uuid: uuid::Uuid,
        }

        #[doc = "Represents a status effect category"]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum StatusEffectCategory {
            Beneficial,
            Harmful,
            Neutral,
        }

        #[doc = "Represents a status effect from the game"]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum StatusEffect {
            #(#effect_variants,)*
        }

        impl StatusEffect {
            #methods

            #[doc = "An array of all effects."]
            pub const ALL: [Self; #effect_count] = [#(Self::#effect_variants,)*];
        }
    }
}
