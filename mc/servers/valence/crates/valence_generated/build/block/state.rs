use heck::{ToPascalCase, ToShoutySnakeCase};
use quote::ToTokens;

mod property;

type Block = super::Block;
type Shape = super::Shape;
type TokenStream = proc_macro2::TokenStream;

pub(super) struct StateCode {
    pub max_state_id: u16,
    pub state_to_kind_arms: TokenStream,
    pub state_to_luminance_arms: TokenStream,
    pub state_to_opaque_arms: TokenStream,
    pub state_to_replaceable_arms: TokenStream,
    pub state_to_blocks_motion_arms: TokenStream,
    pub shapes: Vec<TokenStream>,
    pub shape_count: usize,
    pub state_to_collision_shapes_arms: TokenStream,
    pub get_arms: TokenStream,
    pub set_arms: TokenStream,
    pub default_block_states: TokenStream,
    pub state_to_wall_variant_arms: TokenStream,
    pub state_to_block_entity_type_arms: TokenStream,
    pub kind_to_state_arms: TokenStream,
}

pub(super) fn build(blocks: &[Block], shapes: &[Shape]) -> StateCode {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    StateCode {
        max_state_id: blocks
            .iter()
            .filter_map(Block::max_state_id)
            .max()
            .unwrap_or(0),
        state_to_kind_arms: state_to_kind_arms(blocks),
        state_to_luminance_arms: state_to_luminance_arms(blocks),
        state_to_opaque_arms: state_to_opaque_arms(blocks),
        state_to_replaceable_arms: state_to_replaceable_arms(blocks),
        state_to_blocks_motion_arms: state_to_blocks_motion_arms(blocks),
        shapes: shape_tokens(shapes),
        shape_count: shapes.len(),
        state_to_collision_shapes_arms: state_to_collision_shapes_arms(blocks),
        get_arms: property::get_arms(blocks),
        set_arms: property::set_arms(blocks),
        default_block_states: default_block_states(blocks),
        state_to_wall_variant_arms: state_to_wall_variant_arms(blocks),
        state_to_block_entity_type_arms: state_to_block_entity_type_arms(blocks),
        kind_to_state_arms: kind_to_state_arms(blocks),
    }
}

fn state_to_kind_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .map(|block| {
            let name = pascal_ident(&block.name);
            let mut token_stream = TokenStream::new();
            let Some(min_id) = block.min_state_id() else {
                return token_stream;
            };
            let Some(max_id) = block.max_state_id() else {
                return token_stream;
            };

            if min_id == max_id {
                quote::quote!(#min_id).to_tokens(&mut token_stream);
            } else {
                for id in min_id..max_id {
                    quote::quote!(#id | ).to_tokens(&mut token_stream);
                }
                quote::quote!(#max_id).to_tokens(&mut token_stream);
            }
            quote::quote!(=> BlockKind::#name,).to_tokens(&mut token_stream);
            token_stream
        })
        .collect()
}

fn state_to_luminance_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .flat_map(|block| {
            block
                .states
                .iter()
                .filter(|state| state.luminance != 0)
                .map(|state| {
                    let id = state.id;
                    let luminance = state.luminance;
                    quote::quote! {
                        #id => #luminance,
                    }
                })
        })
        .collect()
}

fn state_to_opaque_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .flat_map(|block| {
            block
                .states
                .iter()
                .filter(|state| !state.opaque)
                .map(|state| {
                    let id = state.id;
                    quote::quote! {
                        #id => false,
                    }
                })
        })
        .collect()
}

fn state_to_replaceable_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .flat_map(|block| {
            block
                .states
                .iter()
                .filter(|state| state.replaceable)
                .map(|state| {
                    let id = state.id;
                    quote::quote! {
                        #id => true,
                    }
                })
        })
        .collect()
}

fn state_to_blocks_motion_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .flat_map(|block| {
            block
                .states
                .iter()
                .filter(|state| state.blocks_motion)
                .map(|state| {
                    let id = state.id;
                    quote::quote! {
                        #id => true,
                    }
                })
        })
        .collect()
}

fn shape_tokens(shapes: &[Shape]) -> Vec<TokenStream> {
    shapes
        .iter()
        .map(|shape| {
            let min_x = shape.min_x;
            let min_y = shape.min_y;
            let min_z = shape.min_z;
            let max_x = shape.max_x;
            let max_y = shape.max_y;
            let max_z = shape.max_z;
            quote::quote! {
                valence_math::Aabb::new_unchecked(
                    valence_math::DVec3::new(#min_x, #min_y, #min_z),
                    valence_math::DVec3::new(#max_x, #max_y, #max_z),
                )
            }
        })
        .collect()
}

fn state_to_collision_shapes_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .flat_map(|block| {
            block.states.iter().map(|state| {
                let id = state.id;
                let collision_shapes = &state.collision_shapes;
                quote::quote! {
                    #id => &[#(#collision_shapes),*],
                }
            })
        })
        .collect()
}

fn default_block_states(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .map(|block| {
            let name = shouty_ident(&block.name);
            let state = block.default_state_id;
            let doc = format!("The default block state for `{}`.", block.name);
            quote::quote! {
                #[doc = #doc]
                pub const #name: BlockState = BlockState(#state);
            }
        })
        .collect()
}

fn state_to_wall_variant_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .filter(|block| block.wall_variant_id.is_some())
        .map(|block| {
            let block_name = shouty_ident(&block.name);
            let Some(wall_variant_id) = block.wall_variant_id else {
                return TokenStream::new();
            };
            let Some(wall_variant) = blocks.get(usize::from(wall_variant_id)) else {
                return TokenStream::new();
            };
            let wall_block_name = shouty_ident(&wall_variant.name);
            quote::quote! {
                BlockState::#block_name => Some(BlockState::#wall_block_name),
            }
        })
        .collect()
}

fn state_to_block_entity_type_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .flat_map(|block| {
            block.states.iter().filter_map(|state| {
                let id = state.id;
                let block_entity_type = state.block_entity_type?;
                Some(quote::quote! {
                    #id => Some(#block_entity_type),
                })
            })
        })
        .collect()
}

fn kind_to_state_arms(blocks: &[Block]) -> TokenStream {
    debug_assert!(!blocks.is_empty(), "generated block list is non-empty");
    blocks
        .iter()
        .map(|block| {
            let kind = pascal_ident(&block.name);
            let state = shouty_ident(&block.name);
            quote::quote! {
                BlockKind::#kind => BlockState::#state,
            }
        })
        .collect()
}

fn pascal_ident(name: &str) -> proc_macro2::Ident {
    valence_build_utils::ident(name.to_pascal_case())
}

fn shouty_ident(name: &str) -> proc_macro2::Ident {
    valence_build_utils::ident(name.to_shouty_snake_case())
}
