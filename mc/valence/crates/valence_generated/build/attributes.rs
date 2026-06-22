use heck::ToPascalCase;

type AttributeMap = std::collections::BTreeMap<String, EntityAttribute>;
type TokenStream = proc_macro2::TokenStream;

const ADD_OPERATION_ID: u8 = 0;
const MULTIPLY_BASE_OPERATION_ID: u8 = 1;
const MULTIPLY_TOTAL_OPERATION_ID: u8 = 2;

#[derive(serde::Deserialize)]
struct EntityAttribute {
    id: u8,
    default_value: f64,
    translation_key: String,
    tracked: bool,
    min_value: f64,
    max_value: f64,
}

struct AttributeCode {
    variants: TokenStream,
    get_id_arms: TokenStream,
    from_id_arms: TokenStream,
    name_arms: TokenStream,
    default_value_arms: TokenStream,
    translation_key_arms: TokenStream,
    tracked_arms: TokenStream,
    min_value_arms: TokenStream,
    max_value_arms: TokenStream,
}

pub(crate) fn build() -> anyhow::Result<TokenStream> {
    valence_build_utils::rerun_if_changed(["extracted/attributes.json"]);
    let attributes: AttributeMap =
        serde_json::from_str(include_str!("../extracted/attributes.json"))?;
    debug_assert!(
        !attributes.is_empty(),
        "generated attribute list is non-empty"
    );
    Ok(output(attribute_code(&attributes)))
}

fn attribute_code(attributes: &AttributeMap) -> AttributeCode {
    debug_assert!(
        !attributes.is_empty(),
        "generated attribute list is non-empty"
    );
    AttributeCode {
        variants: variants(attributes),
        get_id_arms: get_id_arms(attributes),
        from_id_arms: from_id_arms(attributes),
        name_arms: name_arms(attributes),
        default_value_arms: default_value_arms(attributes),
        translation_key_arms: translation_key_arms(attributes),
        tracked_arms: tracked_arms(attributes),
        min_value_arms: min_value_arms(attributes),
        max_value_arms: max_value_arms(attributes),
    }
}

fn variants(attributes: &AttributeMap) -> TokenStream {
    attributes
        .keys()
        .map(|name| {
            let key = attribute_key(name);
            quote::quote! { #key, }
        })
        .collect()
}

fn get_id_arms(attributes: &AttributeMap) -> TokenStream {
    attributes
        .iter()
        .map(|(name, attribute)| {
            let key = attribute_key(name);
            let id = attribute.id;
            quote::quote! { EntityAttribute::#key => #id, }
        })
        .collect()
}

fn from_id_arms(attributes: &AttributeMap) -> TokenStream {
    attributes
        .iter()
        .map(|(name, attribute)| {
            let key = attribute_key(name);
            let id = attribute.id;
            quote::quote! { #id => Some(EntityAttribute::#key), }
        })
        .collect()
}

fn name_arms(attributes: &AttributeMap) -> TokenStream {
    attributes
        .keys()
        .map(|name| {
            let key = attribute_key(name);
            quote::quote! { EntityAttribute::#key => #name, }
        })
        .collect()
}

fn default_value_arms(attributes: &AttributeMap) -> TokenStream {
    attributes
        .iter()
        .map(|(name, attribute)| {
            let key = attribute_key(name);
            let default_value = attribute.default_value;
            quote::quote! { EntityAttribute::#key => #default_value, }
        })
        .collect()
}

fn translation_key_arms(attributes: &AttributeMap) -> TokenStream {
    attributes
        .iter()
        .map(|(name, attribute)| {
            let key = attribute_key(name);
            let translation_key = &attribute.translation_key;
            quote::quote! { EntityAttribute::#key => #translation_key, }
        })
        .collect()
}

fn tracked_arms(attributes: &AttributeMap) -> TokenStream {
    attributes
        .iter()
        .map(|(name, attribute)| {
            let key = attribute_key(name);
            let is_tracked = attribute.tracked;
            quote::quote! { EntityAttribute::#key => #is_tracked, }
        })
        .collect()
}

fn min_value_arms(attributes: &AttributeMap) -> TokenStream {
    attributes
        .iter()
        .map(|(name, attribute)| {
            let key = attribute_key(name);
            let min_value = attribute.min_value;
            quote::quote! { EntityAttribute::#key => #min_value, }
        })
        .collect()
}

fn max_value_arms(attributes: &AttributeMap) -> TokenStream {
    attributes
        .iter()
        .map(|(name, attribute)| {
            let key = attribute_key(name);
            let max_value = attribute.max_value;
            quote::quote! { EntityAttribute::#key => #max_value, }
        })
        .collect()
}

fn output(code: AttributeCode) -> TokenStream {
    debug_assert!(
        !code.variants.is_empty(),
        "attribute variants are non-empty"
    );
    let variants = code.variants;
    let get_id_arms = code.get_id_arms;
    let from_id_arms = code.from_id_arms;
    let name_arms = code.name_arms;
    let default_value_arms = code.default_value_arms;
    let translation_key_arms = code.translation_key_arms;
    let tracked_arms = code.tracked_arms;
    let min_value_arms = code.min_value_arms;
    let max_value_arms = code.max_value_arms;

    quote::quote! {
        #[doc = "An attribute modifier operation."]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum EntityAttributeOperation {
            #[doc = "Adds the modifier to the base value."]
            Add,
            #[doc = "Multiplies the modifier with the base value."]
            MultiplyBase,
            #[doc = "Multiplies the modifier with the total value."]
            MultiplyTotal,
        }

        impl EntityAttributeOperation {
            #[doc = "Converts from a raw [`u8`]."]
            pub fn from_raw(raw: u8) -> Option<Self> {
                match raw {
                    #ADD_OPERATION_ID => Some(Self::Add),
                    #MULTIPLY_BASE_OPERATION_ID => Some(Self::MultiplyBase),
                    #MULTIPLY_TOTAL_OPERATION_ID => Some(Self::MultiplyTotal),
                    _ => None,
                }
            }

            #[doc = "Converts to a raw [`u8`]."]
            pub fn to_raw(self) -> u8 {
                match self {
                    Self::Add => #ADD_OPERATION_ID,
                    Self::MultiplyBase => #MULTIPLY_BASE_OPERATION_ID,
                    Self::MultiplyTotal => #MULTIPLY_TOTAL_OPERATION_ID,
                }
            }
        }

        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub enum EntityAttribute { #variants }

        impl EntityAttribute {
            pub fn get_id(self) -> u8 { match self { #get_id_arms } }
            pub fn from_id(id: u8) -> Option<Self> {
                match id { #from_id_arms _ => None }
            }
            pub fn name(self) -> &'static str { match self { #name_arms } }
            pub fn default_value(self) -> f64 { match self { #default_value_arms } }
            pub fn translation_key(self) -> &'static str { match self { #translation_key_arms } }
            pub fn tracked(self) -> bool { match self { #tracked_arms } }
            pub fn min_value(self) -> f64 { match self { #min_value_arms } }
            pub fn max_value(self) -> f64 { match self { #max_value_arms } }
        }
    }
}

fn attribute_key(name: &str) -> proc_macro2::Ident {
    valence_build_utils::ident(name.to_pascal_case())
}
