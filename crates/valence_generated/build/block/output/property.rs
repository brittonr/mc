type PropertyCode = super::super::property::PropertyCode;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn name_tokens(properties: &PropertyCode) -> TokenStream {
    debug_assert!(
        !properties.prop_name_variants.is_empty(),
        "property names are non-empty"
    );
    debug_assert_eq!(
        properties.prop_name_count,
        properties.prop_name_variants.len(),
        "property name count matches variants"
    );
    let prop_name_variants = &properties.prop_name_variants;
    let prop_name_from_str_arms = &properties.prop_name_from_str_arms;
    let prop_name_to_str_arms = &properties.prop_name_to_str_arms;
    let prop_name_count = properties.prop_name_count;

    quote::quote! {
        #[doc = "Contains all possible block state property names."]
        #[doc = ""]
        #[doc = "For example, `waterlogged`, `facing`, and `half` are all property names."]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum PropName { #(#prop_name_variants,)* }

        impl PropName {
            #[doc = "Construct a property name from its `snake_case` name."]
            #[doc = ""]
            #[doc = "Returns `None` if the given name is not valid."]
            pub fn from_str(name: &str) -> Option<Self> {
                match name { #prop_name_from_str_arms _ => None }
            }

            #[doc = "Get the `snake_case` name of this property name."]
            pub const fn to_str(self) -> &'static str {
                match self { #prop_name_to_str_arms }
            }

            #[doc = "An array of all property names."]
            pub const ALL: [Self; #prop_name_count] = [#(Self::#prop_name_variants,)*];
        }
    }
}

pub(super) fn value_tokens(properties: &PropertyCode) -> TokenStream {
    debug_assert!(
        !properties.prop_value_variants.is_empty(),
        "property values are non-empty"
    );
    debug_assert_eq!(
        properties.prop_value_count,
        properties.prop_value_variants.len(),
        "property value count matches variants"
    );
    let prop_value_variants = &properties.prop_value_variants;
    let prop_value_from_str_arms = &properties.prop_value_from_str_arms;
    let prop_value_to_str_arms = &properties.prop_value_to_str_arms;
    let prop_value_from_u16_arms = &properties.prop_value_from_u16_arms;
    let prop_value_to_u16_arms = &properties.prop_value_to_u16_arms;
    let prop_value_count = properties.prop_value_count;

    quote::quote! {
        #[doc = "Contains all possible values that a block property might have."]
        #[doc = ""]
        #[doc = "For example, `upper`, `true`, and `2` are all property values."]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum PropValue { #(#prop_value_variants,)* }

        impl PropValue {
            #[doc = "Construct a property value from its `snake_case` name."]
            #[doc = ""]
            #[doc = "Returns `None` if the given name is not valid."]
            pub fn from_str(name: &str) -> Option<Self> {
                match name { #prop_value_from_str_arms _ => None }
            }

            #[doc = "Get the `snake_case` name of this property value."]
            pub const fn to_str(self) -> &'static str {
                match self { #prop_value_to_str_arms }
            }

            #[doc = "Converts a `u16` into a numeric property value."]
            #[doc = "Returns `None` if the given number does not have a"]
            #[doc = "corresponding property value."]
            pub const fn from_u16(n: u16) -> Option<Self> {
                match n { #prop_value_from_u16_arms _ => None }
            }

            #[doc = "Converts this property value into a `u16` if it is numeric."]
            #[doc = "Returns `None` otherwise."]
            pub const fn to_u16(self) -> Option<u16> {
                match self { #prop_value_to_u16_arms _ => return None }
            }

            #[doc = "Converts a `bool` to a `True` or `False` property value."]
            pub const fn from_bool(b: bool) -> Self {
                if b { Self::True } else { Self::False }
            }

            #[doc = "Converts a `True` or `False` property value to a `bool`."]
            #[doc = ""]
            #[doc = "Returns `None` if this property value is not `True` or `False`"]
            pub const fn to_bool(self) -> Option<bool> {
                match self {
                    Self::True => Some(true),
                    Self::False => Some(false),
                    _ => return None,
                }
            }

            #[doc = "An array of all property values."]
            pub const ALL: [Self; #prop_value_count] = [#(Self::#prop_value_variants,)*];
        }

        impl From<bool> for PropValue {
            fn from(b: bool) -> Self { Self::from_bool(b) }
        }
    }
}
