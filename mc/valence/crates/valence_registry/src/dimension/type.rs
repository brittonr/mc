//! Contains dimension types and the dimension type registry. Minecraft's
//! default dimensions are added to the registry by default.
//!
//! ### **NOTE:**
//! - Modifying the dimension type registry after the server has started can
//!   break invariants within instances and clients! Make sure there are no
//!   instances or clients spawned before mutating.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
pub use valence_ident::{ident, Ident};

type RegistryCodec = crate::codec::RegistryCodec;
type RegistryValue = crate::codec::RegistryValue;
type Registry<I, V> = crate::Registry<I, V>;
pub struct DimensionTypePlugin;

impl Plugin for DimensionTypePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DimensionTypeRegistry>()
            .add_systems(PreStartup, load_default_dimension_types)
            .add_systems(
                PostUpdate,
                update_dimension_type_registry.before(crate::RegistrySet),
            );
    }
}

/// Loads the default dimension types from the registry codec.
fn load_default_dimension_types(mut reg: ResMut<DimensionTypeRegistry>, codec: Res<RegistryCodec>) {
    let mut helper = move || -> anyhow::Result<()> {
        for value in codec.registry(DimensionTypeRegistry::KEY) {
            let mut dimension_type =
                <DimensionType as serde::Deserialize>::deserialize(value.element.clone())?;

            // HACK: We don't have a lighting engine implemented. To avoid shrouding the
            // world in darkness, give all dimensions the max ambient light.
            dimension_type.ambient_light = 1.0;

            reg.insert(value.name.clone(), dimension_type);
        }

        Ok(())
    };

    if let Err(e) = helper() {
        tracing::error!("failed to load default dimension types from registry codec: {e:#}");
    }
}

/// Updates the registry codec as the dimension type registry is modified by
/// users.
fn update_dimension_type_registry(
    reg: Res<DimensionTypeRegistry>,
    mut codec: ResMut<RegistryCodec>,
) {
    if reg.is_changed() {
        let dimension_types = codec.registry_mut(DimensionTypeRegistry::KEY);

        dimension_types.clear();

        dimension_types.extend(reg.iter().filter_map(|(_, name, dim)| {
            let Ok(element) = <DimensionType as serde::Serialize>::serialize(
                dim,
                valence_nbt::serde::CompoundSerializer,
            ) else {
                tracing::error!("failed to serialize dimension type {name}");
                return None;
            };
            Some(RegistryValue {
                name: name.into(),
                element,
            })
        }));
    }
}

#[derive(Resource, Default, Debug)]
pub struct DimensionTypeRegistry {
    reg: Registry<DimensionTypeId, DimensionType>,
}

impl DimensionTypeRegistry {
    pub const KEY: Ident<&'static str> = ident!("dimension_type");
}

const DIMENSION_TYPE_ID_MAX_INDEX: usize = 65_535;
const INVALID_DIMENSION_TYPE_ID: u16 = u16::MAX;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct DimensionTypeId(u16);

impl crate::RegistryIdx for DimensionTypeId {
    const MAX: usize = DIMENSION_TYPE_ID_MAX_INDEX;

    fn to_index(self) -> usize {
        usize::from(self.0)
    }

    fn from_index(idx: usize) -> Self {
        let bounded = idx.min(DIMENSION_TYPE_ID_MAX_INDEX);
        let Ok(idx) = u16::try_from(bounded) else {
            return Self(INVALID_DIMENSION_TYPE_ID);
        };
        Self(idx)
    }
}

// Compatibility API: registry wrappers historically dereference to their
// backing registry.
#[allow(unknown_lints, deref_polymorphism)]
impl std::ops::Deref for DimensionTypeRegistry {
    type Target = Registry<DimensionTypeId, DimensionType>;

    fn deref(&self) -> &Self::Target {
        &self.reg
    }
}

// Compatibility API: registry wrappers historically dereference to their
// backing registry.
#[allow(unknown_lints, deref_polymorphism)]
impl std::ops::DerefMut for DimensionTypeRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.reg
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct DimensionType {
    pub ambient_light: f32,
    pub bed_works: bool,
    pub coordinate_scale: f64,
    pub effects: DimensionEffects,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_time: Option<i32>,
    pub has_ceiling: bool,
    pub has_raids: bool,
    pub has_skylight: bool,
    pub height: i32,
    pub infiniburn: String,
    pub logical_height: i32,
    pub min_y: i32,
    pub monster_spawn_block_light_limit: i32,
    pub monster_spawn_light_level: MonsterSpawnLightLevel,
    pub natural: bool,
    pub piglin_safe: bool,
    pub respawn_anchor_works: bool,
    pub ultrawarm: bool,
}

impl Default for DimensionType {
    fn default() -> Self {
        Self {
            ambient_light: 0.0,
            bed_works: true,
            coordinate_scale: 1.0,
            effects: DimensionEffects::default(),
            fixed_time: None,
            has_ceiling: false,
            has_raids: true,
            has_skylight: true,
            height: 384,
            infiniburn: "#minecraft:infiniburn_overworld".into(),
            logical_height: 384,
            min_y: -64,
            monster_spawn_block_light_limit: 0,
            monster_spawn_light_level: MonsterSpawnLightLevel::Int(7),
            natural: true,
            piglin_safe: false,
            respawn_anchor_works: false,
            ultrawarm: false,
        }
    }
}

/// Determines what skybox/fog effects to use in dimensions.
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum DimensionEffects {
    #[serde(rename = "minecraft:overworld")]
    #[default]
    Overworld,
    #[serde(rename = "minecraft:the_nether")]
    TheNether,
    #[serde(rename = "minecraft:the_end")]
    TheEnd,
}

#[derive(Copy, Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevel {
    Int(i32),
    Tagged(MonsterSpawnLightLevelTagged),
}

#[derive(Copy, Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum MonsterSpawnLightLevelTagged {
    #[serde(rename = "minecraft:uniform")]
    Uniform {
        min_inclusive: i32,
        max_inclusive: i32,
    },
}

impl From<i32> for MonsterSpawnLightLevel {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}
