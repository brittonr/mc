use heck::ToPascalCase;

type Block = super::Block;
type TokenStream = proc_macro2::TokenStream;

pub(super) struct PropertyCode {
    pub prop_name_variants: Vec<proc_macro2::Ident>,
    pub prop_name_from_str_arms: TokenStream,
    pub prop_name_to_str_arms: TokenStream,
    pub prop_name_count: usize,
    pub prop_value_variants: Vec<proc_macro2::Ident>,
    pub prop_value_from_str_arms: TokenStream,
    pub prop_value_to_str_arms: TokenStream,
    pub prop_value_from_u16_arms: TokenStream,
    pub prop_value_to_u16_arms: TokenStream,
    pub prop_value_count: usize,
}

pub(super) fn build(blocks: &[Block]) -> PropertyCode {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    let prop_names = prop_names(blocks);
    let prop_values = prop_values(blocks);

    PropertyCode {
        prop_name_variants: prop_names.iter().map(|name| pascal_ident(name)).collect(),
        prop_name_from_str_arms: name_from_str_arms(&prop_names),
        prop_name_to_str_arms: name_to_str_arms(&prop_names),
        prop_name_count: prop_names.len(),
        prop_value_variants: prop_values
            .iter()
            .map(|value| pascal_ident(value))
            .collect(),
        prop_value_from_str_arms: value_from_str_arms(&prop_values),
        prop_value_to_str_arms: value_to_str_arms(&prop_values),
        prop_value_from_u16_arms: value_from_u16_arms(&prop_values),
        prop_value_to_u16_arms: value_to_u16_arms(&prop_values),
        prop_value_count: prop_values.len(),
    }
}

fn prop_names(blocks: &[Block]) -> Vec<String> {
    blocks
        .iter()
        .flat_map(|block| {
            block
                .properties
                .iter()
                .map(|property| property.name.as_str())
        })
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn prop_values(blocks: &[Block]) -> Vec<String> {
    blocks
        .iter()
        .flat_map(|block| {
            block
                .properties
                .iter()
                .flat_map(|property| &property.values)
        })
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .map(str::to_owned)
        .collect()
}

fn name_from_str_arms(prop_names: &[String]) -> TokenStream {
    prop_names
        .iter()
        .map(|name| {
            let ident = pascal_ident(name);
            quote::quote! {
                #name => Some(PropName::#ident),
            }
        })
        .collect()
}

fn name_to_str_arms(prop_names: &[String]) -> TokenStream {
    prop_names
        .iter()
        .map(|name| {
            let ident = pascal_ident(name);
            quote::quote! {
                PropName::#ident => #name,
            }
        })
        .collect()
}

fn value_from_str_arms(prop_values: &[String]) -> TokenStream {
    prop_values
        .iter()
        .map(|value| {
            let ident = pascal_ident(value);
            quote::quote! {
                #value => Some(PropValue::#ident),
            }
        })
        .collect()
}

fn value_to_str_arms(prop_values: &[String]) -> TokenStream {
    prop_values
        .iter()
        .map(|value| {
            let ident = pascal_ident(value);
            quote::quote! {
                PropValue::#ident => #value,
            }
        })
        .collect()
}

fn value_from_u16_arms(prop_values: &[String]) -> TokenStream {
    prop_values
        .iter()
        .filter_map(|value| value.parse::<u16>().ok())
        .map(|number| {
            let ident = valence_build_utils::ident(number.to_string());
            quote::quote! {
                #number => Some(PropValue::#ident),
            }
        })
        .collect()
}

fn value_to_u16_arms(prop_values: &[String]) -> TokenStream {
    prop_values
        .iter()
        .filter_map(|value| value.parse::<u16>().ok())
        .map(|number| {
            let ident = valence_build_utils::ident(number.to_string());
            quote::quote! {
                PropValue::#ident => Some(#number),
            }
        })
        .collect()
}

fn pascal_ident(name: &str) -> proc_macro2::Ident {
    valence_build_utils::ident(name.to_pascal_case())
}
