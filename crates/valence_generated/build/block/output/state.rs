type StateCode = super::super::state::StateCode;
type TokenStream = proc_macro2::TokenStream;

pub(super) fn tokens(state: &StateCode) -> TokenStream {
    debug_assert_eq!(
        state.shape_count,
        state.shapes.len(),
        "shape count matches shapes"
    );
    let core_methods = core_methods(state);
    let property_methods = property_methods(state);
    let query_methods = query_methods(state);
    let shape_methods = shape_methods(state);
    let default_block_states = &state.default_block_states;

    quote::quote! {
        #[doc = "Represents the state of a block. This does not include block entity data such as"]
        #[doc = "the text on a sign, the design on a banner, or the content of a spawner."]
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
        pub struct BlockState(u16);

        impl BlockState {
            #core_methods
            #property_methods
            #query_methods
            #shape_methods
            #default_block_states
        }
    }
}

fn core_methods(state: &StateCode) -> TokenStream {
    debug_assert!(
        state.max_state_id > 0,
        "generated block state IDs are non-empty"
    );
    let max_state_id = state.max_state_id;
    let state_to_kind_arms = &state.state_to_kind_arms;
    let kind_to_state_arms = &state.kind_to_state_arms;
    let state_to_wall_variant_arms = &state.state_to_wall_variant_arms;

    quote::quote! {
        #[doc = "Returns the default block state for a given block type."]
        pub const fn from_kind(kind: BlockKind) -> Self {
            match kind { #kind_to_state_arms }
        }

        #[doc = "Constructs a block state from a raw block state ID."]
        #[doc = ""]
        #[doc = "If the given ID is invalid, `None` is returned."]
        pub const fn from_raw(id: u16) -> Option<Self> {
            if id <= #max_state_id { Some(Self(id)) } else { None }
        }

        #[doc = "Returns the [`BlockKind`] of this block state."]
        pub const fn to_kind(self) -> BlockKind {
            match self.0 { #state_to_kind_arms _ => unreachable!() }
        }

        #[doc = "Converts this block state to its underlying raw block state ID."]
        #[doc = ""]
        #[doc = "The original block state can be recovered with [`BlockState::from_raw`]."]
        pub const fn to_raw(self) -> u16 { self.0 }

        #[doc = "Returns the maximum block state ID."]
        pub const fn max_raw() -> u16 { #max_state_id }

        #[doc = "Returns the wall variant of the block state."]
        #[doc = ""]
        #[doc = "If the given block state doesn't have a wall variant, `None` is returned."]
        pub const fn wall_block_id(self) -> Option<Self> {
            match self { #state_to_wall_variant_arms _ => None }
        }
    }
}

fn property_methods(state: &StateCode) -> TokenStream {
    debug_assert!(
        !state.get_arms.is_empty(),
        "generated property getter arms are non-empty"
    );
    let get_arms = &state.get_arms;
    let set_arms = &state.set_arms;

    quote::quote! {
        #[doc = "Gets the value of the property with the given name from this block."]
        #[doc = ""]
        #[doc = "If this block does not have the property, then `None` is returned."]
        pub const fn get(self, name: PropName) -> Option<PropValue> {
            match self.to_kind() { #get_arms _ => return None }
        }

        #[doc = "Sets the value of a property on this block, returning the modified block."]
        #[doc = ""]
        #[doc = "If this block does not have the given property or the property value is invalid,"]
        #[doc = "then the original block is returned unchanged."]
        #[must_use]
        pub const fn set(self, name: PropName, val: PropValue) -> Self {
            match self.to_kind() { #set_arms _ => self }
        }
    }
}

fn query_methods(state: &StateCode) -> TokenStream {
    debug_assert!(
        !state.state_to_kind_arms.is_empty(),
        "generated state arms are non-empty"
    );
    let state_to_opaque_arms = &state.state_to_opaque_arms;
    let state_to_replaceable_arms = &state.state_to_replaceable_arms;
    let state_to_blocks_motion_arms = &state.state_to_blocks_motion_arms;
    let state_to_luminance_arms = &state.state_to_luminance_arms;
    let state_to_block_entity_type_arms = &state.state_to_block_entity_type_arms;

    quote::quote! {
        #[doc = "If this block is `air`, `cave_air` or `void_air`."]
        pub const fn is_air(self) -> bool {
            matches!(self, BlockState::AIR | BlockState::CAVE_AIR | BlockState::VOID_AIR)
        }

        #[doc = "If this block is water or lava."]
        pub const fn is_liquid(self) -> bool {
            matches!(self.to_kind(), BlockKind::Water | BlockKind::Lava)
        }

        pub const fn is_opaque(self) -> bool {
            match self.0 { #state_to_opaque_arms _ => true }
        }

        pub const fn is_replaceable(self) -> bool {
            match self.0 { #state_to_replaceable_arms _ => false }
        }

        pub const fn blocks_motion(self) -> bool {
            match self.0 { #state_to_blocks_motion_arms _ => false }
        }

        pub const fn luminance(self) -> u8 {
            match self.0 { #state_to_luminance_arms _ => 0 }
        }

        pub const fn block_entity_kind(self) -> Option<BlockEntityKind> {
            let kind = match self.0 { #state_to_block_entity_type_arms _ => None };
            match kind {
                Some(id) => BlockEntityKind::from_id(id),
                None => None,
            }
        }
    }
}

fn shape_methods(state: &StateCode) -> TokenStream {
    debug_assert_eq!(
        state.shape_count,
        state.shapes.len(),
        "shape count matches shapes"
    );
    let shape_count = state.shape_count;
    let shapes = &state.shapes;
    let state_to_collision_shapes_arms = &state.state_to_collision_shapes_arms;

    quote::quote! {
        #[expect(clippy::large_stack_arrays, reason = "generated shape table is intentionally static")]
        const SHAPES: [valence_math::Aabb; #shape_count] = [#(#shapes,)*];

        pub fn collision_shapes(self) -> impl ExactSizeIterator<Item = valence_math::Aabb> + std::iter::FusedIterator + Clone {
            let shape_idxs: &'static [u16] = match self.0 {
                #state_to_collision_shapes_arms
                _ => &[],
            };

            shape_idxs.into_iter().map(|idx| Self::SHAPES[usize::from(*idx)])
        }
    }
}
