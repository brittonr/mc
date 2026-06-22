const UUID_HYPHENATED_LEN: usize = 36;

type AttributeModifiers = super::AttributeModifiers;
type StatusEffect = super::StatusEffect;
type StatusEffectCategory = super::StatusEffectCategory;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn category_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .map(|effect| {
            let category = category_path(&effect.category);
            let name = super::arms::variant_name(effect);
            quote::quote! {
                Self::#name => #category,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the category of this effect."]
        pub const fn category(&self) -> StatusEffectCategory {
            match self {
                #arms
            }
        }
    }
}

pub(super) fn color_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .map(|effect| {
            let color = &effect.color;
            let name = super::arms::variant_name(effect);
            quote::quote! {
                Self::#name => #color,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the color of this effect."]
        pub const fn color(&self) -> u32 {
            match self {
                #arms
            }
        }
    }
}

pub(super) fn instant_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .map(|effect| {
            let is_instant = &effect.instant;
            let name = super::arms::variant_name(effect);
            quote::quote! {
                Self::#name => #is_instant,
            }
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets whether this effect is instant."]
        pub const fn instant(&self) -> bool {
            match self {
                #arms
            }
        }
    }
}

pub(super) fn attribute_modifier_arms(effects: &[StatusEffect]) -> TokenStream {
    debug_assert!(
        !effects.is_empty(),
        "generated status-effect list is non-empty"
    );
    let arms = effects
        .iter()
        .filter_map(|effect| {
            effect.attribute_modifiers.as_ref().map(|modifiers| {
                let name = super::arms::variant_name(effect);
                let modifiers = modifiers.iter().map(attribute_modifier_tokens);
                quote::quote! {
                    Self::#name => vec![#(#modifiers,)*],
                }
            })
        })
        .collect::<TokenStream>();

    quote::quote! {
        #[doc = "Gets the attribute modifiers of this effect."]
        pub fn attribute_modifiers(&self) -> Vec<AttributeModifier> {
            match self {
                #arms
                _ => vec![],
            }
        }
    }
}

fn attribute_modifier_tokens(modifier: &AttributeModifiers) -> TokenStream {
    debug_assert!(!modifier.uuid.is_empty(), "modifier UUID is non-empty");
    debug_assert_eq!(
        modifier.uuid.len(),
        UUID_HYPHENATED_LEN,
        "modifier UUID has hyphenated length"
    );
    let attribute = &modifier.attribute;
    let operation = &modifier.operation;
    let value = &modifier.value;
    let uuid = &modifier.uuid;

    quote::quote! {
        AttributeModifier {
            attribute: match super::attributes::EntityAttribute::from_id(#attribute) {
                Some(attribute) => attribute,
                None => return Vec::new(),
            },
            operation: match super::attributes::EntityAttributeOperation::from_raw(#operation) {
                Some(operation) => operation,
                None => return Vec::new(),
            },
            value: #value,
            uuid: match uuid::Uuid::parse_str(#uuid) {
                Ok(uuid) => uuid,
                Err(_) => return Vec::new(),
            },
        }
    }
}

fn category_path(category: &StatusEffectCategory) -> TokenStream {
    match category {
        StatusEffectCategory::Beneficial => {
            quote::quote! { StatusEffectCategory::Beneficial }
        }
        StatusEffectCategory::Harmful => quote::quote! { StatusEffectCategory::Harmful },
        StatusEffectCategory::Neutral => quote::quote! { StatusEffectCategory::Neutral },
    }
}
