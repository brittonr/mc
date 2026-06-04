type TokenStream = proc_macro2::TokenStream;

mod entity;
mod kind;
mod output;
mod property;
mod state;

#[derive(serde::Deserialize, Clone, Debug)]
struct TopLevel {
    blocks: Vec<Block>,
    shapes: Vec<Shape>,
    block_entity_types: Vec<BlockEntityKind>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Block {
    id: u16,
    item_id: u16,
    wall_variant_id: Option<u16>,
    translation_key: String,
    name: String,
    properties: Vec<Property>,
    default_state_id: u16,
    states: Vec<State>,
}

impl Block {
    fn min_state_id(&self) -> Option<u16> {
        self.states.iter().map(|state| state.id).min()
    }

    fn max_state_id(&self) -> Option<u16> {
        self.states.iter().map(|state| state.id).max()
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
struct BlockEntityKind {
    id: u32,
    ident: String,
    name: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Property {
    name: String,
    values: Vec<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct State {
    id: u16,
    luminance: u8,
    opaque: bool,
    replaceable: bool,
    blocks_motion: bool,
    collision_shapes: Vec<u16>,
    block_entity_type: Option<u32>,
}

#[derive(serde::Deserialize, Clone, Debug)]
struct Shape {
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

struct GeneratedBlockCode {
    state: state::StateCode,
    kind: kind::KindCode,
    properties: property::PropertyCode,
    entity: entity::EntityCode,
}

pub(crate) fn build() -> anyhow::Result<TokenStream> {
    valence_build_utils::rerun_if_changed(["extracted/blocks.json"]);

    let top_level: TopLevel = serde_json::from_str(include_str!("../extracted/blocks.json"))?;
    debug_assert!(
        !top_level.blocks.is_empty(),
        "generated block list is non-empty"
    );
    Ok(output::tokens(generated_code(&top_level)))
}

fn generated_code(top_level: &TopLevel) -> GeneratedBlockCode {
    GeneratedBlockCode {
        state: state::build(&top_level.blocks, &top_level.shapes),
        kind: kind::build(&top_level.blocks),
        properties: property::build(&top_level.blocks),
        entity: entity::build(&top_level.block_entity_types),
    }
}
