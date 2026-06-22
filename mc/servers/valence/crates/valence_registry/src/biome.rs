//! Contains biomes and the biome registry. Minecraft's default biomes are added
//! to the registry by default.
//!
//! ### **NOTE:**
//! - Modifying the biome registry after the server has started can break
//!   invariants within instances and clients! Make sure there are no instances
//!   or clients spawned before mutating.
//! - A biome named "minecraft:plains" must exist. Otherwise, vanilla clients
//!   will be disconnected.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
pub use valence_ident::{ident, Ident};

type Registry<I, V> = crate::Registry<I, V>;
type RegistryCodec = crate::codec::RegistryCodec;
type RegistryValue = crate::codec::RegistryValue;

pub struct BiomePlugin;

impl Plugin for BiomePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BiomeRegistry>()
            .add_systems(PreStartup, load_default_biomes)
            .add_systems(PostUpdate, update_biome_registry.before(crate::RegistrySet));
    }
}

fn load_default_biomes(mut reg: ResMut<BiomeRegistry>, codec: Res<RegistryCodec>) {
    let mut helper = move || -> anyhow::Result<()> {
        for value in codec.registry(BiomeRegistry::KEY) {
            let biome = <Biome as serde::Deserialize>::deserialize(value.element.clone())?;

            reg.insert(value.name.clone(), biome);
        }

        // Move "plains" to the front so that `BiomeId::default()` is the ID of plains.
        reg.swap_to_front(ident!("plains"));

        Ok(())
    };

    if let Err(e) = helper() {
        tracing::error!("failed to load default biomes from registry codec: {e:#}");
    }
}

fn update_biome_registry(reg: Res<BiomeRegistry>, mut codec: ResMut<RegistryCodec>) {
    if reg.is_changed() {
        let biomes = codec.registry_mut(BiomeRegistry::KEY);

        biomes.clear();

        biomes.extend(reg.iter().filter_map(|(_, name, biome)| {
            let Ok(element) = <Biome as serde::Serialize>::serialize(
                biome,
                valence_nbt::serde::CompoundSerializer,
            ) else {
                tracing::error!("failed to serialize biome {name}");
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
pub struct BiomeRegistry {
    reg: Registry<BiomeId, Biome>,
}

impl BiomeRegistry {
    pub const KEY: Ident<&'static str> = ident!("worldgen/biome");
}

// Compatibility API: registry wrappers historically dereference to their
// backing registry.
#[allow(unknown_lints, deref_polymorphism)]
impl std::ops::Deref for BiomeRegistry {
    type Target = Registry<BiomeId, Biome>;

    fn deref(&self) -> &Self::Target {
        &self.reg
    }
}

// Compatibility API: registry wrappers historically dereference to their
// backing registry.
#[allow(unknown_lints, deref_polymorphism)]
impl std::ops::DerefMut for BiomeRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.reg
    }
}

const BIOME_ID_MAX_INDEX: usize = 4_294_967_295;
const INVALID_BIOME_ID: u32 = u32::MAX;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct BiomeId(u32);

impl BiomeId {
    pub const DEFAULT: Self = BiomeId(0);
}

impl crate::RegistryIdx for BiomeId {
    const MAX: usize = BIOME_ID_MAX_INDEX;

    #[inline]
    fn to_index(self) -> usize {
        let Ok(index) = usize::try_from(self.0) else {
            return BIOME_ID_MAX_INDEX;
        };
        index
    }

    #[inline]
    fn from_index(idx: usize) -> Self {
        let bounded = idx.min(BIOME_ID_MAX_INDEX);
        let Ok(index) = u32::try_from(bounded) else {
            return Self(INVALID_BIOME_ID);
        };
        Self(index)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Biome {
    pub downfall: f32,
    pub effects: BiomeEffects,
    pub has_precipitation: bool,
    pub temperature: f32,
    // TODO: more stuff.
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct BiomeEffects {
    pub fog_color: u32,
    pub sky_color: u32,
    pub water_color: u32,
    pub water_fog_color: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grass_color: Option<u32>,
    // TODO: more stuff.
}

impl Default for Biome {
    fn default() -> Self {
        Self {
            downfall: 0.4,
            effects: BiomeEffects::default(),
            has_precipitation: true,
            temperature: 0.8,
        }
    }
}

impl Default for BiomeEffects {
    fn default() -> Self {
        Self {
            fog_color: 12638463,
            sky_color: 7907327,
            water_color: 4159204,
            water_fog_color: 329011,
            grass_color: None,
        }
    }
}
