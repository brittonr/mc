type EntityCode = super::super::entity::EntityCode;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn tokens(entity: &EntityCode) -> TokenStream {
    debug_assert!(
        !entity.block_entity_kind_variants.is_empty(),
        "block entity variants are non-empty"
    );
    let block_entity_kind_variants = &entity.block_entity_kind_variants;
    let block_entity_kind_from_id_arms = &entity.block_entity_kind_from_id_arms;
    let block_entity_kind_to_id_arms = &entity.block_entity_kind_to_id_arms;
    let block_entity_kind_from_ident_arms = &entity.block_entity_kind_from_ident_arms;
    let block_entity_kind_to_ident_arms = &entity.block_entity_kind_to_ident_arms;

    quote::quote! {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub enum BlockEntityKind { #block_entity_kind_variants }

        impl BlockEntityKind {
            pub const fn from_id(num: u32) -> Option<Self> {
                match num { #block_entity_kind_from_id_arms _ => None }
            }

            pub const fn id(self) -> u32 {
                match self { #block_entity_kind_to_id_arms }
            }

            pub fn from_ident(ident: valence_ident::Ident<&str>) -> Option<Self> {
                match ident.as_str() { #block_entity_kind_from_ident_arms _ => None }
            }

            pub fn ident(self) -> valence_ident::Ident<&'static str> {
                match self { #block_entity_kind_to_ident_arms }
            }
        }
    }
}
