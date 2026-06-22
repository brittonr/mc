use heck::ToPascalCase;

type BlockEntityKind = super::BlockEntityKind;
type TokenStream = proc_macro2::TokenStream;

pub(super) struct EntityCode {
    pub block_entity_kind_variants: TokenStream,
    pub block_entity_kind_from_id_arms: TokenStream,
    pub block_entity_kind_to_id_arms: TokenStream,
    pub block_entity_kind_from_ident_arms: TokenStream,
    pub block_entity_kind_to_ident_arms: TokenStream,
}

pub(super) fn build(block_entity_types: &[BlockEntityKind]) -> EntityCode {
    debug_assert!(
        !block_entity_types.is_empty(),
        "generated block-entity list is non-empty"
    );
    EntityCode {
        block_entity_kind_variants: variants(block_entity_types),
        block_entity_kind_from_id_arms: from_id_arms(block_entity_types),
        block_entity_kind_to_id_arms: to_id_arms(block_entity_types),
        block_entity_kind_from_ident_arms: from_ident_arms(block_entity_types),
        block_entity_kind_to_ident_arms: to_ident_arms(block_entity_types),
    }
}

fn variants(block_entity_types: &[BlockEntityKind]) -> TokenStream {
    block_entity_types
        .iter()
        .map(|block_entity| {
            let name = variant_name(block_entity);
            let doc = format!(
                "The block entity type `{}` (ID {}).",
                block_entity.name, block_entity.id
            );
            quote::quote! {
                #[doc = #doc]
                #name,
            }
        })
        .collect()
}

fn from_id_arms(block_entity_types: &[BlockEntityKind]) -> TokenStream {
    block_entity_types
        .iter()
        .map(|block_entity| {
            let id = block_entity.id;
            let name = variant_name(block_entity);
            quote::quote! {
                #id => Some(Self::#name),
            }
        })
        .collect()
}

fn to_id_arms(block_entity_types: &[BlockEntityKind]) -> TokenStream {
    block_entity_types
        .iter()
        .map(|block_entity| {
            let id = block_entity.id;
            let name = variant_name(block_entity);
            quote::quote! {
                Self::#name => #id,
            }
        })
        .collect()
}

fn from_ident_arms(block_entity_types: &[BlockEntityKind]) -> TokenStream {
    block_entity_types
        .iter()
        .map(|block_entity| {
            let name = variant_name(block_entity);
            let ident = &block_entity.ident;
            quote::quote! {
                #ident => Some(Self::#name),
            }
        })
        .collect()
}

fn to_ident_arms(block_entity_types: &[BlockEntityKind]) -> TokenStream {
    block_entity_types
        .iter()
        .map(|block_entity| {
            let name = variant_name(block_entity);
            let ident = &block_entity.ident;
            quote::quote! {
                Self::#name => valence_ident::ident!(#ident),
            }
        })
        .collect()
}

fn variant_name(block_entity: &BlockEntityKind) -> proc_macro2::Ident {
    valence_build_utils::ident(block_entity.name.to_pascal_case())
}
