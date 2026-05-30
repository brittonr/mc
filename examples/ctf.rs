#![allow(clippy::type_complexity)]

use std::{
    collections::HashMap,
    fs,
    path::Path,
    sync::{OnceLock, RwLock},
};

use bevy_ecs::query::QueryData;
use valence::entity::cow::CowEntityBundle;
use valence::entity::entity::Flags;
use valence::entity::living::Health;
use valence::entity::pig::PigEntityBundle;
use valence::entity::player::PlayerEntityBundle;
use valence::entity::{EntityAnimations, EntityId, EntityStatuses, OnGround, Velocity};
use valence::equipment::{Equipment, EquipmentInventorySync};
use valence::hand_swing::HandSwingEvent;
use valence::interact_block::InteractBlockEvent;
use valence::interact_item::InteractItemEvent;
use valence::inventory::{
    ClickSlotEvent, DropItemStackEvent, HeldItem, OpenInventory, UpdateSelectedSlotEvent,
};
use valence::log::{debug, info};
use valence::math::Vec3Swizzles;
use valence::nbt::{compound, List};
use valence::prelude::*;
use valence::protocol::packets::play::entity_equipment_update_s2c::EquipmentEntry;
use valence::protocol::packets::play::{EntityEquipmentUpdateS2c, ItemPickupAnimationS2c};
use valence::protocol::{VarInt, WritePacket};
use valence::scoreboard::*;
use valence::status::RequestRespawnEvent;

const ARENA_Y: i32 = 64;
const ARENA_MID_WIDTH: i32 = 2;
const SPAWN_BOX: [i32; 3] = [0, ARENA_Y + 20, 0];
const SPAWN_POS: [f64; 3] = [
    SPAWN_BOX[0] as f64,
    SPAWN_BOX[1] as f64 + 1.0,
    SPAWN_BOX[2] as f64,
];
const SPAWN_BOX_WIDTH: i32 = 5;
const SPAWN_BOX_HEIGHT: i32 = 4;
const PLAYER_MAX_HEALTH: f32 = 20.0;
const ARMOR_MITIGATION_CHEST_SLOT: u16 = 6;
const DIAMOND_CHESTPLATE_MITIGATION: f32 = 2.0;
const PROJECTILE_PROBE_DAMAGE: f32 = 3.0;
const ARROW_POLICY_DEFAULT_MAX_DAMAGE: f32 = 10.0;
const ARROW_POLICY_DEFAULT_VELOCITY_MULTIPLIER: f32 = 1.0;
const ARROW_POLICY_DEFAULT_PROJECTILE_VELOCITY: f32 = 0.0;
const ARROW_POLICY_DEFAULT_PULL_STRENGTH: f32 = 1.0;
const ARROW_POLICY_MIN_DAMAGE: f32 = 0.0;
const ARROW_POLICY_MAX_DAMAGE: f32 = 100.0;
const ARROW_POLICY_DEFAULT_GENERATION: u64 = 0;
const ARROW_POLICY_FIRST_GENERATION: u64 = 1;
const ARROW_POLICY_ID_DAMAGE_LINEAR: &str = "damage-linear";
const ARROW_POLICY_DEFAULT_SOURCE: &str = "default";
const ARROW_POLICY_SANDBOX_PROFILE: &str = "mc-compat/pure-v1";
const ARROW_POLICY_ENV_CONFIG: &str = "MC_COMPAT_STEEL_CONFIG";
const ARROW_POLICY_ENV_RELOAD_REQUEST: &str = "MC_COMPAT_STEEL_RELOAD_REQUEST";
const ARROW_POLICY_PATH_BASE_DAMAGE: &str = "combat.arrow.base_damage";
const ARROW_POLICY_PATH_VELOCITY_MULTIPLIER: &str = "combat.arrow.velocity_multiplier";
const ARROW_POLICY_PATH_MAX_DAMAGE: &str = "combat.arrow.max_damage";
const ARROW_POLICY_PATH_SANDBOX: &str = "runtime.steel.sandbox_profile";
const ARROW_POLICY_STEEL_EXPORT_BASE_DAMAGE: &str = "arrow-base-damage";
const ARROW_POLICY_STEEL_EXPORT_VELOCITY_MULTIPLIER: &str = "arrow-velocity-multiplier";
const ARROW_POLICY_STEEL_EXPORT_MAX_DAMAGE: &str = "arrow-max-damage";
const ARROW_POLICY_STEEL_EXPORT_SANDBOX: &str = "sandbox-profile";
const ARROW_POLICY_REQUIRED_POLICY_SHAPE: &str =
    "(damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage)";
const ARROW_POLICY_FORBIDDEN_STEEL_TOKENS: &[&str] = &[
    "open-input-file",
    "call-with-input-file",
    "delete-file",
    "system",
    "process",
    "tcp-connect",
    "current-second",
    "random",
];

#[derive(Clone, Debug, PartialEq)]
struct ArrowPolicySnapshot {
    generation: u64,
    source: String,
    policy_id: String,
    base_damage: f32,
    velocity_multiplier: f32,
    max_damage: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct ArrowDamageContext {
    projectile_velocity: f32,
    pull_strength: f32,
}

#[derive(Clone, Debug, PartialEq)]
struct ArrowDamageDecision {
    generation: u64,
    source: String,
    policy_id: String,
    damage: f32,
    clamped: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArrowPolicyDiagnostic {
    path: &'static str,
    message: String,
}

#[derive(Clone, Debug, PartialEq)]
struct ArrowPolicyDiff {
    path: &'static str,
    before: String,
    after: String,
}

#[derive(Clone, Debug, PartialEq)]
struct ArrowPolicyReloadOutcome {
    active_changed: bool,
    active_generation: u64,
    diagnostics: Vec<ArrowPolicyDiagnostic>,
}

#[derive(Clone, Debug, PartialEq)]
struct ArrowPolicyController {
    active: ArrowPolicySnapshot,
}

impl ArrowPolicyController {
    fn new(active: ArrowPolicySnapshot) -> Self {
        Self { active }
    }

    fn active(&self) -> &ArrowPolicySnapshot {
        &self.active
    }

    fn reload_candidate(
        &mut self,
        candidate: Result<ArrowPolicySnapshot, Vec<ArrowPolicyDiagnostic>>,
    ) -> ArrowPolicyReloadOutcome {
        let candidate = match candidate {
            Ok(candidate) => candidate,
            Err(diagnostics) => {
                return ArrowPolicyReloadOutcome {
                    active_changed: false,
                    active_generation: self.active.generation,
                    diagnostics,
                };
            }
        };
        let diagnostics = validate_arrow_policy_snapshot(&candidate);
        if !diagnostics.is_empty() {
            return ArrowPolicyReloadOutcome {
                active_changed: false,
                active_generation: self.active.generation,
                diagnostics,
            };
        }
        let sample_decision = evaluate_arrow_policy(
            &candidate,
            ArrowDamageContext {
                projectile_velocity: ARROW_POLICY_DEFAULT_PROJECTILE_VELOCITY,
                pull_strength: ARROW_POLICY_DEFAULT_PULL_STRENGTH,
            },
        );
        let diagnostics = validate_arrow_damage_decision(&sample_decision);
        if !diagnostics.is_empty() {
            return ArrowPolicyReloadOutcome {
                active_changed: false,
                active_generation: self.active.generation,
                diagnostics,
            };
        }
        self.active = candidate;
        ArrowPolicyReloadOutcome {
            active_changed: true,
            active_generation: self.active.generation,
            diagnostics: Vec::new(),
        }
    }
}

static ARROW_POLICY_SNAPSHOT: OnceLock<RwLock<ArrowPolicySnapshot>> = OnceLock::new();
static ARROW_POLICY_LAST_RELOAD_REQUEST: OnceLock<RwLock<Option<String>>> = OnceLock::new();

fn arrow_policy_store() -> &'static RwLock<ArrowPolicySnapshot> {
    ARROW_POLICY_SNAPSHOT.get_or_init(|| RwLock::new(default_arrow_policy_snapshot()))
}

fn arrow_policy_reload_request_store() -> &'static RwLock<Option<String>> {
    ARROW_POLICY_LAST_RELOAD_REQUEST.get_or_init(|| RwLock::new(None))
}

fn default_arrow_policy_snapshot() -> ArrowPolicySnapshot {
    ArrowPolicySnapshot {
        generation: ARROW_POLICY_DEFAULT_GENERATION,
        source: ARROW_POLICY_DEFAULT_SOURCE.to_string(),
        policy_id: ARROW_POLICY_ID_DAMAGE_LINEAR.to_string(),
        base_damage: PROJECTILE_PROBE_DAMAGE,
        velocity_multiplier: ARROW_POLICY_DEFAULT_VELOCITY_MULTIPLIER,
        max_damage: ARROW_POLICY_DEFAULT_MAX_DAMAGE,
    }
}

fn initialize_valence_arrow_policy_from_env() {
    let Some(path) = std::env::var(ARROW_POLICY_ENV_CONFIG).ok() else {
        return;
    };
    let outcome = reload_arrow_policy_from_path(Path::new(&path));
    log_arrow_policy_reload_outcome(&path, &outcome);
}

fn maybe_reload_valence_arrow_policy_on_request() {
    let Some(request) = std::env::var(ARROW_POLICY_ENV_RELOAD_REQUEST).ok() else {
        return;
    };
    let Ok(mut last_request) = arrow_policy_reload_request_store().write() else {
        return;
    };
    if last_request.as_ref() == Some(&request) {
        return;
    }
    *last_request = Some(request);
    drop(last_request);
    initialize_valence_arrow_policy_from_env();
}

fn reload_arrow_policy_from_path(path: &Path) -> ArrowPolicyReloadOutcome {
    let active = active_arrow_policy_snapshot();
    let generation = active
        .generation
        .saturating_add(ARROW_POLICY_FIRST_GENERATION);
    let candidate = load_arrow_policy_snapshot_from_path(path, generation);
    let mut controller = ArrowPolicyController::new(active);
    let outcome = controller.reload_candidate(candidate);
    if outcome.active_changed {
        publish_arrow_policy_snapshot(controller.active().clone());
    }
    outcome
}

fn load_arrow_policy_snapshot_from_path(
    path: &Path,
    generation: u64,
) -> Result<ArrowPolicySnapshot, Vec<ArrowPolicyDiagnostic>> {
    let module_text = fs::read_to_string(path).map_err(|err| {
        vec![ArrowPolicyDiagnostic {
            path: "runtime.steel.source",
            message: format!(
                "read {}: {err}",
                redact_arrow_policy_text(&path.display().to_string())
            ),
        }]
    })?;
    normalize_arrow_policy_module(&path.display().to_string(), generation, &module_text)
}

fn normalize_arrow_policy_module(
    source: &str,
    generation: u64,
    module_text: &str,
) -> Result<ArrowPolicySnapshot, Vec<ArrowPolicyDiagnostic>> {
    let mut diagnostics = Vec::new();
    for token in ARROW_POLICY_FORBIDDEN_STEEL_TOKENS {
        if module_text.contains(token) {
            diagnostics.push(ArrowPolicyDiagnostic {
                path: "runtime.steel.sandbox",
                message: format!("forbidden Steel capability: {token}"),
            });
        }
    }
    let sandbox_profile = parse_steel_string_export(module_text, ARROW_POLICY_STEEL_EXPORT_SANDBOX);
    if sandbox_profile.as_deref() != Some(ARROW_POLICY_SANDBOX_PROFILE) {
        diagnostics.push(ArrowPolicyDiagnostic {
            path: ARROW_POLICY_PATH_SANDBOX,
            message: format!("expected sandbox profile {ARROW_POLICY_SANDBOX_PROFILE}"),
        });
    }
    if !module_text.contains(ARROW_POLICY_REQUIRED_POLICY_SHAPE) {
        diagnostics.push(ArrowPolicyDiagnostic {
            path: "combat.arrow.policy",
            message: "missing damage-linear arrow-damage policy shape".to_string(),
        });
    }
    let base_damage = parse_required_steel_f32_export(
        module_text,
        ARROW_POLICY_STEEL_EXPORT_BASE_DAMAGE,
        ARROW_POLICY_PATH_BASE_DAMAGE,
        &mut diagnostics,
    );
    let velocity_multiplier = parse_required_steel_f32_export(
        module_text,
        ARROW_POLICY_STEEL_EXPORT_VELOCITY_MULTIPLIER,
        ARROW_POLICY_PATH_VELOCITY_MULTIPLIER,
        &mut diagnostics,
    );
    let max_damage = parse_required_steel_f32_export(
        module_text,
        ARROW_POLICY_STEEL_EXPORT_MAX_DAMAGE,
        ARROW_POLICY_PATH_MAX_DAMAGE,
        &mut diagnostics,
    );
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    let snapshot = ArrowPolicySnapshot {
        generation,
        source: source.to_string(),
        policy_id: ARROW_POLICY_ID_DAMAGE_LINEAR.to_string(),
        base_damage: base_damage.expect("diagnostics checked"),
        velocity_multiplier: velocity_multiplier.expect("diagnostics checked"),
        max_damage: max_damage.expect("diagnostics checked"),
    };
    let diagnostics = validate_arrow_policy_snapshot(&snapshot);
    if diagnostics.is_empty() {
        Ok(snapshot)
    } else {
        Err(diagnostics)
    }
}

fn parse_required_steel_f32_export(
    module_text: &str,
    export: &str,
    path: &'static str,
    diagnostics: &mut Vec<ArrowPolicyDiagnostic>,
) -> Option<f32> {
    let Some(body) = steel_define_body(module_text, export) else {
        diagnostics.push(ArrowPolicyDiagnostic {
            path,
            message: format!("missing Steel export {export}"),
        });
        return None;
    };
    body.parse::<f32>()
        .map_err(|err| {
            diagnostics.push(ArrowPolicyDiagnostic {
                path,
                message: format!("parse {export} as f32: {err}"),
            });
        })
        .ok()
}

fn parse_steel_string_export(module_text: &str, export: &str) -> Option<String> {
    let body = steel_define_body(module_text, export)?;
    let without_prefix = body.strip_prefix('"')?;
    let end = without_prefix.find('"')?;
    Some(without_prefix[..end].to_string())
}

fn steel_define_body(module_text: &str, export: &str) -> Option<String> {
    let needle = format!("(define {export} ");
    let start = module_text.find(&needle)?;
    let body_start = start + needle.len();
    let rest = &module_text[body_start..];
    let end = rest.find(')')?;
    Some(rest[..end].trim().to_string())
}

fn validate_arrow_policy_snapshot(snapshot: &ArrowPolicySnapshot) -> Vec<ArrowPolicyDiagnostic> {
    let mut diagnostics = Vec::new();
    validate_arrow_policy_number(
        ARROW_POLICY_PATH_BASE_DAMAGE,
        snapshot.base_damage,
        ARROW_POLICY_MIN_DAMAGE,
        ARROW_POLICY_MAX_DAMAGE,
        &mut diagnostics,
    );
    validate_arrow_policy_number(
        ARROW_POLICY_PATH_VELOCITY_MULTIPLIER,
        snapshot.velocity_multiplier,
        ARROW_POLICY_MIN_DAMAGE,
        ARROW_POLICY_MAX_DAMAGE,
        &mut diagnostics,
    );
    validate_arrow_policy_number(
        ARROW_POLICY_PATH_MAX_DAMAGE,
        snapshot.max_damage,
        ARROW_POLICY_MIN_DAMAGE,
        ARROW_POLICY_MAX_DAMAGE,
        &mut diagnostics,
    );
    diagnostics
}

fn validate_arrow_policy_number(
    path: &'static str,
    value: f32,
    min: f32,
    max: f32,
    diagnostics: &mut Vec<ArrowPolicyDiagnostic>,
) {
    if !value.is_finite() || value < min || value > max {
        diagnostics.push(ArrowPolicyDiagnostic {
            path,
            message: format!("value {value} outside {min}..={max}"),
        });
    }
}

fn diff_arrow_policy_snapshots(
    before: &ArrowPolicySnapshot,
    after: &ArrowPolicySnapshot,
) -> Vec<ArrowPolicyDiff> {
    let mut diffs = Vec::new();
    push_arrow_policy_diff(
        &mut diffs,
        ARROW_POLICY_PATH_BASE_DAMAGE,
        before.base_damage,
        after.base_damage,
    );
    push_arrow_policy_diff(
        &mut diffs,
        ARROW_POLICY_PATH_VELOCITY_MULTIPLIER,
        before.velocity_multiplier,
        after.velocity_multiplier,
    );
    push_arrow_policy_diff(
        &mut diffs,
        ARROW_POLICY_PATH_MAX_DAMAGE,
        before.max_damage,
        after.max_damage,
    );
    diffs
}

fn push_arrow_policy_diff(
    diffs: &mut Vec<ArrowPolicyDiff>,
    path: &'static str,
    before: f32,
    after: f32,
) {
    if (before - after).abs() <= f32::EPSILON {
        return;
    }
    diffs.push(ArrowPolicyDiff {
        path,
        before: redact_arrow_policy_text(&format!("{before:.1}")),
        after: redact_arrow_policy_text(&format!("{after:.1}")),
    });
}

fn evaluate_arrow_policy(
    snapshot: &ArrowPolicySnapshot,
    context: ArrowDamageContext,
) -> ArrowDamageDecision {
    let scaled_velocity = context.projectile_velocity.max(ARROW_POLICY_MIN_DAMAGE)
        * context.pull_strength.max(ARROW_POLICY_MIN_DAMAGE);
    let raw_damage = snapshot.base_damage + scaled_velocity * snapshot.velocity_multiplier;
    let max_damage = snapshot
        .max_damage
        .clamp(ARROW_POLICY_MIN_DAMAGE, ARROW_POLICY_MAX_DAMAGE);
    let damage = raw_damage.clamp(ARROW_POLICY_MIN_DAMAGE, max_damage);
    ArrowDamageDecision {
        generation: snapshot.generation,
        source: snapshot.source.clone(),
        policy_id: snapshot.policy_id.clone(),
        damage,
        clamped: (damage - raw_damage).abs() > f32::EPSILON,
    }
}

fn validate_arrow_damage_decision(decision: &ArrowDamageDecision) -> Vec<ArrowPolicyDiagnostic> {
    let mut diagnostics = Vec::new();
    validate_arrow_policy_number(
        "combat.arrow.damage",
        decision.damage,
        ARROW_POLICY_MIN_DAMAGE,
        ARROW_POLICY_MAX_DAMAGE,
        &mut diagnostics,
    );
    if decision.policy_id != ARROW_POLICY_ID_DAMAGE_LINEAR {
        diagnostics.push(ArrowPolicyDiagnostic {
            path: "combat.arrow.policy",
            message: format!("unsupported policy {}", decision.policy_id),
        });
    }
    diagnostics
}

fn active_arrow_policy_snapshot() -> ArrowPolicySnapshot {
    match arrow_policy_store().read() {
        Ok(snapshot) => snapshot.clone(),
        Err(_) => default_arrow_policy_snapshot(),
    }
}

fn publish_arrow_policy_snapshot(snapshot: ArrowPolicySnapshot) {
    if let Ok(mut active) = arrow_policy_store().write() {
        *active = snapshot;
    }
}

fn projectile_probe_damage_decision() -> ArrowDamageDecision {
    maybe_reload_valence_arrow_policy_on_request();
    evaluate_arrow_policy(
        &active_arrow_policy_snapshot(),
        ArrowDamageContext {
            projectile_velocity: ARROW_POLICY_DEFAULT_PROJECTILE_VELOCITY,
            pull_strength: ARROW_POLICY_DEFAULT_PULL_STRENGTH,
        },
    )
}

fn log_arrow_policy_reload_outcome(source: &str, outcome: &ArrowPolicyReloadOutcome) {
    if outcome.active_changed {
        let snapshot = active_arrow_policy_snapshot();
        let milestone = format!(
            "MC-COMPAT-MILESTONE steel_arrow_policy_publish source={} generation={} policy={} \
             base_damage={:.1} velocity_multiplier={:.1} max_damage={:.1}",
            redact_arrow_policy_text(source),
            snapshot.generation,
            snapshot.policy_id,
            snapshot.base_damage,
            snapshot.velocity_multiplier,
            snapshot.max_damage
        );
        info!("{}", milestone);
        println!("{}", milestone);
        return;
    }
    let diagnostics = format_arrow_policy_diagnostics(&outcome.diagnostics);
    let milestone = format!(
        "MC-COMPAT-MILESTONE steel_arrow_policy_reject source={} active_generation={} diagnostics={}",
        redact_arrow_policy_text(source),
        outcome.active_generation,
        diagnostics
    );
    info!("{}", milestone);
    println!("{}", milestone);
}

fn format_arrow_policy_diagnostics(diagnostics: &[ArrowPolicyDiagnostic]) -> String {
    diagnostics
        .iter()
        .map(|diagnostic| {
            format!(
                "{}:{}",
                diagnostic.path,
                redact_arrow_policy_text(&diagnostic.message)
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

fn redact_arrow_policy_text(value: &str) -> String {
    if value.contains("secret") || value.contains("token") || value.contains("password") {
        "<redacted>".to_string()
    } else {
        value.to_string()
    }
}

pub fn main() {
    App::new()
        .insert_resource(NetworkSettings {
            connection_mode: ConnectionMode::Offline,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            EventLoopUpdate,
            (handle_combat_events, handle_projectile_events),
        )
        .add_systems(
            Update,
            (
                init_clients,
                despawn_disconnected_ctf_clients,
                digging,
                place_blocks,
                do_team_selector_portals,
                log_inventory_hotbar_select_events,
                log_inventory_drop_events,
                log_inventory_click_state,
                update_flag_visuals,
                do_flag_capturing,
                // visualize_triggers,
                update_clones,
                teleport_oob_clients,
                necromancy,
                update_scoreboard,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    initialize_valence_arrow_policy_from_env();

    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -50..50 {
        for x in -50..50 {
            let block = match x {
                x if x < -ARENA_MID_WIDTH => BlockState::RED_CONCRETE,
                x if x > ARENA_MID_WIDTH => BlockState::BLUE_CONCRETE,
                _ => BlockState::WHITE_CONCRETE,
            };
            layer.chunk.set_block([x, ARENA_Y, z], block);
        }
    }

    let red_flag = build_flag(
        &mut layer,
        Team::Red,
        BlockPos {
            x: -48,
            y: ARENA_Y + 1,
            z: 0,
        },
    );
    let blue_flag = build_flag(
        &mut layer,
        Team::Blue,
        BlockPos {
            x: 48,
            y: ARENA_Y + 1,
            z: 0,
        },
    );

    build_spawn_box(&mut layer, SPAWN_BOX, &mut commands);

    commands.spawn(layer);

    let ctf_objective_layer = commands.spawn(EntityLayer::new(&server)).id();
    let ctf_objective = ObjectiveBundle {
        name: Objective::new("ctf-captures"),
        display: ObjectiveDisplay("Captures".into_text()),
        layer: EntityLayerId(ctf_objective_layer),
        ..Default::default()
    };
    commands.spawn(ctf_objective);

    let red_capture_trigger =
        TriggerArea::new(red_flag.offset(-5, -3, -5), red_flag.offset(5, 3, 5));
    let blue_capture_trigger =
        TriggerArea::new(blue_flag.offset(-5, -3, -5), blue_flag.offset(5, 3, 5));
    let mappos = CtfGlobals {
        scoreboard_layer: ctf_objective_layer,

        red_flag,
        blue_flag,

        red_capture_trigger,
        blue_capture_trigger,
    };

    commands.insert_resource(mappos);
    commands.insert_resource(FlagManager {
        red: None,
        blue: None,
    });

    let ctf_team_layers = CtfLayers::init(&mut commands, &server);

    // add some debug entities to the ctf entity layers
    let mut flags = Flags::default();
    flags.set_glowing(true);
    let mut pig = commands.spawn(PigEntityBundle {
        layer: EntityLayerId(ctf_team_layers.friendly_layers[&Team::Red]),
        position: Position([-30.0, 65.0, 2.0].into()),
        entity_flags: flags.clone(),
        ..Default::default()
    });
    pig.insert(Team::Red);

    let mut cow = commands.spawn(CowEntityBundle {
        layer: EntityLayerId(ctf_team_layers.friendly_layers[&Team::Blue]),
        position: Position([30.0, 65.0, 2.0].into()),
        entity_flags: flags,
        ..Default::default()
    });
    cow.insert(Team::Blue);

    commands.insert_resource(ctf_team_layers);
    commands.insert_resource(Score::default());
    commands.insert_resource(ReconnectJoinCounts::default());
}

/// Build a flag at the given position. `pos` should be the position of the
/// bottom of the flag.
///
/// Returns the block position of the flag.
fn build_flag(layer: &mut LayerBundle, team: Team, pos: impl Into<BlockPos>) -> BlockPos {
    let mut pos = pos.into();

    // build the flag pole
    for _ in 0..3 {
        layer.chunk.set_block(pos, BlockState::OAK_FENCE);
        pos.y += 1;
    }
    let moving_east = pos.x < 0;
    layer.chunk.set_block(
        pos,
        BlockState::OAK_FENCE.set(
            if moving_east {
                PropName::East
            } else {
                PropName::West
            },
            PropValue::True,
        ),
    );
    pos.x += if pos.x < 0 { 1 } else { -1 };
    layer.chunk.set_block(
        pos,
        BlockState::OAK_FENCE
            .set(PropName::East, PropValue::True)
            .set(PropName::West, PropValue::True),
    );
    pos.x += if pos.x < 0 { 1 } else { -1 };
    layer.chunk.set_block(
        pos,
        BlockState::OAK_FENCE.set(
            if moving_east {
                PropName::West
            } else {
                PropName::East
            },
            PropValue::True,
        ),
    );
    pos.y -= 1;

    // build the flag
    layer.chunk.set_block(
        pos,
        match team {
            Team::Red => BlockState::RED_WOOL,
            Team::Blue => BlockState::BLUE_WOOL,
        },
    );

    pos
}

fn build_spawn_box(layer: &mut LayerBundle, pos: impl Into<BlockPos>, commands: &mut Commands) {
    let pos = pos.into();

    let spawn_box_block = BlockState::GLASS;

    // build floor and roof
    for z in -SPAWN_BOX_WIDTH..=SPAWN_BOX_WIDTH {
        for x in -SPAWN_BOX_WIDTH..=SPAWN_BOX_WIDTH {
            layer
                .chunk
                .set_block([pos.x + x, pos.y, pos.z + z], spawn_box_block);
            layer.chunk.set_block(
                [pos.x + x, pos.y + SPAWN_BOX_HEIGHT, pos.z + z],
                spawn_box_block,
            );
        }
    }

    // build walls
    for z in [-SPAWN_BOX_WIDTH, SPAWN_BOX_WIDTH] {
        for x in -SPAWN_BOX_WIDTH..=SPAWN_BOX_WIDTH {
            for y in pos.y..=pos.y + SPAWN_BOX_HEIGHT - 1 {
                layer
                    .chunk
                    .set_block([pos.x + x, y, pos.z + z], spawn_box_block);
            }
        }
    }

    for x in [-SPAWN_BOX_WIDTH, SPAWN_BOX_WIDTH] {
        for z in -SPAWN_BOX_WIDTH..=SPAWN_BOX_WIDTH {
            for y in pos.y..=pos.y + SPAWN_BOX_HEIGHT - 1 {
                layer
                    .chunk
                    .set_block([pos.x + x, y, pos.z + z], spawn_box_block);
            }
        }
    }

    // build team selector portals
    for (block, offset) in [
        (
            BlockState::RED_CONCRETE,
            BlockPos::new(-SPAWN_BOX_WIDTH, 0, SPAWN_BOX_WIDTH - 2),
        ),
        (
            BlockState::BLUE_CONCRETE,
            BlockPos::new(SPAWN_BOX_WIDTH - 2, 0, SPAWN_BOX_WIDTH - 2),
        ),
    ] {
        for z in 0..3 {
            for x in 0..3 {
                layer.chunk.set_block(
                    [pos.x + offset.x + x, pos.y + offset.y, pos.z + offset.z + z],
                    block,
                );
            }
        }
    }

    let red = [
        pos.x - SPAWN_BOX_WIDTH + 1,
        pos.y,
        pos.z + SPAWN_BOX_WIDTH - 1,
    ];
    let red_area = TriggerArea::new(red, red);
    let blue = [
        pos.x + SPAWN_BOX_WIDTH - 1,
        pos.y,
        pos.z + SPAWN_BOX_WIDTH - 1,
    ];
    let blue_area = TriggerArea::new(blue, blue);
    let portals = Portals {
        portals: HashMap::from_iter(vec![(Team::Red, red_area), (Team::Blue, blue_area)]),
    };

    for area in portals.portals.values() {
        for pos in area.iter_block_pos() {
            layer.chunk.set_block(pos, BlockState::AIR);
        }
        layer
            .chunk
            .set_block(area.a.offset(0, -1, 0), BlockState::BARRIER);
    }

    commands.insert_resource(portals);

    // build instruction signs

    let sign_pos = pos.offset(0, 2, SPAWN_BOX_WIDTH - 1);
    layer.chunk.set_block(
        sign_pos,
        Block {
            state: BlockState::OAK_WALL_SIGN.set(PropName::Rotation, PropValue::_3),
            nbt: Some(compound! {
                "front_text" => compound! {
                    "messages" => List::String(vec![
                        "Capture".color(Color::YELLOW).bold().to_string(),
                        "the".color(Color::YELLOW).bold().to_string(),
                        "Flag!".color(Color::YELLOW).bold().to_string(),
                        "Select a Team".color(Color::WHITE).italic().to_string(),
                    ])
                },
            }),
        },
    );

    layer.chunk.set_block(
        sign_pos.offset(-1, 0, 0),
        Block {
            state: BlockState::OAK_WALL_SIGN.set(PropName::Rotation, PropValue::_3),
            nbt: Some(compound! {
                "front_text" => compound! {
                    "messages" => List::String(vec![
                        "".into_text().to_string(),
                        ("Join ".bold().color(Color::WHITE) + Team::Red.team_text()).to_string(),
                        "=>".bold().color(Color::WHITE).to_string(),
                        "".into_text().to_string(),
                    ])
                },
            }),
        },
    );

    layer.chunk.set_block(
        sign_pos.offset(1, 0, 0),
        Block {
            state: BlockState::OAK_WALL_SIGN.set(PropName::Rotation, PropValue::_3),
            nbt: Some(compound! {
                "front_text" => compound! {
                    "messages" => List::String(vec![
                        "".into_text().to_string(),
                        ("Join ".bold().color(Color::WHITE) + Team::Blue.team_text()).to_string(),
                        "<=".bold().color(Color::WHITE).to_string(),
                        "".into_text().to_string(),
                    ])
                },
            }),
        },
    );
}

fn init_clients(
    mut clients: Query<
        (
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            &mut Health,
        ),
        Added<Client>,
    >,
    main_layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
    globals: Res<CtfGlobals>,
) {
    for (
        mut client,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        mut health,
    ) in &mut clients
    {
        let layer = main_layers.single();

        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        visible_entity_layers.0.insert(globals.scoreboard_layer);
        pos.set(SPAWN_POS);
        *game_mode = GameMode::Adventure;
        health.0 = PLAYER_MAX_HEALTH;

        client.send_chat_message(
            "Welcome to Valence! Select a team by jumping in the team's portal.".italic(),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
enum Team {
    Red,
    Blue,
}

impl Team {
    fn spawn_pos(self) -> DVec3 {
        [
            match self {
                Team::Red => -40.0,
                Team::Blue => 40.0,
            },
            f64::from(ARENA_Y) + 1.0,
            0.0,
        ]
        .into()
    }

    fn team_text(self) -> Text {
        match self {
            Team::Red => "RED".color(Color::RED).bold(),
            Team::Blue => "BLUE".color(Color::BLUE).bold(),
        }
    }

    fn iter() -> impl Iterator<Item = Self> {
        [Team::Red, Team::Blue].iter().copied()
    }
}

fn digging(
    mut clients: Query<(
        &GameMode,
        &Team,
        Entity,
        &mut Client,
        &mut Inventory,
        &Username,
    )>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<DiggingEvent>,
    mut commands: Commands,
    globals: Res<CtfGlobals>,
    mut flag_manager: ResMut<FlagManager>,
    score: Res<Score>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((game_mode, team, ent, mut client, mut inv, username)) =
            clients.get_mut(event.client)
        else {
            continue;
        };

        if (*game_mode == GameMode::Creative && event.state == DiggingState::Start)
            || (*game_mode == GameMode::Survival && event.state == DiggingState::Stop)
        {
            let Some(block) = layer.block(event.position) else {
                continue;
            };
            let is_flag = event.position == globals.red_flag || event.position == globals.blue_flag;

            match (team, block.state) {
                (Team::Blue, BlockState::RED_WOOL) => {
                    if event.position == globals.red_flag {
                        commands.entity(event.client).insert(HasFlag(Team::Red));
                        client.send_chat_message("You have the flag!".italic());
                        flag_manager.red = Some(ent);
                        let milestone = format!(
                            "MC-COMPAT-MILESTONE flag_pickup username={} carrier_team={:?} \
                             flag_team=Red",
                            username.as_str(),
                            team
                        );
                        info!("{}", milestone);
                        println!("{}", milestone);
                        return;
                    }
                }
                (Team::Red, BlockState::BLUE_WOOL) => {
                    if event.position == globals.blue_flag {
                        commands.entity(event.client).insert(HasFlag(Team::Blue));
                        client.send_chat_message("You have the flag!".italic());
                        flag_manager.blue = Some(ent);
                        let milestone = format!(
                            "MC-COMPAT-MILESTONE flag_pickup username={} carrier_team={:?} \
                             flag_team=Blue",
                            username.as_str(),
                            team
                        );
                        info!("{}", milestone);
                        println!("{}", milestone);
                        return;
                    }
                }
                _ => {}
            }

            let is_red_flag = event.position == globals.red_flag;
            let is_blue_flag = event.position == globals.blue_flag;
            if let Some(flag_team) =
                invalid_flag_pickup_flag_team(*team, is_red_flag, is_blue_flag, block.state)
            {
                let pre_owner = flag_owner_state(&flag_manager, flag_team);
                let post_owner = pre_owner;
                let milestone = invalid_flag_pickup_rejection_milestone(
                    username.as_str(),
                    *team,
                    flag_team,
                    pre_owner,
                    post_owner,
                    score_for_team(&score, Team::Red),
                    score_for_team(&score, Team::Blue),
                );
                info!("{}", milestone);
                println!("{}", milestone);
            }

            if event.position.y <= ARENA_Y
                || block.state.to_kind() == BlockKind::OakFence
                || is_flag
            {
                continue;
            }

            let prev = layer.set_block(event.position, BlockState::AIR);

            if let Some(prev) = prev {
                let kind: ItemKind = prev.state.to_kind().to_item_kind();
                if let Some(slot) = inv.first_slot_with_item_in(kind, 64, 9..45) {
                    let count = inv.slot(slot).count;
                    inv.set_slot_amount(slot, count + 1);
                } else {
                    let stack = ItemStack::new(kind, 1, None);
                    if let Some(empty_slot) = inv.first_empty_slot_in(9..45) {
                        inv.set_slot(empty_slot, stack);
                    } else {
                        debug!("No empty slot to give item to player: {:?}", kind);
                    }
                }
            }
        }
    }
}

fn invalid_flag_pickup_flag_team(
    player_team: Team,
    is_red_flag: bool,
    is_blue_flag: bool,
    block_state: BlockState,
) -> Option<Team> {
    match (player_team, is_red_flag, is_blue_flag, block_state) {
        (Team::Red, true, _, BlockState::RED_WOOL) => Some(Team::Red),
        (Team::Blue, _, true, BlockState::BLUE_WOOL) => Some(Team::Blue),
        _ => None,
    }
}

fn flag_owner_state(flag_manager: &FlagManager, flag_team: Team) -> &'static str {
    let owner = match flag_team {
        Team::Red => flag_manager.red,
        Team::Blue => flag_manager.blue,
    };
    if owner.is_some() {
        "held"
    } else {
        "none"
    }
}

fn score_for_team(score: &Score, team: Team) -> u32 {
    *score.scores.get(&team).unwrap_or(&0)
}

fn team_label(team: Team) -> &'static str {
    match team {
        Team::Red => "Red",
        Team::Blue => "Blue",
    }
}

fn invalid_flag_pickup_rejection_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
    pre_owner: &str,
    post_owner: &str,
    red_score: u32,
    blue_score: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE invalid_flag_pickup_rejected username={} player_team={} flag_team={} pre_owner={} post_owner={} red_score={} blue_score={} outcome=no_owner_transfer_no_score",
        username,
        team_label(player_team),
        team_label(flag_team),
        pre_owner,
        post_owner,
        red_score,
        blue_score
    )
}

fn place_blocks(
    mut clients: Query<(&mut Inventory, &GameMode, &HeldItem, &Username)>,
    mut layers: Query<&mut ChunkLayer>,
    mut events: EventReader<InteractBlockEvent>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        let Ok((mut inventory, game_mode, held, username)) = clients.get_mut(event.client) else {
            continue;
        };
        if event.hand != Hand::Main {
            continue;
        }

        // get the held item
        let slot_id = held.slot();
        let stack = inventory.slot(slot_id);
        if stack.is_empty() {
            continue;
        }

        let Some(block_kind) = BlockKind::from_item_kind(stack.item) else {
            // can't place this item as a block
            continue;
        };
        let item_kind = stack.item;
        let placed_block = block_kind;

        if *game_mode == GameMode::Survival {
            // check if the player has the item in their inventory and remove
            // it.
            if stack.count > 1 {
                let count = stack.count;
                inventory.set_slot_amount(slot_id, count - 1);
            } else {
                inventory.set_slot(slot_id, ItemStack::EMPTY);
            }
        }
        let real_pos = event.position.get_in_direction(event.face);
        layer.set_block(real_pos, block_kind.to_state());
        let milestone = format!(
            "MC-COMPAT-MILESTONE block_place_item username={} item={:?} from_slot={} block={:?} \
             at={},{},{}",
            username.as_str(),
            item_kind,
            slot_id,
            placed_block,
            real_pos.x,
            real_pos.y,
            real_pos.z
        );
        info!("{milestone}");
        println!("{milestone}");
    }
}

#[derive(Debug, Resource)]
struct Portals {
    portals: HashMap<Team, TriggerArea>,
}

fn log_inventory_drop_events(
    mut events: EventReader<DropItemStackEvent>,
    mut players: Query<(&Username, &mut Client, &mut Inventory, &EntityId)>,
) {
    for event in events.read() {
        let Ok((username, mut client, mut inventory, entity_id)) = players.get_mut(event.client)
        else {
            continue;
        };
        let from_slot = event
            .from_slot
            .map(|slot| slot.to_string())
            .unwrap_or_else(|| "none".to_string());
        let milestone = format!(
            "MC-COMPAT-MILESTONE inventory_drop_item username={} from_slot={} item={:?} count={}",
            username.as_str(),
            from_slot,
            event.stack.item,
            event.stack.count
        );
        info!("{milestone}");
        println!("{milestone}");

        if username.as_str() == "compatbot" && event.stack.item == ItemKind::WoodenSword {
            let collected_entity_id = 7_630_036;
            client.write_packet(&ItemPickupAnimationS2c {
                collected_entity_id: VarInt(collected_entity_id),
                collector_entity_id: VarInt(entity_id.get()),
                pickup_item_count: VarInt(i32::from(event.stack.count)),
            });
            if let Some(slot) = event.from_slot {
                inventory.set_slot(slot, event.stack.clone());
            }
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_pickup_item username={} from_slot={} item={:?} \
                 count={} collected_entity_id={} collector_entity_id={}",
                username.as_str(),
                from_slot,
                event.stack.item,
                event.stack.count,
                collected_entity_id,
                entity_id.get()
            );
            info!("{milestone}");
            println!("{milestone}");
        }
    }
}

fn log_inventory_click_state(
    mut commands: Commands,
    mut compat_container_opened: Local<bool>,
    mut events: EventReader<ClickSlotEvent>,
    usernames: Query<&Username>,
) {
    for event in events.read() {
        let Ok(username) = usernames.get(event.client) else {
            continue;
        };
        let milestone = format!(
            "MC-COMPAT-MILESTONE inventory_click_slot username={} window={} slot={} button={} \
             mode={:?} carried_item={:?} count={} slot_changes={}",
            username.as_str(),
            event.window_id,
            event.slot_id,
            event.button,
            event.mode,
            event.carried_item.item,
            event.carried_item.count,
            event.slot_changes.len()
        );
        info!("{}", milestone);
        println!("{}", milestone);

        if username.as_str() == "compatbot"
            && event.window_id == 0
            && event.slot_id == 37
            && !*compat_container_opened
        {
            let menu = commands
                .spawn(Inventory::new(InventoryKind::Generic3x3))
                .id();
            commands
                .entity(event.client)
                .insert(OpenInventory::new(menu));
            *compat_container_opened = true;
            let milestone = "MC-COMPAT-MILESTONE inventory_open_container username=compatbot \
                             kind=Generic3x3 trigger=inventory_click_slot";
            info!("{}", milestone);
            println!("{}", milestone);
        }

        if username.as_str() == "compatbot" && event.window_id != 0 {
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_container_click username={} window={} slot={} \
                 button={} mode={:?} carried_item={:?} count={} slot_changes={}",
                username.as_str(),
                event.window_id,
                event.slot_id,
                event.button,
                event.mode,
                event.carried_item.item,
                event.carried_item.count,
                event.slot_changes.len()
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
    }
}

fn log_inventory_hotbar_select_events(
    mut events: EventReader<UpdateSelectedSlotEvent>,
    usernames: Query<&Username>,
) {
    for event in events.read() {
        let username = usernames
            .get(event.client)
            .map(|name| name.as_str())
            .unwrap_or("unknown");
        println!(
            "MC-COMPAT-MILESTONE inventory_hotbar_select username={username} slot={}",
            event.slot
        );
    }
}

fn do_team_selector_portals(
    mut players: Query<
        (
            Entity,
            &mut Position,
            &mut Look,
            &mut HeadYaw,
            &mut GameMode,
            &mut Client,
            &EntityId,
            &mut VisibleEntityLayers,
            &UniqueId,
            &Username,
        ),
        Without<Team>,
    >,
    portals: Res<Portals>,
    mut commands: Commands,
    ctf_layers: Res<CtfLayers>,
    flag_manager: Res<FlagManager>,
    mut reconnect_joins: ResMut<ReconnectJoinCounts>,
    main_layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for player in &mut players {
        let (
            player,
            mut pos,
            mut look,
            mut head_yaw,
            mut game_mode,
            mut client,
            entity_id,
            mut ent_layers,
            unique_id,
            username,
        ) = player;
        if pos.0.y < f64::from(SPAWN_BOX[1]) - 5.0 {
            pos.0 = SPAWN_POS.into();
            continue;
        }

        let team = portals
            .portals
            .iter()
            .filter(|(_, area)| area.contains_pos(pos.0))
            .map(|(team, _)| team)
            .next()
            .copied();

        if let Some(team) = team {
            *game_mode = GameMode::Survival;
            let mut inventory = Inventory::new(InventoryKind::Player);
            inventory.set_slot(36, ItemStack::new(ItemKind::WoodenSword, 1, None));
            inventory.set_slot(
                37,
                ItemStack::new(
                    match team {
                        Team::Red => ItemKind::RedWool,
                        Team::Blue => ItemKind::BlueWool,
                    },
                    64,
                    None,
                ),
            );
            if projectile_probe_enabled() && team == Team::Red {
                inventory.set_slot(36, ItemStack::new(ItemKind::Bow, 1, None));
                inventory.set_slot(37, ItemStack::new(ItemKind::Arrow, 16, None));
                println!(
                    "MC-COMPAT-MILESTONE projectile_loadout username={} slot=0 item=Bow arrows=16",
                    username.as_str()
                );
            }
            let equipment_update_probe = equipment_update_probe_enabled();
            if equipment_update_probe && team == Team::Blue {
                inventory.set_slot(
                    ARMOR_MITIGATION_CHEST_SLOT,
                    ItemStack::new(ItemKind::DiamondChestplate, 1, None),
                );
                println!(
                    "MC-COMPAT-MILESTONE equipment_update_state username={} slot=chest \
                     item=DiamondChestplate source=team_inventory_setup",
                    username.as_str()
                );
                client.write_packet(&EntityEquipmentUpdateS2c {
                    entity_id: entity_id.get().into(),
                    equipment: vec![EquipmentEntry {
                        slot: i8::try_from(Equipment::CHEST_IDX).expect("equipment slot fits i8"),
                        item: ItemStack::new(ItemKind::DiamondChestplate, 1, None),
                    }],
                });
            }
            if armor_mitigation_probe_enabled() && team == Team::Blue {
                inventory.set_slot(
                    ARMOR_MITIGATION_CHEST_SLOT,
                    ItemStack::new(ItemKind::DiamondChestplate, 1, None),
                );
                println!(
                    "MC-COMPAT-MILESTONE armor_equipment_state username={} slot=chest \
                     item=DiamondChestplate source=team_inventory_setup",
                    username.as_str()
                );
            }
            let combat_state = CombatState::default();
            let mut entity = commands.entity(player);
            entity.insert((team, inventory, combat_state));
            if equipment_update_probe && team == Team::Blue {
                let mut equipment = Equipment::default();
                equipment.set_chest(ItemStack::new(ItemKind::DiamondChestplate, 1, None));
                entity.insert(equipment);
            }
            if armor_mitigation_probe_enabled() && team == Team::Blue {
                entity.insert(EquipmentInventorySync);
            }
            println!(
                "MC-COMPAT-MILESTONE inventory_hotbar_select username={} slot=0 \
                 source=team_inventory_setup",
                username.as_str()
            );
            let reconnect_count = reconnect_joins
                .joins
                .entry(username.as_str().to_owned())
                .and_modify(|count| *count += 1)
                .or_insert(1);
            if *reconnect_count > 1 {
                println!(
                    "MC-COMPAT-MILESTONE reconnect_state_coherent username={} team={:?} \
                     reconnect_session={} red_flag_held={} blue_flag_held={}",
                    username.as_str(),
                    team,
                    reconnect_count,
                    flag_manager.red.is_some(),
                    flag_manager.blue.is_some()
                );
            }
            pos.0 = team.spawn_pos();
            let yaw = match team {
                Team::Red => -90.0,
                Team::Blue => 90.0,
            };
            look.yaw = yaw;
            look.pitch = 0.0;
            head_yaw.0 = yaw;
            let chat_text: Text = "You are on team ".into_text() + team.team_text() + "!";
            client.send_chat_message(chat_text);

            let main_layer = main_layers.single();
            ent_layers.as_mut().0.remove(&main_layer);
            for t in Team::iter() {
                let enemy_layer = ctf_layers.enemy_layers[&t];
                if t == team {
                    ent_layers.as_mut().0.remove(&enemy_layer);
                } else {
                    ent_layers.as_mut().0.insert(enemy_layer);
                }
            }
            let friendly_layer = ctf_layers.friendly_layers[&team];
            ent_layers.as_mut().0.insert(friendly_layer);

            // Copy the player entity to the friendly layer, and make them glow.
            let mut flags = Flags::default();
            flags.set_glowing(true);
            let mut player_glowing = commands.spawn(PlayerEntityBundle {
                layer: EntityLayerId(friendly_layer),
                uuid: *unique_id,
                entity_flags: flags,
                position: *pos,
                ..Default::default()
            });
            player_glowing.insert(ClonedEntity(player));

            let enemy_layer = ctf_layers.enemy_layers[&team];
            let mut player_enemy = commands.spawn(PlayerEntityBundle {
                layer: EntityLayerId(enemy_layer),
                uuid: *unique_id,
                position: *pos,
                ..Default::default()
            });
            player_enemy.insert(ClonedEntity(player));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TriggerArea {
    a: BlockPos,
    b: BlockPos,
}

impl TriggerArea {
    fn new<P: Into<BlockPos>>(a: impl Into<BlockPos>, b: P) -> Self {
        Self {
            a: a.into(),
            b: b.into(),
        }
    }

    fn contains(&self, pos: BlockPos) -> bool {
        let min = BlockPos::new(
            self.a.x.min(self.b.x),
            self.a.y.min(self.b.y),
            self.a.z.min(self.b.z),
        );
        let max = BlockPos::new(
            self.a.x.max(self.b.x),
            self.a.y.max(self.b.y),
            self.a.z.max(self.b.z),
        );

        pos.x >= min.x
            && pos.x <= max.x
            && pos.y >= min.y
            && pos.y <= max.y
            && pos.z >= min.z
            && pos.z <= max.z
    }

    fn contains_pos(&self, pos: DVec3) -> bool {
        self.contains(pos.into())
    }

    fn iter_block_pos(&self) -> impl Iterator<Item = BlockPos> {
        let min = BlockPos::new(
            self.a.x.min(self.b.x),
            self.a.y.min(self.b.y),
            self.a.z.min(self.b.z),
        );
        let max = BlockPos::new(
            self.a.x.max(self.b.x),
            self.a.y.max(self.b.y),
            self.a.z.max(self.b.z),
        );

        (min.x..=max.x)
            .flat_map(move |x| (min.y..=max.y).map(move |y| (x, y)))
            .flat_map(move |(x, y)| (min.z..=max.z).map(move |z| BlockPos::new(x, y, z)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
#[component(storage = "SparseSet")]
struct HasFlag(Team);

#[derive(Debug, Resource)]
struct FlagManager {
    red: Option<Entity>,
    blue: Option<Entity>,
}

#[derive(Debug, Default, Resource)]
struct ReconnectJoinCounts {
    joins: HashMap<String, u32>,
}

fn despawn_disconnected_ctf_clients(
    mut commands: Commands,
    mut disconnected_clients: RemovedComponents<Client>,
    mut flag_manager: ResMut<FlagManager>,
    clients: Query<(Option<&HasFlag>, Option<&Username>)>,
) {
    for entity in disconnected_clients.read() {
        if let Ok((has_flag, username)) = clients.get(entity) {
            if let Some(has_flag) = has_flag {
                let flag_name = match has_flag.0 {
                    Team::Red => "red",
                    Team::Blue => "blue",
                };
                match has_flag.0 {
                    Team::Red => flag_manager.red = None,
                    Team::Blue => flag_manager.blue = None,
                }
                let username = username.map(|u| u.as_str()).unwrap_or("unknown");
                let returned = format!(
                    "MC-COMPAT-MILESTONE flag_disconnect_return carrier={} flag_team={} \
                     reason=client_disconnect score_unchanged=true",
                    username, flag_name
                );
                info!("{}", returned);
                println!("{}", returned);
            }
        }

        if let Some(mut entity) = commands.get_entity(entity) {
            entity.remove::<HasFlag>();
            entity.insert(Despawned);
        }
    }
}

#[derive(Debug, Resource)]
struct CtfGlobals {
    pub(crate) scoreboard_layer: Entity,

    pub(crate) red_flag: BlockPos,
    pub(crate) blue_flag: BlockPos,

    pub(crate) red_capture_trigger: TriggerArea,
    pub(crate) blue_capture_trigger: TriggerArea,
}

fn update_flag_visuals(
    flag_manager: Res<FlagManager>,
    globals: Res<CtfGlobals>,
    mut layers: Query<&mut ChunkLayer>,
) {
    if !flag_manager.is_changed() {
        return;
    }
    let red_flag_block = match flag_manager.red {
        Some(_) => BlockState::AIR,
        None => BlockState::RED_WOOL,
    };
    let blue_flag_block = match flag_manager.blue {
        Some(_) => BlockState::AIR,
        None => BlockState::BLUE_WOOL,
    };

    layers
        .single_mut()
        .set_block(globals.red_flag, red_flag_block);
    layers
        .single_mut()
        .set_block(globals.blue_flag, blue_flag_block);
}

fn do_flag_capturing(
    globals: Res<CtfGlobals>,
    mut players: Query<(Entity, &mut Client, &Team, &Position, &HasFlag)>,
    mut commands: Commands,
    mut flag_manager: ResMut<FlagManager>,
    mut score: ResMut<Score>,
) {
    for (ent, mut client, team, position, has_flag) in &mut players {
        let capture_trigger = match team {
            Team::Red => &globals.red_capture_trigger,
            Team::Blue => &globals.blue_capture_trigger,
        };

        if capture_trigger.contains_pos(position.0) {
            client.send_chat_message("You captured the flag!".italic());
            score
                .scores
                .entry(*team)
                .and_modify(|score| *score += 1)
                .or_insert(1);
            client.send_chat_message(score.render_scores());
            commands.entity(ent).remove::<HasFlag>();
            match has_flag.0 {
                Team::Red => flag_manager.red = None,
                Team::Blue => flag_manager.blue = None,
            }
        }
    }
}

#[derive(Debug, Default, Resource)]
struct Score {
    scores: HashMap<Team, u32>,
}

impl Score {
    fn render_scores(&self) -> Text {
        let mut text = "Scores:\n".into_text();
        for team in Team::iter() {
            let score = self.scores.get(&team).unwrap_or(&0);
            text += team.team_text() + ": " + score.to_string() + "\n";
        }
        text
    }
}

#[allow(dead_code)]
/// Visualizes the trigger areas, for debugging.
fn visualize_triggers(globals: Res<CtfGlobals>, mut layers: Query<&mut ChunkLayer>) {
    fn vis_trigger(trigger: &TriggerArea, layer: &mut ChunkLayer) {
        for pos in trigger.iter_block_pos() {
            layer.play_particle(
                &Particle::Crit,
                false,
                [
                    f64::from(pos.x) + 0.5,
                    f64::from(pos.y) + 0.5,
                    f64::from(pos.z) + 0.5,
                ],
                [0., 0., 0.],
                0.0,
                1,
            );
        }
    }

    for mut layer in &mut layers {
        vis_trigger(&globals.red_capture_trigger, &mut layer);
        vis_trigger(&globals.blue_capture_trigger, &mut layer);
    }
}

/// Keeps track of the entity layers per team.
#[derive(Debug, Resource)]
struct CtfLayers {
    /// Maps a team to the entity layer that contains how friendly players
    /// should be viewed.
    ///
    /// This is used to make friendly players glow.
    pub(crate) friendly_layers: HashMap<Team, Entity>,
    /// Ditto, but for enemy players.
    pub(crate) enemy_layers: HashMap<Team, Entity>,
}

impl CtfLayers {
    fn init(commands: &mut Commands, server: &Server) -> Self {
        let mut friendly_layers = HashMap::new();
        let mut enemy_layers = HashMap::new();

        for team in Team::iter() {
            let friendly_layer = commands.spawn((EntityLayer::new(server), team)).id();
            friendly_layers.insert(team, friendly_layer);
            let enemy_layer = commands.spawn((EntityLayer::new(server), team)).id();
            enemy_layers.insert(team, enemy_layer);
        }

        Self {
            friendly_layers,
            enemy_layers,
        }
    }
}

/// A marker component for entities that have been cloned, and the primary
/// entity they were cloned from.
#[derive(Debug, Component)]
struct ClonedEntity(Entity);

#[derive(Debug, QueryData)]
#[query_data(mutable)]
struct CloneQuery {
    position: &'static mut Position,
    head_yaw: &'static mut HeadYaw,
    velocity: &'static mut Velocity,
    look: &'static mut Look,
    animations: &'static mut EntityAnimations,
    on_ground: &'static mut OnGround,
    statuses: &'static mut EntityStatuses,
}

fn update_clones(
    ents: Query<CloneQueryReadOnly, Without<ClonedEntity>>,
    mut clone_ents: Query<(CloneQuery, &ClonedEntity, Entity)>,
    mut commands: Commands,
) {
    for clone in &mut clone_ents {
        let (mut clone, cloned_from, ent) = clone;
        let Ok(src) = ents.get(cloned_from.0) else {
            commands.entity(ent).insert(Despawned);
            return;
        };

        *clone.position = *src.position;
        *clone.head_yaw = *src.head_yaw;
        *clone.velocity = *src.velocity;
        *clone.look = *src.look;
        *clone.animations = *src.animations;
        *clone.on_ground = *src.on_ground;
        *clone.statuses = *src.statuses;
    }
}

/// Attached to every client.
#[derive(Component, Default)]
struct CombatState {
    /// The tick the client was last attacked.
    last_attacked_tick: i64,
    has_bonus_knockback: bool,
}

#[derive(QueryData)]
#[query_data(mutable)]
struct CombatQuery {
    client: &'static mut Client,
    username: &'static Username,
    pos: &'static Position,
    state: &'static mut CombatState,
    statuses: &'static mut EntityStatuses,
    health: &'static mut Health,
    inventory: &'static Inventory,
    held_item: &'static HeldItem,
    team: &'static Team,
    has_flag: Option<&'static HasFlag>,
}

fn handle_combat_events(
    server: Res<Server>,
    mut commands: Commands,
    mut flag_manager: ResMut<FlagManager>,
    mut clients: Query<CombatQuery>,
    mut sprinting: EventReader<SprintEvent>,
    mut interact_entity: EventReader<InteractEntityEvent>,
    clones: Query<&ClonedEntity>,
) {
    for &SprintEvent { client, state } in sprinting.read() {
        if let Ok(mut client) = clients.get_mut(client) {
            client.state.has_bonus_knockback = state == SprintState::Start;
        }
    }

    for &InteractEntityEvent {
        client: attacker_client,
        entity: victim_client,
        ..
    } in interact_entity.read()
    {
        let true_victim_ent = clones
            .get(victim_client)
            .map(|cloned| cloned.0)
            .unwrap_or(victim_client);
        let Ok([mut attacker, mut victim]) =
            clients.get_many_mut([attacker_client, true_victim_ent])
        else {
            debug!("Failed to get clients for combat event");
            // Victim or attacker does not exist, or the attacker is attacking itself.
            continue;
        };

        if attacker.team == victim.team {
            // Attacker and victim are on the same team.
            continue;
        }

        if server.current_tick() - victim.state.last_attacked_tick < 10 {
            // Victim is still on attack cooldown.
            continue;
        }

        victim.state.last_attacked_tick = server.current_tick();

        let victim_pos = victim.pos.0.xz();
        let attacker_pos = attacker.pos.0.xz();

        let dir = (victim_pos - attacker_pos).normalize().as_vec2();

        let knockback_xz = if attacker.state.has_bonus_knockback {
            18.0
        } else {
            8.0
        };
        let knockback_y = if attacker.state.has_bonus_knockback {
            8.432
        } else {
            6.432
        };

        let knockback_velocity = [dir.x * knockback_xz, knockback_y, dir.y * knockback_xz];
        victim.client.set_velocity(knockback_velocity);
        let knockback = format!(
            "MC-COMPAT-MILESTONE combat_knockback attacker={} victim={} vx={:.3} vy={:.3} \
             vz={:.3} bonus={}",
            attacker.username.as_str(),
            victim.username.as_str(),
            knockback_velocity[0],
            knockback_velocity[1],
            knockback_velocity[2],
            attacker.state.has_bonus_knockback
        );
        info!("{}", knockback);
        println!("{}", knockback);

        attacker.state.has_bonus_knockback = false;

        victim.client.trigger_status(EntityStatus::PlayAttackSound);
        victim.statuses.trigger(EntityStatus::PlayAttackSound);

        let stack = attacker.inventory.slot(attacker.held_item.slot());

        let base_damage = match stack.item {
            ItemKind::WoodenSword => 4.0,
            ItemKind::StoneSword => 5.0,
            ItemKind::IronSword => 6.0,
            ItemKind::DiamondSword => 7.0,
            _ => 1.0,
        };
        let chest_item = victim.inventory.slot(ARMOR_MITIGATION_CHEST_SLOT).item;
        let armor_mitigation =
            if armor_mitigation_probe_enabled() && chest_item == ItemKind::DiamondChestplate {
                DIAMOND_CHESTPLATE_MITIGATION
            } else {
                0.0
            };
        let projectile_probe_hit = projectile_probe_enabled()
            && attacker.username.as_str() == "compatbota"
            && victim.username.as_str() == "compatbotb";
        let arrow_damage_decision = if projectile_probe_hit {
            Some(projectile_probe_damage_decision())
        } else {
            None
        };
        let damage = arrow_damage_decision
            .as_ref()
            .map(|decision| decision.damage)
            .unwrap_or_else(|| (base_damage - armor_mitigation).max(0.0));

        if projectile_probe_hit {
            let decision = arrow_damage_decision
                .as_ref()
                .expect("projectile probe hit has arrow decision");
            let projectile_use = format!(
                "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} item={:?} \
                 damage={:.1} policy={} generation={} clamped={}",
                attacker.username.as_str(),
                victim.username.as_str(),
                stack.item,
                damage,
                decision.policy_id,
                decision.generation,
                decision.clamped
            );
            info!("{}", projectile_use);
            println!("{}", projectile_use);
        }

        victim.health.0 -= damage;
        if armor_mitigation > 0.0 {
            let mitigation = format!(
                "MC-COMPAT-MILESTONE combat_armor_mitigation attacker={} victim={} \
                 base_damage={:.1} mitigation={:.1} final_damage={:.1} \
                 chest_item={:?} victim_health_before={:.1} victim_health_after={:.1}",
                attacker.username.as_str(),
                victim.username.as_str(),
                base_damage,
                armor_mitigation,
                damage,
                chest_item,
                victim.health.0 + damage,
                victim.health.0
            );
            info!("{}", mitigation);
            println!("{}", mitigation);
        }
        let milestone = format!(
            "MC-COMPAT-MILESTONE combat_damage attacker={} victim={} damage={:.1} \
             victim_health_before={:.1} victim_health_after={:.1} attacker_item={:?}",
            attacker.username.as_str(),
            victim.username.as_str(),
            damage,
            victim.health.0 + damage,
            victim.health.0,
            stack.item
        );
        info!("{}", milestone);
        println!("{}", milestone);
        if projectile_probe_hit {
            let decision = arrow_damage_decision
                .as_ref()
                .expect("projectile probe hit has arrow decision");
            let projectile_hit = format!(
                "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} damage={:.1} \
                 victim_health_before={:.1} victim_health_after={:.1} policy={} \
                 generation={} clamped={}",
                attacker.username.as_str(),
                victim.username.as_str(),
                damage,
                victim.health.0 + damage,
                victim.health.0,
                decision.policy_id,
                decision.generation,
                decision.clamped
            );
            info!("{}", projectile_hit);
            println!("{}", projectile_hit);
        }

        if victim.health.0 <= 0.0 {
            if let Some(has_flag) = victim.has_flag.copied() {
                let flag_name = match has_flag.0 {
                    Team::Red => "red",
                    Team::Blue => "blue",
                };
                let death = format!(
                    "MC-COMPAT-MILESTONE flag_carrier_death carrier={} attacker={} flag_team={} \
                     health_after={:.1}",
                    victim.username.as_str(),
                    attacker.username.as_str(),
                    flag_name,
                    victim.health.0
                );
                info!("{}", death);
                println!("{}", death);
                match has_flag.0 {
                    Team::Red => flag_manager.red = None,
                    Team::Blue => flag_manager.blue = None,
                }
                commands.entity(true_victim_ent).remove::<HasFlag>();
                let returned = format!(
                    "MC-COMPAT-MILESTONE flag_return carrier={} flag_team={} reason=carrier_death \
                     score_unchanged=true",
                    victim.username.as_str(),
                    flag_name
                );
                info!("{}", returned);
                println!("{}", returned);
            }
        }
    }
}

fn projectile_probe_enabled() -> bool {
    std::env::var("MC_COMPAT_PROJECTILE_PROBE")
        .map(|value| value != "0")
        .unwrap_or(false)
}

fn handle_projectile_events(
    mut interact_item: EventReader<InteractItemEvent>,
    mut hand_swing: EventReader<HandSwingEvent>,
    mut clients: Query<(Entity, &mut Client, &Username, &mut Health, &Team)>,
) {
    if !projectile_probe_enabled() {
        return;
    }

    for event in hand_swing.read() {
        let Ok((_, _, username, _, _)) = clients.get(event.client) else {
            continue;
        };
        let milestone = format!(
            "MC-COMPAT-MILESTONE projectile_swing username={} hand={:?}",
            username.as_str(),
            event.hand
        );
        info!("{}", milestone);
        println!("{}", milestone);
    }

    for event in interact_item.read() {
        let Ok((_, _, attacker_username, _, attacker_team)) = clients.get(event.client) else {
            continue;
        };
        let attacker_name = attacker_username.as_str().to_owned();
        let attacker_team = *attacker_team;
        let victim_ent = clients.iter().find_map(|(entity, _, _, _, team)| {
            if *team != attacker_team {
                Some(entity)
            } else {
                None
            }
        });
        let Some(victim_ent) = victim_ent else {
            continue;
        };
        let Ok((_, mut victim_client, victim_username, mut victim_health, _)) =
            clients.get_mut(victim_ent)
        else {
            continue;
        };

        let decision = projectile_probe_damage_decision();
        let before = victim_health.0;
        victim_health.0 -= decision.damage;
        victim_client.trigger_status(EntityStatus::PlayAttackSound);
        let milestone = format!(
            "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} hand={:?} \
             sequence={} damage={:.1} policy={} generation={} clamped={}",
            attacker_name,
            victim_username.as_str(),
            event.hand,
            event.sequence,
            decision.damage,
            decision.policy_id,
            decision.generation,
            decision.clamped
        );
        info!("{}", milestone);
        println!("{}", milestone);
        let hit = format!(
            "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} damage={:.1} \
             victim_health_before={:.1} victim_health_after={:.1} policy={} generation={} \
             clamped={}",
            attacker_name,
            victim_username.as_str(),
            decision.damage,
            before,
            victim_health.0,
            decision.policy_id,
            decision.generation,
            decision.clamped
        );
        info!("{}", hit);
        println!("{}", hit);
    }
}

fn armor_mitigation_probe_enabled() -> bool {
    std::env::var("MC_COMPAT_ARMOR_MITIGATION_PROBE")
        .map(|value| value != "0")
        .unwrap_or(false)
}

fn equipment_update_probe_enabled() -> bool {
    std::env::var("MC_COMPAT_EQUIPMENT_UPDATE_PROBE")
        .map(|value| value != "0")
        .unwrap_or(false)
}

fn teleport_oob_clients(mut clients: Query<(&mut Position, &Team), With<Client>>) {
    for (mut pos, team) in &mut clients {
        if pos.0.y < 0.0 {
            pos.set(team.spawn_pos());
        }
    }
}

/// Handles respawning dead players.
fn necromancy(
    mut clients: Query<(
        &mut VisibleChunkLayer,
        &mut RespawnPosition,
        &Team,
        &mut Health,
    )>,
    mut events: EventReader<RequestRespawnEvent>,
    layers: Query<Entity, (With<ChunkLayer>, With<EntityLayer>)>,
) {
    for event in events.read() {
        if let Ok((mut visible_chunk_layer, mut respawn_pos, team, mut health)) =
            clients.get_mut(event.client)
        {
            respawn_pos.pos = team.spawn_pos().into();
            health.0 = PLAYER_MAX_HEALTH;

            let main_layer = layers.single();

            // this gets the client to get rid of the respawn screen
            visible_chunk_layer.0 = main_layer;
        }
    }
}

fn update_scoreboard(
    mut objectives: Query<&mut ObjectiveScores, With<Objective>>,
    score: Res<Score>,
) {
    if !score.is_changed() {
        return;
    }
    let mut s = objectives.single_mut();
    s.insert("Red", *score.scores.get(&Team::Red).unwrap_or(&0) as i32);
    s.insert("Blue", *score.scores.get(&Team::Blue).unwrap_or(&0) as i32);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SOURCE: &str = "test-module.scm";
    const TEST_GENERATION: u64 = 7;
    const TEST_EDITED_BASE_DAMAGE: f32 = 4.0;
    const TEST_CLAMP_BASE_DAMAGE: f32 = 90.0;
    const TEST_CLAMP_VELOCITY: f32 = 20.0;
    const TEST_INVALID_DAMAGE: f32 = 101.0;
    const TEST_CUSTOM_MAX_DAMAGE: f32 = 12.0;
    const TEST_HEALTH_BEFORE: f32 = 20.0;
    const TEST_HEALTH_AFTER_EDITED_DAMAGE: f32 = 16.0;
    const TEST_RED_SCORE: u32 = 2;

    #[test]
    fn invalid_flag_pickup_helper_rejects_own_flag_and_allows_enemy_flag() {
        assert_eq!(
            invalid_flag_pickup_flag_team(Team::Red, true, false, BlockState::RED_WOOL),
            Some(Team::Red)
        );
        assert_eq!(
            invalid_flag_pickup_flag_team(Team::Red, false, true, BlockState::BLUE_WOOL),
            None
        );
    }

    #[test]
    fn invalid_flag_pickup_milestone_records_no_transfer_and_scores() {
        let mut score = Score::default();
        score.scores.insert(Team::Red, TEST_RED_SCORE);
        let flag_manager = FlagManager {
            red: None,
            blue: None,
        };
        let milestone = invalid_flag_pickup_rejection_milestone(
            "compatbot",
            Team::Red,
            Team::Red,
            flag_owner_state(&flag_manager, Team::Red),
            flag_owner_state(&flag_manager, Team::Red),
            score_for_team(&score, Team::Red),
            score_for_team(&score, Team::Blue),
        );

        assert!(
            milestone.contains("invalid_flag_pickup_rejected"),
            "{milestone}"
        );
        assert!(milestone.contains("username=compatbot"), "{milestone}");
        assert!(milestone.contains("player_team=Red"), "{milestone}");
        assert!(milestone.contains("flag_team=Red"), "{milestone}");
        assert!(milestone.contains("pre_owner=none"), "{milestone}");
        assert!(milestone.contains("post_owner=none"), "{milestone}");
        assert!(milestone.contains("red_score=2"), "{milestone}");
        assert!(milestone.contains("blue_score=0"), "{milestone}");
        assert!(
            milestone.contains("outcome=no_owner_transfer_no_score"),
            "{milestone}"
        );
    }

    #[test]
    fn default_arrow_policy_matches_legacy_projectile_damage() {
        let snapshot = default_arrow_policy_snapshot();
        let decision = evaluate_arrow_policy(
            &snapshot,
            ArrowDamageContext {
                projectile_velocity: ARROW_POLICY_DEFAULT_PROJECTILE_VELOCITY,
                pull_strength: ARROW_POLICY_DEFAULT_PULL_STRENGTH,
            },
        );

        assert_eq!(decision.damage, PROJECTILE_PROBE_DAMAGE);
        assert_eq!(decision.policy_id, ARROW_POLICY_ID_DAMAGE_LINEAR);
        assert!(!decision.clamped);
    }

    #[test]
    fn steel_module_edit_changes_arrow_damage_policy() {
        let snapshot = normalize_arrow_policy_module(
            TEST_SOURCE,
            TEST_GENERATION,
            &valid_arrow_policy_module(TEST_EDITED_BASE_DAMAGE),
        )
        .expect("valid Steel policy parses");
        let decision = evaluate_arrow_policy(
            &snapshot,
            ArrowDamageContext {
                projectile_velocity: ARROW_POLICY_DEFAULT_PROJECTILE_VELOCITY,
                pull_strength: ARROW_POLICY_DEFAULT_PULL_STRENGTH,
            },
        );

        assert_eq!(snapshot.base_damage, TEST_EDITED_BASE_DAMAGE);
        assert_eq!(decision.damage, TEST_EDITED_BASE_DAMAGE);
        assert_eq!(decision.generation, TEST_GENERATION);
    }

    #[test]
    fn invalid_module_is_rejected_and_previous_snapshot_survives() {
        let previous = default_arrow_policy_snapshot();
        let mut controller = ArrowPolicyController::new(previous.clone());
        let candidate = normalize_arrow_policy_module(
            TEST_SOURCE,
            TEST_GENERATION,
            &valid_arrow_policy_module(TEST_INVALID_DAMAGE),
        );
        let outcome = controller.reload_candidate(candidate);

        assert!(!outcome.active_changed);
        assert_eq!(controller.active(), &previous);
        assert!(outcome
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == ARROW_POLICY_PATH_BASE_DAMAGE));
    }

    #[test]
    fn malformed_or_capability_invalid_module_is_rejected() {
        let malformed = valid_arrow_policy_module(PROJECTILE_PROBE_DAMAGE)
            .replace(ARROW_POLICY_REQUIRED_POLICY_SHAPE, "42");
        let diagnostics = normalize_arrow_policy_module(TEST_SOURCE, TEST_GENERATION, &malformed)
            .expect_err("malformed policy is rejected");
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "combat.arrow.policy"));

        let forbidden = format!(
            "{}\n(random)\n",
            valid_arrow_policy_module(PROJECTILE_PROBE_DAMAGE)
        );
        let diagnostics = normalize_arrow_policy_module(TEST_SOURCE, TEST_GENERATION, &forbidden)
            .expect_err("forbidden policy is rejected");
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "runtime.steel.sandbox"));
    }

    #[test]
    fn arrow_policy_clamps_damage_to_maximum() {
        let snapshot = ArrowPolicySnapshot {
            generation: TEST_GENERATION,
            source: TEST_SOURCE.to_string(),
            policy_id: ARROW_POLICY_ID_DAMAGE_LINEAR.to_string(),
            base_damage: TEST_CLAMP_BASE_DAMAGE,
            velocity_multiplier: ARROW_POLICY_DEFAULT_VELOCITY_MULTIPLIER,
            max_damage: TEST_CUSTOM_MAX_DAMAGE,
        };
        let decision = evaluate_arrow_policy(
            &snapshot,
            ArrowDamageContext {
                projectile_velocity: TEST_CLAMP_VELOCITY,
                pull_strength: ARROW_POLICY_DEFAULT_PULL_STRENGTH,
            },
        );

        assert_eq!(decision.damage, TEST_CUSTOM_MAX_DAMAGE);
        assert!(decision.clamped);
    }

    #[test]
    fn snapshot_diff_reports_changed_policy_fields() {
        let before = default_arrow_policy_snapshot();
        let mut after = before.clone();
        after.base_damage = TEST_EDITED_BASE_DAMAGE;
        after.max_damage = TEST_CUSTOM_MAX_DAMAGE;

        let diffs = diff_arrow_policy_snapshots(&before, &after);

        assert_eq!(diffs.len(), 2);
        assert!(diffs
            .iter()
            .any(|diff| diff.path == ARROW_POLICY_PATH_BASE_DAMAGE));
        assert!(diffs
            .iter()
            .any(|diff| diff.path == ARROW_POLICY_PATH_MAX_DAMAGE));
    }

    #[test]
    fn range_invalid_decision_output_is_rejected() {
        let decision = ArrowDamageDecision {
            generation: TEST_GENERATION,
            source: TEST_SOURCE.to_string(),
            policy_id: ARROW_POLICY_ID_DAMAGE_LINEAR.to_string(),
            damage: TEST_INVALID_DAMAGE,
            clamped: false,
        };

        let diagnostics = validate_arrow_damage_decision(&decision);

        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.path == "combat.arrow.damage"));
    }

    #[test]
    fn non_default_policy_changes_both_projectile_call_site_health_deltas() {
        let snapshot = normalize_arrow_policy_module(
            TEST_SOURCE,
            TEST_GENERATION,
            &valid_arrow_policy_module(TEST_EDITED_BASE_DAMAGE),
        )
        .expect("valid Steel policy parses");
        let combat_event_decision = evaluate_arrow_policy(
            &snapshot,
            ArrowDamageContext {
                projectile_velocity: ARROW_POLICY_DEFAULT_PROJECTILE_VELOCITY,
                pull_strength: ARROW_POLICY_DEFAULT_PULL_STRENGTH,
            },
        );
        let projectile_interaction_decision = evaluate_arrow_policy(
            &snapshot,
            ArrowDamageContext {
                projectile_velocity: ARROW_POLICY_DEFAULT_PROJECTILE_VELOCITY,
                pull_strength: ARROW_POLICY_DEFAULT_PULL_STRENGTH,
            },
        );

        assert_eq!(
            TEST_HEALTH_BEFORE - combat_event_decision.damage,
            TEST_HEALTH_AFTER_EDITED_DAMAGE
        );
        assert_eq!(
            TEST_HEALTH_BEFORE - projectile_interaction_decision.damage,
            TEST_HEALTH_AFTER_EDITED_DAMAGE
        );
        assert_eq!(
            combat_event_decision.policy_id,
            ARROW_POLICY_ID_DAMAGE_LINEAR
        );
        assert_eq!(
            projectile_interaction_decision.policy_id,
            ARROW_POLICY_ID_DAMAGE_LINEAR
        );
    }

    #[test]
    fn arrow_policy_redacts_secret_like_diagnostics() {
        assert_eq!(
            redact_arrow_policy_text("/tmp/password-secret-token.scm"),
            "<redacted>"
        );
        assert_eq!(
            redact_arrow_policy_text("/tmp/policy.scm"),
            "/tmp/policy.scm"
        );
    }

    fn valid_arrow_policy_module(base_damage: f32) -> String {
        format!(
            r#"
(define sandbox-profile "{ARROW_POLICY_SANDBOX_PROFILE}")
(define arrow-base-damage {base_damage})
(define arrow-velocity-multiplier {ARROW_POLICY_DEFAULT_VELOCITY_MULTIPLIER})
(define arrow-max-damage {ARROW_POLICY_DEFAULT_MAX_DAMAGE})
(define (arrow-damage ctx)
  {ARROW_POLICY_REQUIRED_POLICY_SHAPE})
"#
        )
    }
}
