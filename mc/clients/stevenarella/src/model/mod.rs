pub mod liquid;

use crate::render;
use crate::resources;
use crate::shared::Direction;
use crate::world;
use crate::world::block::{Block, TintType};
use byteorder::{NativeEndian, WriteBytesExt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, RwLock};

use crate::types::hash::FNVHash;
use log::error;
use std::hash::BuildHasherDefault;

use image::GenericImageView;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct Factory {
    resources: resources::SharedManager,
    pub textures: Arc<RwLock<render::TextureManager>>,

    models: HashMap<Key, StateModel, BuildHasherDefault<FNVHash>>,

    grass_colors: image::DynamicImage,
    foliage_colors: image::DynamicImage,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Key(String, String);

macro_rules! try_log {
    ($e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => {
                error!("Error loading model {:?}", err);
                return false;
            }
        }
    };
    (opt $e:expr) => {
        match $e {
            Ok(val) => val,
            Err(err) => {
                error!("Error loading model {:?}", err);
                return None;
            }
        }
    };
}

const MINECRAFT_PLUGIN: &str = "minecraft";
const BLOCK_MODEL_PREFIX: &str = "block/";
const BUILTIN_MODEL_PREFIX: &str = "builtin/";
const ITEM_MODEL_PREFIX: &str = "item/";
const MODEL_FILE_PREFIX: &str = "models/";
const MODEL_FILE_SUFFIX: &str = ".json";

#[derive(Debug, PartialEq, Eq)]
struct ResourceReference {
    plugin: String,
    location: String,
}

fn parse_resource_reference(default_plugin: &str, reference: &str) -> Option<ResourceReference> {
    if reference.is_empty() {
        return None;
    }
    let (plugin, location) = reference
        .split_once(':')
        .unwrap_or((default_plugin, reference));
    if plugin.is_empty() || location.is_empty() {
        return None;
    }
    Some(ResourceReference {
        plugin: plugin.to_owned(),
        location: location.to_owned(),
    })
}

fn is_absolute_model_location(location: &str) -> bool {
    location.starts_with(BLOCK_MODEL_PREFIX)
        || location.starts_with(BUILTIN_MODEL_PREFIX)
        || location.starts_with(ITEM_MODEL_PREFIX)
}

fn model_file_path(location: &str) -> String {
    format!("{}{}{}", MODEL_FILE_PREFIX, location, MODEL_FILE_SUFFIX)
}

fn modern_block_model_location(location: &str) -> String {
    if let Some(color) = location.strip_suffix("_stained_hardened_clay") {
        let color = if color == "silver" {
            "light_gray"
        } else {
            color
        };
        return format!("{}_terracotta", color);
    }
    match location {
        "brick_block" => "bricks".to_owned(),
        "fence" => "oak_fence".to_owned(),
        "fence_gate" => "oak_fence_gate".to_owned(),
        "grass_path" => "dirt_path".to_owned(),
        "hardened_clay" => "terracotta".to_owned(),
        "lit_pumpkin" => "jack_o_lantern".to_owned(),
        "melon_block" => "melon".to_owned(),
        "mob_spawner" => "spawner".to_owned(),
        "nether_brick" => "nether_bricks".to_owned(),
        "noteblock" => "note_block".to_owned(),
        "red_nether_brick" => "red_nether_bricks".to_owned(),
        "reeds" => "sugar_cane".to_owned(),
        "silver_glazed_terracotta" => "light_gray_glazed_terracotta".to_owned(),
        "standing_sign" => "oak_sign".to_owned(),
        "wall_sign" => "oak_wall_sign".to_owned(),
        "waterlily" => "lily_pad".to_owned(),
        "web" => "cobweb".to_owned(),
        "wooden_button" => "oak_button".to_owned(),
        "wooden_door" => "oak_door".to_owned(),
        "wooden_pressure_plate" => "oak_pressure_plate".to_owned(),
        _ => location.to_owned(),
    }
}

fn block_state_model_resource(default_plugin: &str, reference: &str) -> Option<ResourceReference> {
    let mut resource = parse_resource_reference(default_plugin, reference)?;
    if is_absolute_model_location(&resource.location) {
        resource.location = model_file_path(&resource.location);
    } else {
        let location = if resource.plugin == MINECRAFT_PLUGIN {
            modern_block_model_location(&resource.location)
        } else {
            resource.location.clone()
        };
        resource.location = model_file_path(&format!("{}{}", BLOCK_MODEL_PREFIX, location));
    }
    Some(resource)
}

fn parent_model_resource(default_plugin: &str, reference: &str) -> Option<ResourceReference> {
    let mut resource = parse_resource_reference(default_plugin, reference)?;
    resource.location = model_file_path(&resource.location);
    Some(resource)
}

// ---------------------------------------------------------------------------
// Pure model decision cores.
//
// These free functions are the deterministic functional core of the model
// module: they take explicit inputs and return a decision without touching the
// resource manager, JSON readers, texture manager, random sources, renderer
// allocation, or logging. The `Factory` and render/light shells keep every side
// effect and delegate the model decisions to these cores so block model
// behavior stays testable in isolation.
// ---------------------------------------------------------------------------

/// Vertex light channel scale used to pack a 0..15 light sample.
const LIGHT_PACK_SCALE: u16 = 4000;

/// Minecraft models use 16 units per block.
const MODEL_UNITS_PER_BLOCK: f64 = 16.0;

/// Minecraft face UVs live in a 0..16 model space; texture lookups multiply by
/// this extent when mapping UV coordinates to texture pixels.
const MODEL_UV_EXTENT: i16 = 16;

/// Resolve the builtin model type from a resolved parent location.
///
/// Mirrors the `builtin:` parent handling inside `Factory::parse_model` so the
/// inheritance decision is testable without resource reads.
fn resolve_builtin_type(parent_location: &str) -> BuiltinType {
    match parent_location {
        "builtin/generated" => BuiltinType::Generated,
        "builtin/entity" => BuiltinType::Entity,
        "builtin/compass" => BuiltinType::Compass,
        "builtin/clock" => BuiltinType::Clock,
        _ => BuiltinType::False,
    }
}

/// Decide whether a parent model should be loaded from resources.
///
/// A parent is loaded only when it is non-empty and not a builtin model;
/// builtin parents are handled by [`resolve_builtin_type`] instead.
fn should_load_parent_model(parent: &str, parent_location: &str) -> bool {
    !parent.is_empty() && !parent_location.starts_with(BUILTIN_MODEL_PREFIX)
}

/// Decide whether a state model resolves variants through multipart rules
/// rather than a single named variant.
fn uses_multipart(state_model: &StateModel) -> bool {
    !state_model.multipart.is_empty()
}

/// Evaluate parsed multipart `when` rules against a block-matching predicate.
///
/// The matcher is supplied by the shell (typically
/// `|key, val| block.match_multipart(key, val)`), keeping this core free of
/// `Block` and world state. Empty rules match vacuously; an empty `OR` group
/// matches nothing.
fn eval_multipart_rules(rules: &[Rule], matches: &dyn Fn(&str, &str) -> bool) -> bool {
    for rule in rules {
        match rule {
            Rule::Or(sub_groups) => {
                let mut matched = false;
                for sub_rules in sub_groups {
                    if eval_multipart_rules(sub_rules, matches) {
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    return false;
                }
            }
            Rule::Match(key, val) => {
                if !matches(key.as_str(), val.as_str()) {
                    return false;
                }
            }
        }
    }
    true
}

/// Average a set of RGB samples, matching the per-pixel accumulation used by
/// biome color blending. An empty sample set returns black so callers cannot
/// divide by zero.
fn average_rgb(samples: &[(u8, u8, u8)]) -> (u8, u8, u8) {
    if samples.is_empty() {
        return (0, 0, 0);
    }
    let mut r = 0u32;
    let mut g = 0u32;
    let mut b = 0u32;
    for &(cr, cg, cb) in samples {
        r += cr as u32;
        g += cg as u32;
        b += cb as u32;
    }
    let count = samples.len() as u32;
    ((r / count) as u8, (g / count) as u8, (b / count) as u8)
}

/// Pack a single light sample into the vertex light channel.
fn pack_light_value(value: u8) -> u16 {
    (value as u16) * LIGHT_PACK_SCALE
}

/// Average accumulated smooth-lighting samples into packed vertex light.
///
/// `count` is the number of samples that produced `block_light_sum` and
/// `sky_light_sum`; a zero count returns no light so callers cannot divide by
/// zero.
fn average_packed_light(block_light_sum: u32, sky_light_sum: u32, count: u32) -> (u16, u16) {
    if count == 0 {
        return (0, 0);
    }
    (
        ((block_light_sum * (LIGHT_PACK_SCALE as u32)) / count) as u16,
        ((sky_light_sum * (LIGHT_PACK_SCALE as u32)) / count) as u16,
    )
}

/// Rotate face UV bounds by a model face rotation in degrees.
///
/// `uv` is `(ux1, ux2, uy1, uy2)` already scaled to texture pixels. Only the
/// Minecraft rotations `90`, `180`, and `270` transform the bounds; any other
/// value (including `0` and unsupported angles) leaves them unchanged so the
/// shell fails closed on invalid face data.
fn rotate_face_uv(
    uv: (i16, i16, i16, i16),
    rotation: i32,
    tw: i16,
    th: i16,
) -> (i16, i16, i16, i16) {
    let (ux1, ux2, uy1, uy2) = uv;
    match rotation {
        270 => (uy1, uy2, tw * MODEL_UV_EXTENT - ux2, tw * MODEL_UV_EXTENT - ux1),
        180 => (
            tw * MODEL_UV_EXTENT - ux2,
            tw * MODEL_UV_EXTENT - ux1,
            th * MODEL_UV_EXTENT - uy2,
            th * MODEL_UV_EXTENT - uy1,
        ),
        90 => (
            th * MODEL_UV_EXTENT - uy2,
            th * MODEL_UV_EXTENT - uy1,
            ux1,
            ux2,
        ),
        _ => uv,
    }
}

/// Select a model-space vertex coordinate from element bounds.
///
/// `base` is the precomputed unit-cube vertex coordinate (0.0 or 1.0 from
/// [`BlockVertex`]); vertices on the low face use `from`, vertices on the high
/// face use `to`, matching Minecraft element layout.
fn element_vertex_coord(base: f32, from: f64, to: f64) -> f32 {
    if base < 0.5 {
        (from / MODEL_UNITS_PER_BLOCK) as f32
    } else {
        (to / MODEL_UNITS_PER_BLOCK) as f32
    }
}

/// Select the low or high UV offset for a vertex from its unit-cube offset.
fn select_uv_offset(toffset: i16, low: i16, high: i16) -> i16 {
    if toffset == 0 {
        low
    } else {
        high
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_PLUGIN: &str = "minecraft";

    #[test]
    fn model_references_accept_namespaced_modern_paths() {
        assert_eq!(
            block_state_model_resource(DEFAULT_PLUGIN, "minecraft:block/grass_block"),
            Some(ResourceReference {
                plugin: "minecraft".to_owned(),
                location: "models/block/grass_block.json".to_owned(),
            })
        );
        assert_eq!(
            parent_model_resource(DEFAULT_PLUGIN, "minecraft:block/cube_all"),
            Some(ResourceReference {
                plugin: "minecraft".to_owned(),
                location: "models/block/cube_all.json".to_owned(),
            })
        );
    }

    #[test]
    fn model_references_keep_legacy_blockstate_model_names() {
        assert_eq!(
            block_state_model_resource(DEFAULT_PLUGIN, "oak_planks"),
            Some(ResourceReference {
                plugin: "minecraft".to_owned(),
                location: "models/block/oak_planks.json".to_owned(),
            })
        );
    }

    #[test]
    fn model_references_modernize_renamed_legacy_block_names() {
        assert_eq!(
            block_state_model_resource(DEFAULT_PLUGIN, "web"),
            Some(ResourceReference {
                plugin: "minecraft".to_owned(),
                location: "models/block/cobweb.json".to_owned(),
            })
        );
        assert_eq!(
            block_state_model_resource("example", "web"),
            Some(ResourceReference {
                plugin: "example".to_owned(),
                location: "models/block/web.json".to_owned(),
            })
        );
        assert_eq!(
            block_state_model_resource(DEFAULT_PLUGIN, "orange_stained_hardened_clay"),
            Some(ResourceReference {
                plugin: "minecraft".to_owned(),
                location: "models/block/orange_terracotta.json".to_owned(),
            })
        );
        assert_eq!(
            block_state_model_resource(DEFAULT_PLUGIN, "silver_glazed_terracotta"),
            Some(ResourceReference {
                plugin: "minecraft".to_owned(),
                location: "models/block/light_gray_glazed_terracotta.json".to_owned(),
            })
        );
    }

    #[test]
    fn model_references_reject_empty_plugin_or_location() {
        assert_eq!(parse_resource_reference(DEFAULT_PLUGIN, ""), None);
        assert_eq!(
            parse_resource_reference(DEFAULT_PLUGIN, ":block/stone"),
            None
        );
        assert_eq!(parse_resource_reference(DEFAULT_PLUGIN, "minecraft:"), None);
    }

    #[test]
    fn model_file_path_wraps_with_models_prefix_and_json_suffix() {
        assert_eq!(
            model_file_path("block/grass_block"),
            "models/block/grass_block.json",
        );
        assert_eq!(
            model_file_path("builtin/generated"),
            "models/builtin/generated.json",
        );
    }

    #[test]
    fn is_absolute_model_location_detects_known_prefixes() {
        assert!(is_absolute_model_location("block/stone"));
        assert!(is_absolute_model_location("builtin/generated"));
        assert!(is_absolute_model_location("item/stick"));
        assert!(!is_absolute_model_location("grass_block"));
    }

    #[test]
    fn block_state_model_resource_rejects_unsafe_references() {
        assert_eq!(block_state_model_resource(DEFAULT_PLUGIN, ""), None);
        assert_eq!(block_state_model_resource(DEFAULT_PLUGIN, "minecraft:"), None);
    }

    #[test]
    fn parent_model_resource_preserves_absolute_and_builtin_paths() {
        assert_eq!(
            parent_model_resource(DEFAULT_PLUGIN, "minecraft:block/cube_all"),
            Some(ResourceReference {
                plugin: "minecraft".to_owned(),
                location: "models/block/cube_all.json".to_owned(),
            }),
        );
        assert_eq!(
            parent_model_resource(DEFAULT_PLUGIN, "builtin/generated"),
            Some(ResourceReference {
                plugin: "minecraft".to_owned(),
                location: "models/builtin/generated.json".to_owned(),
            }),
        );
    }

    #[test]
    fn resolve_builtin_type_recognizes_known_builtins() {
        assert_eq!(
            resolve_builtin_type("builtin/generated"),
            BuiltinType::Generated
        );
        assert_eq!(resolve_builtin_type("builtin/entity"), BuiltinType::Entity);
        assert_eq!(resolve_builtin_type("builtin/compass"), BuiltinType::Compass);
        assert_eq!(resolve_builtin_type("builtin/clock"), BuiltinType::Clock);
    }

    #[test]
    fn resolve_builtin_type_defaults_unknown_builtins_to_false() {
        // Unknown builtin parents fall back to no builtin, preserving the
        // current behavior of treating unrecognized parents as ordinary models.
        assert_eq!(resolve_builtin_type("builtin/unknown"), BuiltinType::False);
        assert_eq!(resolve_builtin_type(""), BuiltinType::False);
        assert_eq!(resolve_builtin_type("block/cube_all"), BuiltinType::False);
    }

    #[test]
    fn should_load_parent_model_skips_empty_and_builtin_parents() {
        assert!(should_load_parent_model("block/cube_all", "block/cube_all"));
        // Missing parent: nothing to inherit.
        assert!(!should_load_parent_model("", ""));
        // Builtin parents are handled by resolve_builtin_type, not loaded.
        assert!(!should_load_parent_model(
            "builtin/generated",
            "builtin/generated",
        ));
        assert!(!should_load_parent_model("builtin/clock", "builtin/clock"));
    }

    #[test]
    fn uses_multipart_detects_multipart_state_models() {
        let variants_only = StateModel {
            variants: HashMap::with_hasher(BuildHasherDefault::default()),
            multipart: vec![],
        };
        assert!(!uses_multipart(&variants_only));

        let multipart = StateModel {
            variants: HashMap::with_hasher(BuildHasherDefault::default()),
            multipart: vec![MultipartRule {
                apply: Variants { models: vec![] },
                rules: vec![Rule::Match("variant".to_owned(), "north".to_owned())],
            }],
        };
        assert!(uses_multipart(&multipart));
    }

    #[test]
    fn state_model_selects_named_variant() {
        let mut variants = HashMap::with_hasher(BuildHasherDefault::default());
        variants.insert("east".to_owned(), Variants { models: vec![] });
        let state_model = StateModel {
            variants,
            multipart: vec![],
        };
        assert!(state_model.get_variants("east").is_some());
        assert!(state_model.get_variants("north").is_none());
    }

    #[test]
    fn eval_multipart_rules_matches_simple_rules() {
        let always = |_: &str, _: &str| true;
        let never = |_: &str, _: &str| false;
        let facing_north = |key: &str, val: &str| key == "facing" && val == "north";

        // Empty rules match vacuously.
        assert!(eval_multipart_rules(&[], &always));
        // A matching rule passes.
        assert!(eval_multipart_rules(
            &[Rule::Match("facing".to_owned(), "north".to_owned())],
            &facing_north,
        ));
        // A non-matching rule fails.
        assert!(!eval_multipart_rules(
            &[Rule::Match("facing".to_owned(), "south".to_owned())],
            &facing_north,
        ));
        // A rule the matcher never satisfies fails.
        assert!(!eval_multipart_rules(
            &[Rule::Match("facing".to_owned(), "north".to_owned())],
            &never,
        ));
    }

    #[test]
    fn eval_multipart_rules_handles_or_groups() {
        let axis_x_or_z = |key: &str, val: &str| key == "axis" && (val == "x" || val == "z");
        let rules = vec![Rule::Or(vec![
            vec![Rule::Match("axis".to_owned(), "x".to_owned())],
            vec![Rule::Match("axis".to_owned(), "z".to_owned())],
        ])];
        assert!(eval_multipart_rules(&rules, &axis_x_or_z));
        // An empty OR group matches nothing (invalid multipart rule fails closed).
        assert!(!eval_multipart_rules(&[Rule::Or(vec![])], &|_, _| true));
    }

    #[test]
    fn average_rgb_averages_samples() {
        assert_eq!(average_rgb(&[(10, 20, 30), (30, 40, 50)]), (20, 30, 40));
        // Nine equal samples collapse to the sample color (biome blend shape).
        assert_eq!(average_rgb(&[(255, 0, 0); 9]), (255, 0, 0));
    }

    #[test]
    fn average_rgb_returns_black_for_no_samples() {
        // Unsupported/empty biome inputs fail closed to black.
        assert_eq!(average_rgb(&[]), (0, 0, 0));
    }

    #[test]
    fn pack_light_value_scales_single_sample() {
        assert_eq!(pack_light_value(0), 0);
        assert_eq!(pack_light_value(15), 60_000);
    }

    #[test]
    fn average_packed_light_blends_samples() {
        // Eight samples of block light 2 and sky light 4 match the smooth
        // lighting shape used by calculate_light.
        assert_eq!(average_packed_light(16, 32, 8), (8_000, 16_000));
    }

    #[test]
    fn average_packed_light_returns_zero_without_samples() {
        // Unsupported/empty light inputs fail closed to no light instead of
        // dividing by zero.
        assert_eq!(average_packed_light(100, 100, 0), (0, 0));
    }

    #[test]
    fn rotate_face_uv_transforms_known_rotations() {
        let uv = (10, 20, 30, 40);
        let tw = 2;
        let th = 3;
        assert_eq!(
            rotate_face_uv(uv, 90, tw, th),
            (th * 16 - 40, th * 16 - 30, 10, 20),
        );
        assert_eq!(
            rotate_face_uv(uv, 180, tw, th),
            (tw * 16 - 20, tw * 16 - 10, th * 16 - 40, th * 16 - 30),
        );
        assert_eq!(
            rotate_face_uv(uv, 270, tw, th),
            (30, 40, tw * 16 - 20, tw * 16 - 10),
        );
    }

    #[test]
    fn rotate_face_uv_leaves_invalid_rotations_unchanged() {
        let uv = (10, 20, 30, 40);
        // Rotation 0 and unsupported angles fail closed: UV unchanged.
        assert_eq!(rotate_face_uv(uv, 0, 2, 3), uv);
        assert_eq!(rotate_face_uv(uv, 45, 2, 3), uv);
        assert_eq!(rotate_face_uv(uv, -90, 2, 3), uv);
    }

    #[test]
    fn element_vertex_coord_picks_low_or_high_bound() {
        // Low-face vertices (base < 0.5) use `from`.
        assert_eq!(element_vertex_coord(0.0, 0.0, 16.0), 0.0);
        // High-face vertices use `to`, scaled from model to block space.
        assert_eq!(element_vertex_coord(1.0, 0.0, 16.0), 1.0);
        assert_eq!(element_vertex_coord(1.0, 0.0, 8.0), 0.5);
    }

    #[test]
    fn select_uv_offset_chooses_low_or_high() {
        assert_eq!(select_uv_offset(0, 1, 2), 1);
        assert_eq!(select_uv_offset(1, 1, 2), 2);
    }

    #[test]
    fn rotate_direction_advances_face_cycle() {
        assert_eq!(
            rotate_direction(Direction::North, 1, FACE_ROTATION, &[Direction::Invalid]),
            Direction::East,
        );
        assert_eq!(
            rotate_direction(Direction::West, 1, FACE_ROTATION, &[Direction::Invalid]),
            Direction::North,
        );
    }

    #[test]
    fn rotate_direction_passes_invalid_through_unchanged() {
        assert_eq!(
            rotate_direction(Direction::Invalid, 1, FACE_ROTATION, &[Direction::Invalid]),
            Direction::Invalid,
        );
    }
}

thread_local!(
    static MULTIPART_CACHE: RefCell<HashMap<(Key, Block), Model, BuildHasherDefault<FNVHash>>> = RefCell::new(HashMap::with_hasher(BuildHasherDefault::default()))
);

impl Factory {
    pub fn new(
        resources: resources::SharedManager,
        textures: Arc<RwLock<render::TextureManager>>,
    ) -> Factory {
        Factory {
            grass_colors: Factory::load_biome_colors(resources.clone(), "grass"),
            foliage_colors: Factory::load_biome_colors(resources.clone(), "foliage"),
            resources,
            textures,

            models: HashMap::with_hasher(BuildHasherDefault::default()),
        }
    }

    fn load_biome_colors(res: resources::SharedManager, name: &str) -> image::DynamicImage {
        let mut val = match res
            .read()
            .unwrap()
            .open("minecraft", &format!("textures/colormap/{}.png", name))
        {
            Some(val) => val,
            None => return image::DynamicImage::new_rgb8(256, 256),
        };
        let mut data = Vec::new();
        val.read_to_end(&mut data).unwrap();
        image::load_from_memory(&data).unwrap()
    }

    pub fn version_change(&mut self) {
        self.models.clear();
        self.grass_colors = Factory::load_biome_colors(self.resources.clone(), "grass");
        self.foliage_colors = Factory::load_biome_colors(self.resources.clone(), "foliage");
    }

    fn get_model<R: Rng, W: Write>(
        &self,
        key: Key,
        block: Block,
        rng: &mut R,
        snapshot: &world::Snapshot,
        x: i32,
        y: i32,
        z: i32,
        buf: &mut W,
    ) -> Result<usize, bool> {
        use std::collections::hash_map::Entry;
        if let Some(model) = self.models.get(&key) {
            if !uses_multipart(model) {
                let variant = block.get_model_variant();
                if let Some(var) = model.get_variants(&variant) {
                    let model = var.choose_model(rng);
                    return Ok(model.render(self, snapshot, x, y, z, buf));
                }
            } else {
                return MULTIPART_CACHE.with(|cache| {
                    let mut cache = cache.borrow_mut();
                    let entry = cache.entry((key.clone(), block));
                    match entry {
                        Entry::Occupied(e) => {
                            return Ok(e.get().render(self, snapshot, x, y, z, buf));
                        }
                        Entry::Vacant(e) => {
                            let mut res: Option<Model> = None;
                            for rule in &model.multipart {
                                let ok = eval_multipart_rules(
                                    &rule.rules,
                                    &|rule_key, rule_val| block.match_multipart(rule_key, rule_val),
                                );
                                if ok {
                                    if res.is_some() {
                                        res.as_mut().unwrap().join(rule.apply.choose_model(rng));
                                    } else {
                                        res = Some(rule.apply.choose_model(rng).clone());
                                    }
                                }
                            }
                            if let Some(mdl) = res {
                                return Ok(e.insert(mdl).render(self, snapshot, x, y, z, buf));
                            }
                        }
                    };
                    Err(true)
                });
            }
            return Err(true);
        }
        Err(false)
    }

    pub fn get_state_model<R: Rng, W: Write>(
        models: &Arc<RwLock<Factory>>,
        block: Block,
        rng: &mut R,
        snapshot: &world::Snapshot,
        x: i32,
        y: i32,
        z: i32,
        buf: &mut W,
    ) -> usize {
        let (plugin, name) = block.get_model();
        let key = Key(plugin.to_owned(), name.to_owned());
        let mut missing_variant;
        {
            let m = models.read().unwrap();
            match m.get_model(key.clone(), block, rng, snapshot, x, y, z, buf) {
                Ok(val) => return val,
                Err(val) => missing_variant = val,
            };
        }
        if !missing_variant {
            // Whole model not loaded, try and load
            let mut m = models.write().unwrap();
            if !m.models.contains_key(&key) && !m.load_model(&plugin, &name) {
                error!("Error loading model {}:{}", plugin, name);
            }
            match m.get_model(key.clone(), block, rng, snapshot, x, y, z, buf) {
                Ok(val) => return val,
                Err(val) => missing_variant = val,
            };
        }
        let ret = Factory::get_state_model(models, Block::Missing {}, rng, snapshot, x, y, z, buf);
        if !missing_variant {
            // Still no model, replace with placeholder
            let mut m = models.write().unwrap();
            let model = m
                .models
                .get(&Key("steven".to_owned(), "missing_block".to_owned()))
                .unwrap()
                .clone();
            m.models.insert(key, model);
        }
        ret
    }

    fn load_model(&mut self, plugin: &str, name: &str) -> bool {
        let file = match self
            .resources
            .read()
            .unwrap()
            .open(plugin, &format!("blockstates/{}.json", name))
        {
            Some(val) => val,
            None => {
                error!("Error missing block state for {}:{}", plugin, name);
                return false;
            }
        };
        let mdl: serde_json::Value = try_log!(serde_json::from_reader(file));

        let mut model = StateModel {
            variants: HashMap::with_hasher(BuildHasherDefault::default()),
            multipart: vec![],
        };

        if let Some(variants) = mdl.get("variants").and_then(|v| v.as_object()) {
            for (k, v) in variants {
                let vars = self.parse_model_list(plugin, v);
                if vars.models.is_empty() {
                    return false;
                }
                model.variants.insert(k.clone(), vars);
            }
        }
        if let Some(multipart) = mdl.get("multipart").and_then(|v| v.as_array()) {
            for rule in multipart {
                let apply = self.parse_model_list(plugin, rule.get("apply").unwrap());
                let mut rules = vec![];
                if let Some(when) = rule.get("when").and_then(|v| v.as_object()) {
                    Self::parse_rules(when, &mut rules);
                }
                model.multipart.push(MultipartRule { apply, rules })
            }
        }

        self.models
            .insert(Key(plugin.to_owned(), name.to_owned()), model);
        true
    }

    fn parse_rules(when: &serde_json::Map<String, serde_json::Value>, rules: &mut Vec<Rule>) {
        for (name, val) in when {
            if name == "OR" {
                let mut or_rules = vec![];
                for sub in val.as_array().unwrap() {
                    let mut sub_rules = vec![];
                    Self::parse_rules(sub.as_object().unwrap(), &mut sub_rules);
                    or_rules.push(sub_rules);
                }
                rules.push(Rule::Or(or_rules));
            } else {
                let v = match *val {
                    serde_json::Value::Bool(ref v) => v.to_string(),
                    serde_json::Value::Number(ref v) => v.to_string(),
                    serde_json::Value::String(ref v) => v.to_owned(),
                    _ => unreachable!(),
                };
                rules.push(Rule::Match(name.to_owned(), v));
            }
        }
    }

    fn parse_model_list(&self, plugin: &str, v: &serde_json::Value) -> Variants {
        let mut variants = Variants { models: vec![] };
        if let Some(list) = v.as_array() {
            for val in list {
                if let Some(mdl) = self.parse_block_state_variant(plugin, val) {
                    variants.models.push(self.process_model(mdl));
                }
            }
        } else if let Some(mdl) = self.parse_block_state_variant(plugin, v) {
            variants.models.push(self.process_model(mdl));
        }
        variants
    }

    fn parse_block_state_variant(&self, plugin: &str, v: &serde_json::Value) -> Option<RawModel> {
        let model_name = match v.get("model").and_then(|v| v.as_str()) {
            Some(val) => val,
            None => {
                error!("Couldn't find model name");
                return None;
            }
        };

        let model_resource = match block_state_model_resource(plugin, model_name) {
            Some(val) => val,
            None => {
                error!("Invalid model reference {}", model_name);
                return None;
            }
        };
        let file = match self
            .resources
            .read()
            .unwrap()
            .open(&model_resource.plugin, &model_resource.location)
        {
            Some(val) => val,
            None => {
                error!(
                    "Couldn't find model {}:{}",
                    model_resource.plugin, model_resource.location
                );
                return None;
            }
        };
        let block_model: serde_json::Value = try_log!(opt serde_json::from_reader(file));

        let mut model = match self.parse_model(&model_resource.plugin, &block_model) {
            Some(val) => val,
            None => {
                error!(
                    "Failed to parse model {}:{}",
                    model_resource.plugin, model_resource.location
                );
                return None;
            }
        };

        model.y = v.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
        model.x = v.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
        model.uvlock = v.get("uvlock").and_then(|v| v.as_bool()).unwrap_or(false);
        model.weight = v.get("weight").and_then(|v| v.as_f64()).unwrap_or(1.0);
        Some(model)
    }

    fn parse_model(&self, plugin: &str, v: &serde_json::Value) -> Option<RawModel> {
        let parent = v.get("parent").and_then(|v| v.as_str()).unwrap_or("");
        let parent_reference = parse_resource_reference(plugin, parent);
        let parent_location = parent_reference
            .as_ref()
            .map(|resource| resource.location.as_str())
            .unwrap_or("");
        let mut model = if should_load_parent_model(parent, parent_location) {
            let parent_resource = match parent_model_resource(plugin, parent) {
                Some(val) => val,
                None => {
                    error!("Invalid parent model reference {}", parent);
                    return None;
                }
            };
            let file = match self
                .resources
                .read()
                .unwrap()
                .open(&parent_resource.plugin, &parent_resource.location)
            {
                Some(val) => val,
                None => {
                    error!(
                        "Couldn't find model {}:{}",
                        parent_resource.plugin, parent_resource.location
                    );
                    return None;
                }
            };
            let block_model: serde_json::Value = try_log!(opt serde_json::from_reader(file));
            match self.parse_model(&parent_resource.plugin, &block_model) {
                Some(val) => val,
                None => {
                    error!(
                        "Failed to parse model {}:{}",
                        parent_resource.plugin, parent_resource.location
                    );
                    return None;
                }
            }
        } else {
            RawModel {
                texture_vars: HashMap::with_hasher(BuildHasherDefault::default()),
                elements: vec![],
                ambient_occlusion: true,
                ao_set: false,

                x: 0.0,
                y: 0.0,
                uvlock: false,
                weight: 1.0,

                display: HashMap::with_hasher(BuildHasherDefault::default()),
                builtin: resolve_builtin_type(parent_location),
            }
        };

        if let Some(textures) = v.get("textures").and_then(|v| v.as_object()) {
            for (k, v) in textures {
                model
                    .texture_vars
                    .insert(k.clone(), v.as_str().unwrap_or("").to_owned());
            }
        }

        if let Some(ao) = v.get("ambientocclusion").and_then(|v| v.as_bool()) {
            model.ambient_occlusion = ao;
            model.ao_set = true;
        } else if !model.ao_set {
            model.ambient_occlusion = true;
        }

        if let Some(elements) = v.get("elements").and_then(|v| v.as_array()) {
            for e in elements {
                model.elements.push(self.parse_block_element(e));
            }
        }

        // TODO: Display

        Some(model)
    }

    fn parse_block_element(&self, v: &serde_json::Value) -> ModelElement {
        let mut element = ModelElement {
            from: v
                .get("from")
                .and_then(|v| v.as_array())
                .map(|v| {
                    [
                        v[0].as_f64().unwrap(),
                        v[1].as_f64().unwrap(),
                        v[2].as_f64().unwrap(),
                    ]
                })
                .unwrap(),
            to: v
                .get("to")
                .and_then(|v| v.as_array())
                .map(|v| {
                    [
                        v[0].as_f64().unwrap(),
                        v[1].as_f64().unwrap(),
                        v[2].as_f64().unwrap(),
                    ]
                })
                .unwrap(),
            shade: v.get("shade").and_then(|v| v.as_bool()).unwrap_or(false),
            faces: [None, None, None, None, None, None],
            rotation: None,
        };
        if let Some(faces) = v.get("faces").and_then(|v| v.as_object()) {
            for dir in Direction::all() {
                if let Some(face) = faces.get(dir.as_string()) {
                    element.faces[dir.index()] = Some(BlockFace {
                        uv: face.get("uv").and_then(|v| v.as_array()).map_or_else(
                            || {
                                let mut uv = [0.0, 0.0, 16.0, 16.0];
                                match dir {
                                    Direction::North | Direction::South => {
                                        uv[0] = element.from[0];
                                        uv[2] = element.to[0];
                                        uv[1] = 16.0 - element.to[1];
                                        uv[3] = 16.0 - element.from[1];
                                    }
                                    Direction::West | Direction::East => {
                                        uv[0] = element.from[2];
                                        uv[2] = element.to[2];
                                        uv[1] = 16.0 - element.to[1];
                                        uv[3] = 16.0 - element.from[1];
                                    }
                                    Direction::Down | Direction::Up => {
                                        uv[0] = element.from[0];
                                        uv[2] = element.to[0];
                                        uv[1] = 16.0 - element.to[2];
                                        uv[3] = 16.0 - element.from[2];
                                    }
                                    _ => unreachable!(),
                                }
                                uv
                            },
                            |v| {
                                [
                                    v[0].as_f64().unwrap(),
                                    v[1].as_f64().unwrap(),
                                    v[2].as_f64().unwrap(),
                                    v[3].as_f64().unwrap(),
                                ]
                            },
                        ),
                        texture: face
                            .get("texture")
                            .and_then(|v| v.as_str())
                            .map(|v| {
                                if v.starts_with('#') {
                                    v.to_owned()
                                } else {
                                    "#".to_owned() + v
                                }
                            })
                            .unwrap(),
                        cull_face: Direction::from_string(
                            face.get("cullface")
                                .and_then(|v| v.as_str())
                                .unwrap_or("invalid"),
                        ),
                        rotation: face
                            .get("rotation")
                            .and_then(|v| v.as_i64())
                            .map_or(0, |v| v as i32),
                        tint_index: face
                            .get("tintindex")
                            .and_then(|v| v.as_i64())
                            .map_or(-1, |v| v as i32),
                    });
                }
            }
        }

        if let Some(rotation) = v.get("rotation") {
            element.rotation = Some(BlockRotation {
                origin: rotation.get("origin").and_then(|v| v.as_array()).map_or(
                    [8.0, 8.0, 8.0],
                    |v| {
                        [
                            v[0].as_f64().unwrap(),
                            v[1].as_f64().unwrap(),
                            v[2].as_f64().unwrap(),
                        ]
                    },
                ),
                axis: rotation
                    .get("axis")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_owned(),
                angle: rotation
                    .get("angle")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0),
                rescale: rotation
                    .get("rescale")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            });
        }

        element
    }

    fn process_model(&self, mut raw: RawModel) -> Model {
        let mut model = Model {
            faces: vec![],
            ambient_occlusion: raw.ambient_occlusion,
            weight: raw.weight,
        };
        let elements = std::mem::take(&mut raw.elements);
        for el in elements {
            let all_dirs = Direction::all();
            for (i, face) in el.faces.iter().enumerate() {
                if let Some(face) = face.as_ref() {
                    let face_id = all_dirs[i];
                    let mut processed_face = Face {
                        cull_face: face.cull_face,
                        facing: face_id,
                        vertices: vec![],
                        vertices_texture: vec![],
                        indices: 0,
                        shade: el.shade,
                        tint_index: face.tint_index,
                    };
                    if raw.x > 0.0 {
                        let o = (raw.x as i32) / 90;
                        processed_face.cull_face = rotate_direction(
                            processed_face.cull_face,
                            o,
                            FACE_ROTATION_X,
                            &[Direction::East, Direction::West, Direction::Invalid],
                        );
                        processed_face.facing = rotate_direction(
                            processed_face.facing,
                            o,
                            FACE_ROTATION_X,
                            &[Direction::East, Direction::West, Direction::Invalid],
                        );
                    }
                    if raw.y > 0.0 {
                        let o = (raw.y as i32) / 90;
                        processed_face.cull_face = rotate_direction(
                            processed_face.cull_face,
                            o,
                            FACE_ROTATION,
                            &[Direction::Up, Direction::Down, Direction::Invalid],
                        );
                        processed_face.facing = rotate_direction(
                            processed_face.facing,
                            o,
                            FACE_ROTATION,
                            &[Direction::Up, Direction::Down, Direction::Invalid],
                        );
                    }

                    let mut verts = BlockVertex::face_by_direction(all_dirs[i]).to_vec();
                    let texture_name = raw.lookup_texture(&face.texture);
                    let texture = render::Renderer::get_texture(&self.textures, &texture_name);

                    let mut ux1 = (face.uv[0] * (texture.get_width() as f64)) as i16;
                    let mut ux2 = (face.uv[2] * (texture.get_width() as f64)) as i16;
                    let mut uy1 = (face.uv[1] * (texture.get_height() as f64)) as i16;
                    let mut uy2 = (face.uv[3] * (texture.get_height() as f64)) as i16;

                    let tw = texture.get_width() as i16;
                    let th = texture.get_height() as i16;
                    let (rotated_ux1, rotated_ux2, rotated_uy1, rotated_uy2) =
                        rotate_face_uv((ux1, ux2, uy1, uy2), face.rotation, tw, th);
                    ux1 = rotated_ux1;
                    ux2 = rotated_ux2;
                    uy1 = rotated_uy1;
                    uy2 = rotated_uy2;

                    for v in &mut verts {
                        processed_face.vertices_texture.push(texture.clone());
                        v.tx = texture.get_x() as u16;
                        v.ty = texture.get_y() as u16;
                        v.tw = texture.get_width() as u16;
                        v.th = texture.get_height() as u16;
                        v.tatlas = texture.atlas as i16;

                        v.x = element_vertex_coord(v.x, el.from[0], el.to[0]);
                        v.y = element_vertex_coord(v.y, el.from[1], el.to[1]);
                        v.z = element_vertex_coord(v.z, el.from[2], el.to[2]);

                        if let Some(r) = el.rotation.as_ref() {
                            let angle = r.angle * (::std::f64::consts::PI / 180.0);
                            let angle = if r.axis == "z" { angle } else { -angle } as f32;
                            let ci = 1.0 / angle.cos();
                            v.x -= (r.origin[0] / 16.0) as f32;
                            v.y -= (r.origin[1] / 16.0) as f32;
                            v.z -= (r.origin[2] / 16.0) as f32;
                            match &*r.axis {
                                "y" => {
                                    let c = angle.cos();
                                    let s = angle.sin();
                                    let x = v.x;
                                    let z = v.z;
                                    v.x = x * c - z * s;
                                    v.z = z * c + x * s;

                                    if r.rescale {
                                        v.x *= ci;
                                        v.z *= ci;
                                    }
                                }
                                "x" => {
                                    let c = angle.cos();
                                    let s = angle.sin();
                                    let z = v.z;
                                    let y = v.y;
                                    v.z = z * c - y * s;
                                    v.y = y * c + z * s;

                                    if r.rescale {
                                        v.z *= ci;
                                        v.y *= ci;
                                    }
                                }
                                "z" => {
                                    let c = angle.cos();
                                    let s = angle.sin();
                                    let x = v.x;
                                    let y = v.y;
                                    v.x = x * c - y * s;
                                    v.y = y * c + x * s;

                                    if r.rescale {
                                        v.x *= ci;
                                        v.y *= ci;
                                    }
                                }
                                _ => {}
                            }
                            v.x += (r.origin[0] / 16.0) as f32;
                            v.y += (r.origin[1] / 16.0) as f32;
                            v.z += (r.origin[2] / 16.0) as f32;
                        }

                        if raw.x > 0.0 {
                            let rot_x = (raw.x * (::std::f64::consts::PI / 180.0)) as f32;
                            let c = rot_x.cos();
                            let s = rot_x.sin();
                            let z = v.z - 0.5;
                            let y = v.y - 0.5;
                            v.z = 0.5 + (z * c - y * s);
                            v.y = 0.5 + (y * c + z * s);
                        }

                        if raw.y > 0.0 {
                            let rot_y = (raw.y * (::std::f64::consts::PI / 180.0)) as f32;
                            let c = rot_y.cos();
                            let s = rot_y.sin();
                            let x = v.x - 0.5;
                            let z = v.z - 0.5;
                            v.x = 0.5 + (x * c - z * s);
                            v.z = 0.5 + (z * c + x * s);
                        }

                        v.toffsetx = select_uv_offset(v.toffsetx, ux1, ux2);
                        v.toffsety = select_uv_offset(v.toffsety, uy1, uy2);

                        if face.rotation > 0 {
                            let rot_y =
                                (-face.rotation as f64 * (::std::f64::consts::PI / 180.0)) as f32;
                            let c = rot_y.cos() as i16;
                            let s = rot_y.sin() as i16;
                            let x = v.toffsetx - 8 * tw;
                            let y = v.toffsety - 8 * th;
                            v.toffsetx = 8 * tw + (x * c - y * s);
                            v.toffsety = 8 * th + (y * c + x * s);
                        }

                        if raw.uvlock
                            && raw.y > 0.0
                            && (processed_face.facing == Direction::Up
                                || processed_face.facing == Direction::Down)
                        {
                            let rot_y = (raw.y * (::std::f64::consts::PI / 180.0)) as f32;
                            let c = rot_y.cos() as i16;
                            let s = rot_y.sin() as i16;
                            let x = v.toffsetx - 8 * tw;
                            let y = v.toffsety - 8 * th;
                            v.toffsetx = 8 * tw + (x * c - y * s);
                            v.toffsety = 8 * th + (y * c + x * s);
                        }

                        if raw.uvlock
                            && raw.x > 0.0
                            && (processed_face.facing != Direction::Up
                                && processed_face.facing != Direction::Down)
                        {
                            let rot_x = (raw.x * (::std::f64::consts::PI / 180.0)) as f32;
                            let c = rot_x.cos() as i16;
                            let s = rot_x.sin() as i16;
                            let x = v.toffsetx - 8 * tw;
                            let y = v.toffsety - 8 * th;
                            v.toffsetx = 8 * tw + (x * c - y * s);
                            v.toffsety = 8 * th + (y * c + x * s);
                        }
                    }

                    processed_face.vertices = verts;
                    processed_face.indices = 6;
                    model.faces.push(processed_face);
                }
            }
        }
        model
    }
}

const FACE_ROTATION: &[Direction] = &[
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

const FACE_ROTATION_X: &[Direction] = &[
    Direction::North,
    Direction::Down,
    Direction::South,
    Direction::Up,
];

fn rotate_direction(
    val: Direction,
    offset: i32,
    rots: &[Direction],
    invalid: &[Direction],
) -> Direction {
    for d in invalid {
        if *d == val {
            return val;
        }
    }
    let pos = rots.iter().position(|v| *v == val).unwrap_or(0) as i32;
    rots[(rots.len() as i32 + pos + offset) as usize % rots.len()]
}

#[derive(Clone)]
pub struct StateModel {
    variants: HashMap<String, Variants, BuildHasherDefault<FNVHash>>,
    multipart: Vec<MultipartRule>,
}

impl StateModel {
    pub fn get_variants(&self, name: &str) -> Option<&Variants> {
        self.variants.get(name)
    }
}

#[derive(Clone)]
struct MultipartRule {
    apply: Variants,
    rules: Vec<Rule>,
}

#[derive(Clone)]
enum Rule {
    Match(String, String),
    Or(Vec<Vec<Rule>>),
}

#[derive(Clone)]
pub struct Variants {
    models: Vec<Model>,
}

impl Variants {
    fn choose_model<R: Rng>(&self, rng: &mut R) -> &Model {
        // TODO: Weighted random
        self.models.choose(rng).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum BuiltinType {
    False,
    Generated,
    Entity,
    Compass,
    Clock,
}

#[derive(Debug)]
struct RawModel {
    texture_vars: HashMap<String, String, BuildHasherDefault<FNVHash>>,
    elements: Vec<ModelElement>,
    ambient_occlusion: bool,
    ao_set: bool,

    x: f64,
    y: f64,
    uvlock: bool,
    weight: f64,

    #[allow(dead_code)]
    display: HashMap<String, ModelDisplay, BuildHasherDefault<FNVHash>>,
    #[allow(dead_code)]
    builtin: BuiltinType,
}

impl RawModel {
    fn lookup_texture(&self, name: &str) -> String {
        if !name.is_empty() && name.starts_with('#') {
            let tex = self
                .texture_vars
                .get(&name[1..])
                .cloned()
                .unwrap_or_else(|| "".to_owned());
            return self.lookup_texture(&tex);
        }
        name.to_owned()
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct ModelDisplay {
    rotation: [f64; 3],
    translation: [f64; 3],
    scale: [f64; 3],
}

#[derive(Debug)]
struct ModelElement {
    from: [f64; 3],
    to: [f64; 3],
    shade: bool,
    rotation: Option<BlockRotation>,
    faces: [Option<BlockFace>; 6],
}

#[derive(Debug)]
struct BlockRotation {
    origin: [f64; 3],
    axis: String,
    angle: f64,
    rescale: bool,
}

#[derive(Debug)]
struct BlockFace {
    uv: [f64; 4],
    texture: String,
    cull_face: Direction,
    rotation: i32,
    tint_index: i32,
}

#[derive(Clone, Debug)]
struct Model {
    faces: Vec<Face>,
    ambient_occlusion: bool,
    #[allow(dead_code)]
    weight: f64,
}

#[derive(Clone, Debug)]
struct Face {
    cull_face: Direction,
    facing: Direction,
    vertices: Vec<BlockVertex>,
    vertices_texture: Vec<render::Texture>,
    indices: usize,
    #[allow(dead_code)]
    shade: bool,
    tint_index: i32,
}

impl Model {
    fn join(&mut self, other: &Model) {
        self.faces.extend_from_slice(&other.faces);
    }

    fn render<W: Write>(
        &self,
        factory: &Factory,
        snapshot: &world::Snapshot,
        x: i32,
        y: i32,
        z: i32,
        buf: &mut W,
    ) -> usize {
        let this = snapshot.get_block(x, y, z);
        let this_mat = this.get_material();
        let mut indices = 0;

        let tint = this.get_tint();

        for face in &self.faces {
            if face.cull_face != Direction::Invalid && !this_mat.never_cull {
                let (ox, oy, oz) = face.cull_face.get_offset();
                let other = snapshot.get_block(x + ox, y + oy, z + oz);
                if other.get_material().should_cull_against || other == this {
                    continue;
                }
            }
            indices += face.indices;

            for vert in &face.vertices {
                let mut vert = vert.clone();

                vert.x += x as f32;
                vert.y += y as f32;
                vert.z += z as f32;

                let (mut cr, mut cg, mut cb) = if face.tint_index == 0 {
                    match tint {
                        TintType::Default => (255, 255, 255),
                        TintType::Color { r, g, b } => (r, g, b),
                        TintType::Grass => calculate_biome(
                            snapshot,
                            vert.x as i32,
                            vert.z as i32,
                            &factory.grass_colors,
                        ),
                        TintType::Foliage => calculate_biome(
                            snapshot,
                            vert.x as i32,
                            vert.z as i32,
                            &factory.foliage_colors,
                        ),
                    }
                } else {
                    (255, 255, 255)
                };
                if face.facing == Direction::West || face.facing == Direction::East {
                    cr = ((cr as f64) * 0.8) as u8;
                    cg = ((cg as f64) * 0.8) as u8;
                    cb = ((cb as f64) * 0.8) as u8;
                }

                vert.r = cr;
                vert.g = cg;
                vert.b = cb;

                let (bl, sl) = calculate_light(
                    snapshot,
                    x,
                    y,
                    z,
                    vert.x as f64,
                    vert.y as f64,
                    vert.z as f64,
                    face.facing,
                    self.ambient_occlusion,
                    this_mat.force_shade,
                );
                vert.block_light = bl;
                vert.sky_light = sl;
                vert.write(buf);
            }
        }
        indices
    }
}

fn calculate_biome(
    snapshot: &world::Snapshot,
    x: i32,
    z: i32,
    img: &image::DynamicImage,
) -> (u8, u8, u8) {
    use std::cmp::{max, min};
    // Fixed 3x3 neighborhood sampled by Minecraft biome color blending.
    const SMOOTH_BIOME_SAMPLE_COUNT: usize = 9;
    let mut samples: [(u8, u8, u8); SMOOTH_BIOME_SAMPLE_COUNT] =
        [(0, 0, 0); SMOOTH_BIOME_SAMPLE_COUNT];
    let mut index = 0usize;
    for xx in -1..2 {
        for zz in -1..2 {
            let bi = snapshot.get_biome(x + xx, z + zz);
            let color_index = bi.get_color_index();
            let ix = color_index & 0xFF;
            let iy = color_index >> 8;

            let ix = min(max(ix, 0), 255);
            let iy = min(max(iy, 0), 255);

            let col = img.get_pixel(ix as u32, iy as u32);
            let col = bi.process_color(col);
            samples[index] = (col.0[0], col.0[1], col.0[2]);
            index += 1;
        }
    }
    debug_assert_eq!(index, SMOOTH_BIOME_SAMPLE_COUNT);
    average_rgb(&samples[..index])
}

fn calculate_light(
    snapshot: &world::Snapshot,
    orig_x: i32,
    orig_y: i32,
    orig_z: i32,
    x: f64,
    y: f64,
    z: f64,
    face: Direction,
    smooth: bool,
    force: bool,
) -> (u16, u16) {
    use crate::world::block;
    use std::cmp::max;
    let (ox, oy, oz) = face.get_offset();

    let s_block_light = snapshot.get_block_light(orig_x + ox, orig_y + oy, orig_z + oz);
    let s_sky_light = snapshot.get_sky_light(orig_x + ox, orig_y + oy, orig_z + oz);
    if !smooth {
        return (
            pack_light_value(s_block_light),
            pack_light_value(s_sky_light),
        );
    }

    let mut block_light = 0u32;
    let mut sky_light = 0u32;
    let mut count = 0;

    let s_block_light = max((s_block_light as i8) - 8, 0) as u8;
    let s_sky_light = max((s_sky_light as i8) - 8, 0) as u8;

    let dx = (ox as f64) * 0.6;
    let dy = (oy as f64) * 0.6;
    let dz = (oz as f64) * 0.6;

    for ox in [-0.6, 0.0].iter() {
        for oy in [-0.6, 0.0].iter() {
            for oz in [-0.6, 0.0].iter() {
                let lx = (x + ox + dx).round() as i32;
                let ly = (y + oy + dy).round() as i32;
                let lz = (z + oz + dz).round() as i32;
                let mut bl = snapshot.get_block_light(lx, ly, lz);
                let mut sl = snapshot.get_sky_light(lx, ly, lz);
                if (force && !matches!(snapshot.get_block(lx, ly, lz), block::Air {}))
                    || (sl == 0 && bl == 0)
                {
                    bl = s_block_light;
                    sl = s_sky_light;
                }
                block_light += bl as u32;
                sky_light += sl as u32;
                count += 1;
            }
        }
    }

    average_packed_light(block_light, sky_light, count)
}

pub const PRECOMPUTED_VERTS: [&[BlockVertex; 4]; 6] = [
    &[
        // Up
        BlockVertex::base(0.0, 1.0, 0.0, 0, 0),
        BlockVertex::base(1.0, 1.0, 0.0, 1, 0),
        BlockVertex::base(0.0, 1.0, 1.0, 0, 1),
        BlockVertex::base(1.0, 1.0, 1.0, 1, 1),
    ],
    &[
        // Down
        BlockVertex::base(0.0, 0.0, 0.0, 0, 1),
        BlockVertex::base(0.0, 0.0, 1.0, 0, 0),
        BlockVertex::base(1.0, 0.0, 0.0, 1, 1),
        BlockVertex::base(1.0, 0.0, 1.0, 1, 0),
    ],
    &[
        // North
        BlockVertex::base(0.0, 0.0, 0.0, 1, 1),
        BlockVertex::base(1.0, 0.0, 0.0, 0, 1),
        BlockVertex::base(0.0, 1.0, 0.0, 1, 0),
        BlockVertex::base(1.0, 1.0, 0.0, 0, 0),
    ],
    &[
        // South
        BlockVertex::base(0.0, 0.0, 1.0, 0, 1),
        BlockVertex::base(0.0, 1.0, 1.0, 0, 0),
        BlockVertex::base(1.0, 0.0, 1.0, 1, 1),
        BlockVertex::base(1.0, 1.0, 1.0, 1, 0),
    ],
    &[
        // West
        BlockVertex::base(0.0, 0.0, 0.0, 0, 1),
        BlockVertex::base(0.0, 1.0, 0.0, 0, 0),
        BlockVertex::base(0.0, 0.0, 1.0, 1, 1),
        BlockVertex::base(0.0, 1.0, 1.0, 1, 0),
    ],
    &[
        // East
        BlockVertex::base(1.0, 0.0, 0.0, 1, 1),
        BlockVertex::base(1.0, 0.0, 1.0, 0, 1),
        BlockVertex::base(1.0, 1.0, 0.0, 1, 0),
        BlockVertex::base(1.0, 1.0, 1.0, 0, 0),
    ],
];

#[derive(Clone, Debug)]
pub struct BlockVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tx: u16,
    pub ty: u16,
    pub tw: u16,
    pub th: u16,
    pub toffsetx: i16,
    pub toffsety: i16,
    pub tatlas: i16,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub block_light: u16,
    pub sky_light: u16,
}

impl BlockVertex {
    const fn base(x: f32, y: f32, z: f32, tx: i16, ty: i16) -> BlockVertex {
        BlockVertex {
            x,
            y,
            z,
            tx: 0,
            ty: 0,
            tw: 0,
            th: 0,
            toffsetx: tx,
            toffsety: ty,
            tatlas: 0,
            r: 0,
            g: 0,
            b: 0,
            block_light: 0,
            sky_light: 0,
        }
    }
    pub fn write<W: Write>(&self, w: &mut W) {
        let _ = w.write_f32::<NativeEndian>(self.x);
        let _ = w.write_f32::<NativeEndian>(self.y);
        let _ = w.write_f32::<NativeEndian>(self.z);
        let _ = w.write_u16::<NativeEndian>(self.tx);
        let _ = w.write_u16::<NativeEndian>(self.ty);
        let _ = w.write_u16::<NativeEndian>(self.tw);
        let _ = w.write_u16::<NativeEndian>(self.th);
        let _ = w.write_i16::<NativeEndian>(self.toffsetx);
        let _ = w.write_i16::<NativeEndian>(self.toffsety);
        let _ = w.write_i16::<NativeEndian>(self.tatlas);
        let _ = w.write_i16::<NativeEndian>(0);
        let _ = w.write_u8(self.r);
        let _ = w.write_u8(self.g);
        let _ = w.write_u8(self.b);
        let _ = w.write_u8(255);
        let _ = w.write_u16::<NativeEndian>(self.block_light);
        let _ = w.write_u16::<NativeEndian>(self.sky_light);
        let _ = w.write_u16::<NativeEndian>(0);
        let _ = w.write_u16::<NativeEndian>(0);
    }

    pub fn face_by_direction(dir: Direction) -> &'static [BlockVertex; 4] {
        match dir {
            Direction::Up => PRECOMPUTED_VERTS[0],
            Direction::Down => PRECOMPUTED_VERTS[1],
            Direction::North => PRECOMPUTED_VERTS[2],
            Direction::South => PRECOMPUTED_VERTS[3],
            Direction::West => PRECOMPUTED_VERTS[4],
            Direction::East => PRECOMPUTED_VERTS[5],
            _ => unreachable!(),
        }
    }
}
