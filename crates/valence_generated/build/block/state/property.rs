use heck::ToPascalCase;

type Block = super::super::Block;
type Property = super::super::Property;
type TokenStream = proc_macro2::TokenStream;

#[derive(Clone, Copy)]
struct PropertyArithmetic {
    min_state_id: u16,
    product: u16,
    values_count: u16,
}

pub(super) fn get_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .filter(|block| !block.properties.is_empty())
        .map(get_block_arm)
        .collect()
}

fn get_block_arm(block: &Block) -> TokenStream {
    let block_kind_name = pascal_ident(&block.name);
    let arms = block
        .properties
        .iter()
        .filter_map(|property| get_property_arm(block, property))
        .collect::<TokenStream>();

    quote::quote! {
        BlockKind::#block_kind_name => match name {
            #arms
            _ => return None,
        },
    }
}

fn get_property_arm(block: &Block, property: &Property) -> Option<TokenStream> {
    let prop_name = pascal_ident(&property.name);
    let arithmetic = property_arithmetic(block, property)?;
    let min_state_id = arithmetic.min_state_id;
    let product = arithmetic.product;
    let values_count = arithmetic.values_count;
    let arms = property
        .values
        .iter()
        .enumerate()
        .filter_map(prop_value_match_arm)
        .collect::<TokenStream>();

    Some(quote::quote! {
        PropName::#prop_name => match self.0.saturating_sub(#min_state_id) / #product % #values_count {
            #arms
            _ => unreachable!(),
        },
    })
}

fn prop_value_match_arm((index, value): (usize, &String)) -> Option<TokenStream> {
    let value_idx = u16::try_from(index).ok()?;
    let value_name = pascal_ident(value);
    Some(quote::quote! {
        #value_idx => Some(PropValue::#value_name),
    })
}

pub(super) fn set_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .filter(|block| !block.properties.is_empty())
        .map(set_block_arm)
        .collect()
}

fn set_block_arm(block: &Block) -> TokenStream {
    let block_kind_name = pascal_ident(&block.name);
    let arms = block
        .properties
        .iter()
        .filter_map(|property| set_property_arm(block, property))
        .collect::<TokenStream>();

    quote::quote! {
        BlockKind::#block_kind_name => match name {
            #arms
            _ => self,
        },
    }
}

fn set_property_arm(block: &Block, property: &Property) -> Option<TokenStream> {
    let prop_name = pascal_ident(&property.name);
    let arithmetic = property_arithmetic(block, property)?;
    let arms = property
        .values
        .iter()
        .enumerate()
        .filter_map(|entry| set_prop_value_arm(entry, arithmetic))
        .collect::<TokenStream>();

    Some(quote::quote! {
        PropName::#prop_name => match val {
            #arms
            _ => self,
        },
    })
}

fn set_prop_value_arm(
    (index, value): (usize, &String),
    arithmetic: PropertyArithmetic,
) -> Option<TokenStream> {
    let val_idx = u16::try_from(index).ok()?;
    let val_name = pascal_ident(value);
    let min_state_id = arithmetic.min_state_id;
    let product = arithmetic.product;
    let values_count = arithmetic.values_count;
    Some(quote::quote! {
        PropValue::#val_name => Self(
            self.0
                .saturating_sub(
                    (self.0.saturating_sub(#min_state_id) / #product % #values_count)
                        .saturating_mul(#product),
                )
                .saturating_add((#val_idx).saturating_mul(#product)),
        ),
    })
}

fn property_arithmetic(block: &Block, property: &Property) -> Option<PropertyArithmetic> {
    Some(PropertyArithmetic {
        min_state_id: block.min_state_id()?,
        product: property_stride(block, property)?,
        values_count: values_count(property)?,
    })
}

fn property_stride(block: &Block, property: &Property) -> Option<u16> {
    block
        .properties
        .iter()
        .rev()
        .take_while(|other| property.name != other.name)
        .try_fold(1_u16, |product, prop| {
            product.checked_mul(values_count(prop)?)
        })
}

fn values_count(property: &Property) -> Option<u16> {
    u16::try_from(property.values.len()).ok()
}

fn pascal_ident(name: &str) -> proc_macro2::Ident {
    valence_build_utils::ident(name.to_pascal_case())
}
