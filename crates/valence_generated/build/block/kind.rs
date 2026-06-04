use heck::ToPascalCase;

type Block = super::Block;
type TokenStream = proc_macro2::TokenStream;

pub(super) struct KindCode {
    pub kind_to_translation_key_arms: TokenStream,
    pub block_kind_variants: Vec<proc_macro2::Ident>,
    pub block_kind_from_str_arms: TokenStream,
    pub block_kind_to_str_arms: TokenStream,
    pub block_kind_props_arms: TokenStream,
    pub block_kind_to_item_kind_arms: TokenStream,
    pub block_kind_from_item_kind_arms: TokenStream,
    pub block_kind_from_raw_arms: TokenStream,
    pub block_kind_count: usize,
}

pub(super) fn build(blocks: &[Block]) -> KindCode {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    KindCode {
        kind_to_translation_key_arms: translation_key_arms(blocks),
        block_kind_variants: variants(blocks),
        block_kind_from_str_arms: from_str_arms(blocks),
        block_kind_to_str_arms: to_str_arms(blocks),
        block_kind_props_arms: props_arms(blocks),
        block_kind_to_item_kind_arms: to_item_kind_arms(blocks),
        block_kind_from_item_kind_arms: from_item_kind_arms(blocks),
        block_kind_from_raw_arms: from_raw_arms(blocks),
        block_kind_count: blocks.len(),
    }
}

fn translation_key_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .map(|block| {
            let kind = variant_name(block);
            let translation_key = &block.translation_key;
            quote::quote! {
                Self::#kind => #translation_key,
            }
        })
        .collect()
}

fn variants(blocks: &[Block]) -> Vec<proc_macro2::Ident> {
    blocks.iter().map(variant_name).collect()
}

fn from_str_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .map(|block| {
            let name = &block.name;
            let name_ident = variant_name(block);
            quote::quote! {
                #name => Some(BlockKind::#name_ident),
            }
        })
        .collect()
}

fn to_str_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .map(|block| {
            let name = &block.name;
            let name_ident = variant_name(block);
            quote::quote! {
                BlockKind::#name_ident => #name,
            }
        })
        .collect()
}

fn props_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .filter(|block| !block.properties.is_empty())
        .map(|block| {
            let name = variant_name(block);
            let prop_names = block
                .properties
                .iter()
                .map(|property| valence_build_utils::ident(property.name.to_pascal_case()));

            quote::quote! {
                Self::#name => &[#(PropName::#prop_names,)*],
            }
        })
        .collect()
}

fn to_item_kind_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .map(|block| {
            let name = variant_name(block);
            let item_id = block.item_id;
            quote::quote! {
                BlockKind::#name => #item_id,
            }
        })
        .collect()
}

fn from_item_kind_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .filter(|block| block.item_id != 0)
        .map(|block| {
            let name = variant_name(block);
            let item_id = block.item_id;
            quote::quote! {
                #item_id => Some(BlockKind::#name),
            }
        })
        .collect()
}

fn from_raw_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .map(|block| {
            let name = variant_name(block);
            let id = block.id;
            quote::quote! {
                #id => Some(BlockKind::#name),
            }
        })
        .collect()
}

fn variant_name(block: &Block) -> proc_macro2::Ident {
    valence_build_utils::ident(block.name.to_pascal_case())
}
