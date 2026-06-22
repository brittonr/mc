type TokenStream = proc_macro2::TokenStream;

mod entity;
mod kind;
mod property;
mod state;

pub(super) fn tokens(parts: super::GeneratedBlockCode) -> TokenStream {
    let block_state = state::tokens(&parts.state);
    let block_kind = kind::tokens(&parts.kind);
    let prop_name = property::name_tokens(&parts.properties);
    let prop_value = property::value_tokens(&parts.properties);
    let block_entity = entity::tokens(&parts.entity);

    quote::quote! {
        #block_state
        #block_kind
        #prop_name
        #prop_value
        #block_entity
    }
}
