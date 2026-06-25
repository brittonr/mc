#![allow(clippy::type_complexity)]

mod scenario_contracts_generated;

use std::collections::HashMap;
use std::path::Path;
use std::sync::{OnceLock, RwLock};
use std::{env, fs};

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
    ClickMode, ClickSlotEvent, DropItemStackEvent, HeldItem, OpenInventory, UpdateSelectedSlotEvent,
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
const TEAM_RED_YAW: f32 = -90.0;
const TEAM_BLUE_YAW: f32 = 90.0;
const COMPAT_ACTOR_USERNAME: &str = "compatbot";
const INVENTORY_STACK_SPLIT_MERGE_PROBE_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_INVENTORY_STACK_SPLIT_MERGE_PROBE;
const INVENTORY_DRAG_TRANSACTIONS_PROBE_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_INVENTORY_DRAG_TRANSACTIONS_PROBE;
const INVENTORY_STACK_WINDOW_ID: u8 = 0;
const INVENTORY_STACK_SOURCE_SLOT: i16 = 37;
const INVENTORY_STACK_DESTINATION_SLOT: i16 = 38;
const INVENTORY_DRAG_TARGET_SLOT_A: i16 = 38;
const INVENTORY_DRAG_TARGET_SLOT_B: i16 = 39;
const INVENTORY_DRAG_OUTSIDE_SLOT: i16 = -999;
const INVENTORY_STACK_ITEM: ItemKind = ItemKind::RedWool;
const INVENTORY_STACK_FULL_COUNT: i8 = 64;
const INVENTORY_STACK_HALF_COUNT: i8 = 32;
const INVENTORY_STACK_EMPTY_COUNT: i8 = 0;
const INVENTORY_STACK_LEFT_BUTTON: i8 = 0;
const INVENTORY_STACK_RIGHT_BUTTON: i8 = 1;
const INVENTORY_DRAG_START_BUTTON: i8 = 0;
const INVENTORY_DRAG_ADD_SLOT_BUTTON: i8 = 1;
const INVENTORY_DRAG_END_BUTTON: i8 = 2;
const VANILLA_COMBAT_REFERENCE_PROBE_ENV: &str = "MC_COMPAT_VANILLA_COMBAT_REFERENCE_PROBE";
const VANILLA_COMBAT_ARMOR_REFERENCE_PROBE_ENV: &str =
    "MC_COMPAT_VANILLA_COMBAT_ARMOR_REFERENCE_PROBE";
const VANILLA_COMBAT_REFERENCE_ROW: &str = "vanilla-combat-reference-parity";
const VANILLA_COMBAT_ARMOR_REFERENCE_ROW: &str = "vanilla-combat-armor-reference-parity";
const VANILLA_COMBAT_REFERENCE_BACKEND: &str = "valence";
const VANILLA_COMBAT_REFERENCE_ORACLE: &str = "paper-1.20.1-reference-harness";
const VANILLA_COMBAT_REFERENCE_VERSION: &str = "minecraft-1.20.1-protocol-763";
const VANILLA_COMBAT_REFERENCE_ATTACKER: &str = "compatbota";
const VANILLA_COMBAT_REFERENCE_VICTIM: &str = "compatbotb";
const VANILLA_COMBAT_REFERENCE_ATTACKER_X: f64 = 38.0;
const VANILLA_COMBAT_REFERENCE_VICTIM_X: f64 = 40.0;
const VANILLA_COMBAT_REFERENCE_Y: f64 = 65.0;
const VANILLA_COMBAT_REFERENCE_Z: f64 = 0.0;
const VANILLA_COMBAT_REFERENCE_ATTACKER_YAW: f32 = TEAM_RED_YAW;
const VANILLA_COMBAT_REFERENCE_VICTIM_YAW: f32 = TEAM_BLUE_YAW;
const VANILLA_COMBAT_REFERENCE_ARMOR_NONE: &str = "none";
const VANILLA_COMBAT_REFERENCE_WEAPON_IRON_SWORD: &str = "iron_sword";
const VANILLA_COMBAT_REFERENCE_WEAPON_WOODEN_SWORD: &str = "wooden_sword";
const VANILLA_COMBAT_REFERENCE_WEAPON_STONE_SWORD: &str = "stone_sword";
const VANILLA_COMBAT_REFERENCE_WEAPON_DIAMOND_SWORD: &str = "diamond_sword";
const VANILLA_COMBAT_REFERENCE_WEAPON_OTHER: &str = "other";
const VANILLA_COMBAT_REFERENCE_ARMOR_DIAMOND_CHESTPLATE: &str = "diamond_chestplate";
const VANILLA_COMBAT_REFERENCE_ARMOR_OTHER: &str = "other";
const VANILLA_COMBAT_REFERENCE_DAMAGE_TOLERANCE: f32 = 0.0;
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_TOLERANCE: f64 = 0.05;
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_SCALE: f64 = 20.0;
const VANILLA_COMBAT_ARMOR_REDUCTION_DENOMINATOR: f32 = 25.0;
const VANILLA_DIAMOND_CHESTPLATE_ARMOR_POINTS: f32 = 8.0;
const VANILLA_DIAMOND_CHESTPLATE_TOUGHNESS: f32 = 2.0;
const VANILLA_ARMOR_TOUGHNESS_QUARTER_DIVISOR: f32 = 4.0;
const VANILLA_ARMOR_TOUGHNESS_BASE: f32 = 2.0;
const VANILLA_ARMOR_MIN_REDUCTION_DIVISOR: f32 = 5.0;
const VANILLA_ARMOR_MAX_REDUCTION_POINTS: f32 = 20.0;
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_X: f32 = 0.0;
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_Y: f32 = -0.08;
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_Z: f32 = 0.0;
const VANILLA_COMBAT_REFERENCE_KNOCKBACK_VELOCITY: [f32; 3] = [
    VANILLA_COMBAT_REFERENCE_KNOCKBACK_X,
    VANILLA_COMBAT_REFERENCE_KNOCKBACK_Y,
    VANILLA_COMBAT_REFERENCE_KNOCKBACK_Z,
];
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
const CTF_SCORE_LIMIT_WIN_PROBE_ENV: &str =
    scenario_contracts_generated::MC_COMPAT_CTF_SCORE_LIMIT_PROBE;
const CTF_SCORE_LIMIT_CONFIGURED: u32 = 2;
const CTF_SCORE_LIMIT_RED_PRE_FINAL_CAPTURE: u32 = 1;
const CTF_SCORE_LIMIT_BLUE_PRE_FINAL_CAPTURE: u32 = 0;
const CTF_SCORE_LIMIT_FIRST_WIN_EMISSION: u32 = 1;
const CTF_RACE_PROBE_ENV: &str = "MC_COMPAT_CTF_RACE_PROBE";
const CTF_RACE_WINDOW_TICKS: u32 = 40;
const CTF_RACE_FINAL_RED_SCORE: u32 = 1;
const CTF_RACE_FINAL_BLUE_SCORE: u32 = 0;
const CTF_RACE_ACCEPTED_TRANSITION: &str = "pickup";
const CTF_RACE_REJECTED_TRANSITION: &str = "duplicate_pickup";
const CTF_RACE_FINAL_BLUE_FLAG_STATE: &str = "at_base";
const CTF_SPAWN_TEAM_RESET_PROBE_ENV: &str = "MC_COMPAT_CTF_SPAWN_TEAM_RESET_PROBE";
const CTF_SPAWN_EXPECTED_RED_COUNT: u32 = 1;
const CTF_SPAWN_EXPECTED_BLUE_COUNT: u32 = 1;
const CTF_SPAWN_RESET_FINAL_RED_SCORE: u32 = 1;
const CTF_SPAWN_RESET_FINAL_BLUE_SCORE: u32 = 0;
const CTF_SPAWN_SLOT36_RESOURCE: &str = "WoodenSword:1";
const CTF_SPAWN_RED_SLOT37_RESOURCE: &str = "RedWool:64";
const CTF_SPAWN_BLUE_SLOT37_RESOURCE: &str = "BlueWool:64";
const CTF_SPAWN_RESET_SLOT37_RESOURCE: &str = "TeamWool:64";
const CTF_SPAWN_RESET_STATE: &str = "scoreboard_flags_and_resources_coherent";
const CTF_SPAWN_EXPECTED_BLUE_USERNAME: &str = "compatbotb";

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
        "MC-COMPAT-MILESTONE steel_arrow_policy_reject source={} active_generation={} \
         diagnostics={}",
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
    let score = initial_score_from_env();
    log_score_limit_pre_state(&score);
    commands.insert_resource(score);
    commands.insert_resource(WinConditionState::default());
    commands.insert_resource(ReconnectJoinCounts::default());
    commands.insert_resource(CtfRaceProbeState::default());
    commands.insert_resource(CtfSpawnTeamResetProbeState::default());
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct VanillaCombatReferenceAssignment {
    team: Team,
    position: DVec3,
    yaw: f32,
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
    mut race_probe: ResMut<CtfRaceProbeState>,
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
            if event.position == globals.red_flag
                && *team == Team::Blue
                && (flag_manager.red.is_some() || ctf_race_duplicate_pickup_blocked(&race_probe))
            {
                log_ctf_race_rejected_transition(
                    &mut race_probe,
                    username.as_str(),
                    *team,
                    Team::Red,
                );
                return;
            }
            if event.position == globals.blue_flag
                && *team == Team::Red
                && (flag_manager.blue.is_some() || ctf_race_duplicate_pickup_blocked(&race_probe))
            {
                log_ctf_race_rejected_transition(
                    &mut race_probe,
                    username.as_str(),
                    *team,
                    Team::Blue,
                );
                return;
            }

            match (team, block.state) {
                (Team::Blue, BlockState::RED_WOOL) => {
                    if event.position == globals.red_flag {
                        if flag_manager.red.is_some()
                            || ctf_race_duplicate_pickup_blocked(&race_probe)
                        {
                            log_ctf_race_rejected_transition(
                                &mut race_probe,
                                username.as_str(),
                                *team,
                                Team::Red,
                            );
                            return;
                        }
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
                        log_ctf_race_accepted_transition(
                            &mut race_probe,
                            username.as_str(),
                            *team,
                            Team::Red,
                        );
                        return;
                    }
                }
                (Team::Red, BlockState::BLUE_WOOL) => {
                    if event.position == globals.blue_flag {
                        if invalid_opponent_base_return_drop_probe_enabled() {
                            let red_score = score_for_team(&score, Team::Red);
                            let blue_score = score_for_team(&score, Team::Blue);
                            let pre_state = flag_presence_state(&flag_manager, Team::Blue);
                            let milestone = invalid_opponent_base_return_drop_rejection_milestone(
                                username.as_str(),
                                *team,
                                Team::Blue,
                                pre_state,
                                pre_state,
                                red_score,
                                blue_score,
                            );
                            info!("{}", milestone);
                            println!("{}", milestone);
                            return;
                        }
                        if flag_manager.blue.is_some()
                            || ctf_race_duplicate_pickup_blocked(&race_probe)
                        {
                            log_ctf_race_rejected_transition(
                                &mut race_probe,
                                username.as_str(),
                                *team,
                                Team::Blue,
                            );
                            return;
                        }
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
                        log_ctf_race_accepted_transition(
                            &mut race_probe,
                            username.as_str(),
                            *team,
                            Team::Blue,
                        );
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
                let red_score = score_for_team(&score, Team::Red);
                let blue_score = score_for_team(&score, Team::Blue);
                let milestone = if invalid_flag_return_drop_probe_enabled() {
                    let pre_state = flag_presence_state(&flag_manager, flag_team);
                    let post_state = pre_state;
                    invalid_flag_return_drop_rejection_milestone(
                        username.as_str(),
                        *team,
                        flag_team,
                        pre_state,
                        post_state,
                        red_score,
                        blue_score,
                    )
                } else {
                    let pre_owner = flag_owner_state(&flag_manager, flag_team);
                    let post_owner = pre_owner;
                    invalid_flag_pickup_rejection_milestone(
                        username.as_str(),
                        *team,
                        flag_team,
                        pre_owner,
                        post_owner,
                        red_score,
                        blue_score,
                    )
                };
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
    let owner = flag_owner(flag_manager, flag_team);
    if owner.is_some() {
        "held"
    } else {
        "none"
    }
}

fn flag_presence_state(flag_manager: &FlagManager, flag_team: Team) -> &'static str {
    let owner = flag_owner(flag_manager, flag_team);
    if owner.is_some() {
        "held"
    } else {
        "at_base"
    }
}

fn flag_owner(flag_manager: &FlagManager, flag_team: Team) -> Option<Entity> {
    match flag_team {
        Team::Red => flag_manager.red,
        Team::Blue => flag_manager.blue,
    }
}

const CTF_INVALID_RETURN_DROP_PROBE_ENV: &str = "MC_COMPAT_CTF_INVALID_RETURN_DROP_PROBE";
const CTF_INVALID_OPPONENT_BASE_RETURN_DROP_PROBE_ENV: &str =
    "MC_COMPAT_CTF_INVALID_OPPONENT_BASE_RETURN_DROP_PROBE";

fn invalid_flag_return_drop_probe_enabled() -> bool {
    env_flag_enabled(CTF_INVALID_RETURN_DROP_PROBE_ENV)
}

fn invalid_opponent_base_return_drop_probe_enabled() -> bool {
    env_flag_enabled(CTF_INVALID_OPPONENT_BASE_RETURN_DROP_PROBE_ENV)
}

fn env_flag_enabled(name: &str) -> bool {
    env::var(name).map(|value| value != "0").unwrap_or(false)
}

fn score_limit_win_probe_enabled() -> bool {
    env::var(CTF_SCORE_LIMIT_WIN_PROBE_ENV)
        .map(|value| value != "0")
        .unwrap_or(false)
}

fn ctf_race_probe_enabled() -> bool {
    env::var(CTF_RACE_PROBE_ENV)
        .map(|value| value != "0")
        .unwrap_or(false)
}

fn ctf_race_accepted_transition_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE ctf_race_accepted_transition username={} player_team={} \
         flag_team={} transition={} race_window_ticks={}",
        username,
        team_label(player_team),
        team_label(flag_team),
        CTF_RACE_ACCEPTED_TRANSITION,
        CTF_RACE_WINDOW_TICKS
    )
}

fn ctf_race_rejected_transition_milestone(
    username: &str,
    player_team: Team,
    flag_team: Team,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE ctf_race_rejected_transition username={} player_team={} \
         flag_team={} transition={} reason=flag_already_held race_window_ticks={}",
        username,
        team_label(player_team),
        team_label(flag_team),
        CTF_RACE_REJECTED_TRANSITION,
        CTF_RACE_WINDOW_TICKS
    )
}

fn ctf_race_duplicate_pickup_blocked(state: &CtfRaceProbeState) -> bool {
    ctf_race_probe_enabled() && state.accepted_username.is_some()
}

fn log_ctf_race_accepted_transition(
    state: &mut CtfRaceProbeState,
    username: &str,
    player_team: Team,
    flag_team: Team,
) {
    if !ctf_race_probe_enabled() {
        return;
    }
    if state.accepted_username.is_some() {
        let milestone = format!(
            "MC-COMPAT-MILESTONE ctf_race_double_accept username={} player_team={} \
             flag_team={} outcome=forbidden_double_accept",
            username,
            team_label(player_team),
            team_label(flag_team)
        );
        info!("{milestone}");
        println!("{milestone}");
        return;
    }
    state.accepted_username = Some(username.to_owned());
    let milestone = ctf_race_accepted_transition_milestone(username, player_team, flag_team);
    info!("{milestone}");
    println!("{milestone}");
}

fn log_ctf_race_rejected_transition(
    state: &mut CtfRaceProbeState,
    username: &str,
    player_team: Team,
    flag_team: Team,
) {
    if !ctf_race_probe_enabled() {
        return;
    }
    state.rejected_username = Some(username.to_owned());
    let milestone = ctf_race_rejected_transition_milestone(username, player_team, flag_team);
    info!("{milestone}");
    println!("{milestone}");
}

fn ctf_race_final_state_milestone(
    accepted_username: &str,
    rejected_username: &str,
    capture_username: &str,
    capture_team: Team,
    carried_flag: Team,
    red_score_after: u32,
    blue_score_after: u32,
    flag_manager: &FlagManager,
) -> Option<String> {
    let blue_flag_at_base = flag_manager.blue.is_none();
    let red_flag_at_base = flag_manager.red.is_none();
    if capture_team != Team::Red
        || carried_flag != Team::Blue
        || red_score_after != CTF_RACE_FINAL_RED_SCORE
        || blue_score_after != CTF_RACE_FINAL_BLUE_SCORE
        || !blue_flag_at_base
        || !red_flag_at_base
    {
        return None;
    }
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_race_final_state capture_username={} \
         accepted_username={} rejected_username={} capture_team={} carried_flag={} \
         final_blue_flag_state={} red_score={} blue_score={} race_window_ticks={} \
         accepted_transition={} rejected_transition={}",
        capture_username,
        accepted_username,
        rejected_username,
        team_label(capture_team),
        team_label(carried_flag),
        CTF_RACE_FINAL_BLUE_FLAG_STATE,
        red_score_after,
        blue_score_after,
        CTF_RACE_WINDOW_TICKS,
        CTF_RACE_ACCEPTED_TRANSITION,
        CTF_RACE_REJECTED_TRANSITION
    ))
}

fn log_ctf_race_final_state(
    state: &mut CtfRaceProbeState,
    capture_username: &str,
    capture_team: Team,
    carried_flag: Team,
    red_score_after: u32,
    blue_score_after: u32,
    flag_manager: &FlagManager,
) {
    if !ctf_race_probe_enabled() || state.final_logged {
        return;
    }
    let Some(accepted_username) = state.accepted_username.as_deref() else {
        return;
    };
    let Some(rejected_username) = state.rejected_username.as_deref() else {
        return;
    };
    let Some(milestone) = ctf_race_final_state_milestone(
        accepted_username,
        rejected_username,
        capture_username,
        capture_team,
        carried_flag,
        red_score_after,
        blue_score_after,
        flag_manager,
    ) else {
        return;
    };
    state.final_logged = true;
    info!("{milestone}");
    println!("{milestone}");
}

fn initial_score_from_env() -> Score {
    let mut score = Score::default();
    if score_limit_win_probe_enabled() {
        score
            .scores
            .insert(Team::Red, CTF_SCORE_LIMIT_RED_PRE_FINAL_CAPTURE);
        score
            .scores
            .insert(Team::Blue, CTF_SCORE_LIMIT_BLUE_PRE_FINAL_CAPTURE);
    }
    score
}

fn log_score_limit_pre_state(score: &Score) {
    if !score_limit_win_probe_enabled() {
        return;
    }
    let milestone = score_limit_pre_state_milestone(score);
    info!("{milestone}");
    println!("{milestone}");
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

fn ctf_spawn_team_reset_probe_enabled() -> bool {
    env::var_os(CTF_SPAWN_TEAM_RESET_PROBE_ENV).is_some()
}

fn team_spawn_parts(team: Team) -> (f64, f64, f64) {
    let pos = team.spawn_pos();
    (pos.x, pos.y, pos.z)
}

fn team_slot37_resource(team: Team) -> &'static str {
    match team {
        Team::Red => CTF_SPAWN_RED_SLOT37_RESOURCE,
        Team::Blue => CTF_SPAWN_BLUE_SLOT37_RESOURCE,
    }
}

fn ctf_spawn_reset_should_defer_team_assignment(username: &str, team: Team) -> bool {
    username == CTF_SPAWN_EXPECTED_BLUE_USERNAME && team == Team::Red
}

fn ctf_spawn_team_assignment_milestone(
    username: &str,
    team: Team,
    red_count: u32,
    blue_count: u32,
) -> String {
    let (spawn_x, spawn_y, spawn_z) = team_spawn_parts(team);
    format!(
        "MC-COMPAT-MILESTONE ctf_spawn_team_assignment username={} team={} red_count={} \
         blue_count={} spawn_x={:.1} spawn_y={:.1} spawn_z={:.1} slot36={} slot37={} \
         correlation_id=team-select-{}",
        username,
        team_label(team),
        red_count,
        blue_count,
        spawn_x,
        spawn_y,
        spawn_z,
        CTF_SPAWN_SLOT36_RESOURCE,
        team_slot37_resource(team),
        username
    )
}

fn ctf_spawn_team_balance_milestone(state: &CtfSpawnTeamResetProbeState) -> Option<String> {
    let red_username = state.red_username.as_deref()?;
    let blue_username = state.blue_username.as_deref()?;
    if state.red_count != CTF_SPAWN_EXPECTED_RED_COUNT
        || state.blue_count != CTF_SPAWN_EXPECTED_BLUE_COUNT
    {
        return None;
    }
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_spawn_team_balance red_count={} blue_count={} \
         selected_teams={}:Red,{}:Blue outcome=balanced",
        state.red_count, state.blue_count, red_username, blue_username
    ))
}

fn ctf_spawn_resource_reset_state_milestone(
    state: &CtfSpawnTeamResetProbeState,
    capture_username: &str,
    capture_team: Team,
    carried_flag: Team,
    score: &Score,
) -> Option<String> {
    if capture_team != Team::Red || carried_flag != Team::Blue {
        return None;
    }
    let red_score = score_for_team(score, Team::Red);
    let blue_score = score_for_team(score, Team::Blue);
    if state.red_count != CTF_SPAWN_EXPECTED_RED_COUNT
        || state.blue_count != CTF_SPAWN_EXPECTED_BLUE_COUNT
        || red_score != CTF_SPAWN_RESET_FINAL_RED_SCORE
        || blue_score != CTF_SPAWN_RESET_FINAL_BLUE_SCORE
    {
        return None;
    }
    let (red_x, red_y, red_z) = team_spawn_parts(Team::Red);
    let (blue_x, blue_y, blue_z) = team_spawn_parts(Team::Blue);
    Some(format!(
        "MC-COMPAT-MILESTONE ctf_spawn_resource_reset_state trigger=score \
         capture_username={} capture_team={} carried_flag={} red_count={} blue_count={} \
         red_score={} blue_score={} red_spawn={:.1},{:.1},{:.1} blue_spawn={:.1},{:.1},{:.1} \
         slot36={} slot37={} reset_state={} correlation_id=score-reset-{}",
        capture_username,
        team_label(capture_team),
        team_label(carried_flag),
        state.red_count,
        state.blue_count,
        red_score,
        blue_score,
        red_x,
        red_y,
        red_z,
        blue_x,
        blue_y,
        blue_z,
        CTF_SPAWN_SLOT36_RESOURCE,
        CTF_SPAWN_RESET_SLOT37_RESOURCE,
        CTF_SPAWN_RESET_STATE,
        capture_username
    ))
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
        "MC-COMPAT-MILESTONE invalid_flag_pickup_rejected username={} player_team={} flag_team={} \
         pre_owner={} post_owner={} red_score={} blue_score={} outcome=no_owner_transfer_no_score",
        username,
        team_label(player_team),
        team_label(flag_team),
        pre_owner,
        post_owner,
        red_score,
        blue_score
    )
}

fn invalid_flag_return_drop_rejection_milestone(
    username: &str,
    actor_team: Team,
    flag_team: Team,
    pre_state: &str,
    post_state: &str,
    red_score: u32,
    blue_score: u32,
) -> String {
    format_invalid_return_drop_rejection_milestone(
        "invalid_flag_return_drop_rejected",
        username,
        actor_team,
        flag_team,
        pre_state,
        post_state,
        red_score,
        blue_score,
    )
}

fn invalid_opponent_base_return_drop_rejection_milestone(
    username: &str,
    actor_team: Team,
    flag_team: Team,
    pre_state: &str,
    post_state: &str,
    red_score: u32,
    blue_score: u32,
) -> String {
    format_invalid_return_drop_rejection_milestone(
        "invalid_opponent_base_return_drop_rejected",
        username,
        actor_team,
        flag_team,
        pre_state,
        post_state,
        red_score,
        blue_score,
    )
}

fn format_invalid_return_drop_rejection_milestone(
    milestone: &str,
    username: &str,
    actor_team: Team,
    flag_team: Team,
    pre_state: &str,
    post_state: &str,
    red_score: u32,
    blue_score: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE {} username={} actor_team={} \
         flag_team={} pre_state={} post_state={} red_score={} blue_score={} \
         outcome=no_flag_state_mutation_no_score",
        milestone,
        username,
        team_label(actor_team),
        team_label(flag_team),
        pre_state,
        post_state,
        red_score,
        blue_score
    )
}

fn score_limit_pre_state_milestone(score: &Score) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_pre_state score_limit={} red_score={} blue_score={} \
         next_capture_team=Red outcome=one_capture_before_win",
        CTF_SCORE_LIMIT_CONFIGURED,
        score_for_team(score, Team::Red),
        score_for_team(score, Team::Blue)
    )
}

fn score_limit_final_capture_milestone(
    username: &str,
    capture_team: Team,
    carried_flag: Team,
    red_score_before: u32,
    blue_score_before: u32,
    red_score_after: u32,
    blue_score_after: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_final_capture username={} capture_team={} \
         carried_flag={} score_limit={} red_score_before={} blue_score_before={} \
         red_score_after={} blue_score_after={}",
        username,
        team_label(capture_team),
        team_label(carried_flag),
        CTF_SCORE_LIMIT_CONFIGURED,
        red_score_before,
        blue_score_before,
        red_score_after,
        blue_score_after
    )
}

fn score_limit_win_condition_milestone(
    username: &str,
    winning_team: Team,
    score: &Score,
    emission_count: u32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_win_condition username={} winning_team={} score_limit={} \
         red_score={} blue_score={} end_state=winner_declared win_emissions={} \
         duplicate_win=false post_win_score_delta=0",
        username,
        team_label(winning_team),
        CTF_SCORE_LIMIT_CONFIGURED,
        score_for_team(score, Team::Red),
        score_for_team(score, Team::Blue),
        emission_count
    )
}

fn score_limit_duplicate_win_milestone(username: &str, winning_team: Team) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_duplicate_win username={} winning_team={} score_limit={} \
         outcome=forbidden_duplicate_win",
        username,
        team_label(winning_team),
        CTF_SCORE_LIMIT_CONFIGURED
    )
}

fn score_limit_post_win_score_mutation_milestone(username: &str, winning_team: Team) -> String {
    format!(
        "MC-COMPAT-MILESTONE score_limit_post_win_score_mutation username={} winning_team={} \
         score_limit={} outcome=forbidden_score_after_win",
        username,
        team_label(winning_team),
        CTF_SCORE_LIMIT_CONFIGURED
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

#[derive(Debug, Default)]
struct InventoryStackSplitMergeProbeState {
    split_pickup_state_id: Option<i32>,
    split_place_state_id: Option<i32>,
    merge_pickup_state_id: Option<i32>,
    merge_place_state_id: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InventoryStackSplitMergeServerAction {
    SplitPickup,
    SplitPlace,
    MergePickup,
    MergePlace,
}

fn inventory_stack_slot_change_matches(
    event: &ClickSlotEvent,
    slot: i16,
    item: ItemKind,
    count: i8,
) -> bool {
    event.slot_changes.iter().any(|change| {
        change.idx == slot && change.stack.item == item && change.stack.count == count
    })
}

fn inventory_stack_slot_change_empty(event: &ClickSlotEvent, slot: i16) -> bool {
    event
        .slot_changes
        .iter()
        .any(|change| change.idx == slot && change.stack.is_empty())
}

fn classify_inventory_stack_split_merge_event(
    username: &str,
    event: &ClickSlotEvent,
    state: &InventoryStackSplitMergeProbeState,
) -> Option<InventoryStackSplitMergeServerAction> {
    if username != COMPAT_ACTOR_USERNAME
        || event.window_id != INVENTORY_STACK_WINDOW_ID
        || event.mode != ClickMode::Click
    {
        return None;
    }

    if state.split_pickup_state_id.is_none()
        && event.slot_id == INVENTORY_STACK_SOURCE_SLOT
        && event.button == INVENTORY_STACK_RIGHT_BUTTON
        && event.carried_item.item == INVENTORY_STACK_ITEM
        && event.carried_item.count == INVENTORY_STACK_HALF_COUNT
        && inventory_stack_slot_change_matches(
            event,
            INVENTORY_STACK_SOURCE_SLOT,
            INVENTORY_STACK_ITEM,
            INVENTORY_STACK_HALF_COUNT,
        )
    {
        return Some(InventoryStackSplitMergeServerAction::SplitPickup);
    }

    if state.split_pickup_state_id.is_some()
        && state.split_place_state_id.is_none()
        && event.slot_id == INVENTORY_STACK_DESTINATION_SLOT
        && event.button == INVENTORY_STACK_LEFT_BUTTON
        && event.carried_item.is_empty()
        && inventory_stack_slot_change_matches(
            event,
            INVENTORY_STACK_DESTINATION_SLOT,
            INVENTORY_STACK_ITEM,
            INVENTORY_STACK_HALF_COUNT,
        )
    {
        return Some(InventoryStackSplitMergeServerAction::SplitPlace);
    }

    if state.split_place_state_id.is_some()
        && state.merge_pickup_state_id.is_none()
        && event.slot_id == INVENTORY_STACK_DESTINATION_SLOT
        && event.button == INVENTORY_STACK_LEFT_BUTTON
        && event.carried_item.item == INVENTORY_STACK_ITEM
        && event.carried_item.count == INVENTORY_STACK_HALF_COUNT
        && inventory_stack_slot_change_empty(event, INVENTORY_STACK_DESTINATION_SLOT)
    {
        return Some(InventoryStackSplitMergeServerAction::MergePickup);
    }

    if state.merge_pickup_state_id.is_some()
        && state.merge_place_state_id.is_none()
        && event.slot_id == INVENTORY_STACK_SOURCE_SLOT
        && event.button == INVENTORY_STACK_LEFT_BUTTON
        && event.carried_item.is_empty()
        && inventory_stack_slot_change_matches(
            event,
            INVENTORY_STACK_SOURCE_SLOT,
            INVENTORY_STACK_ITEM,
            INVENTORY_STACK_FULL_COUNT,
        )
    {
        return Some(InventoryStackSplitMergeServerAction::MergePlace);
    }

    None
}

fn log_inventory_stack_split_merge_event(
    username: &str,
    event: &ClickSlotEvent,
    state: &mut InventoryStackSplitMergeProbeState,
) {
    if !inventory_stack_split_merge_probe_enabled() {
        return;
    }

    let Some(action) = classify_inventory_stack_split_merge_event(username, event, state) else {
        return;
    };

    match action {
        InventoryStackSplitMergeServerAction::SplitPickup => {
            state.split_pickup_state_id = Some(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_stack_server_split_pickup username={} window={} state_id={} source_slot={} button={} mode=Click item={:?} source_count_after={} carried_count={}",
                username,
                event.window_id,
                event.state_id,
                INVENTORY_STACK_SOURCE_SLOT,
                INVENTORY_STACK_RIGHT_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_HALF_COUNT,
                INVENTORY_STACK_HALF_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
        InventoryStackSplitMergeServerAction::SplitPlace => {
            state.split_place_state_id = Some(event.state_id);
            let split_pickup_state_id = state.split_pickup_state_id.unwrap_or(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_stack_server_split username={} window={} state_id_sequence={}->{} source_slot={} destination_slot={} button={} mode=Click item={:?} source_count_after={} destination_count_after={} carried_count={}",
                username,
                event.window_id,
                split_pickup_state_id,
                event.state_id,
                INVENTORY_STACK_SOURCE_SLOT,
                INVENTORY_STACK_DESTINATION_SLOT,
                INVENTORY_STACK_LEFT_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_HALF_COUNT,
                INVENTORY_STACK_HALF_COUNT,
                INVENTORY_STACK_EMPTY_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
        InventoryStackSplitMergeServerAction::MergePickup => {
            state.merge_pickup_state_id = Some(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_stack_server_merge_pickup username={} window={} state_id={} destination_slot={} button={} mode=Click item={:?} destination_count_after={} carried_count={}",
                username,
                event.window_id,
                event.state_id,
                INVENTORY_STACK_DESTINATION_SLOT,
                INVENTORY_STACK_LEFT_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_EMPTY_COUNT,
                INVENTORY_STACK_HALF_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
        InventoryStackSplitMergeServerAction::MergePlace => {
            state.merge_place_state_id = Some(event.state_id);
            let split_place_state_id = state.split_place_state_id.unwrap_or(event.state_id);
            let merge_pickup_state_id = state.merge_pickup_state_id.unwrap_or(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_stack_server_merge username={} window={} state_id_sequence={}->{}->{} source_slot={} destination_slot={} button={} mode=Click item={:?} source_count_after={} destination_count_after={} carried_count={}",
                username,
                event.window_id,
                split_place_state_id,
                merge_pickup_state_id,
                event.state_id,
                INVENTORY_STACK_SOURCE_SLOT,
                INVENTORY_STACK_DESTINATION_SLOT,
                INVENTORY_STACK_LEFT_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_FULL_COUNT,
                INVENTORY_STACK_EMPTY_COUNT,
                INVENTORY_STACK_EMPTY_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
    }
}

#[derive(Debug, Default)]
struct InventoryDragTransactionsProbeState {
    pickup_state_id: Option<i32>,
    drag_start_state_id: Option<i32>,
    target_a_state_id: Option<i32>,
    target_b_state_id: Option<i32>,
    drag_end_state_id: Option<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InventoryDragTransactionsServerAction {
    PickupSource,
    DragStart,
    AddTargetA,
    AddTargetB,
    DragEnd,
}

fn classify_inventory_drag_transactions_event(
    username: &str,
    event: &ClickSlotEvent,
    state: &InventoryDragTransactionsProbeState,
) -> Option<InventoryDragTransactionsServerAction> {
    if username != COMPAT_ACTOR_USERNAME || event.window_id != INVENTORY_STACK_WINDOW_ID {
        return None;
    }

    if state.pickup_state_id.is_none()
        && event.mode == ClickMode::Click
        && event.slot_id == INVENTORY_STACK_SOURCE_SLOT
        && event.button == INVENTORY_STACK_LEFT_BUTTON
        && event.carried_item.item == INVENTORY_STACK_ITEM
        && event.carried_item.count == INVENTORY_STACK_FULL_COUNT
        && inventory_stack_slot_change_empty(event, INVENTORY_STACK_SOURCE_SLOT)
    {
        return Some(InventoryDragTransactionsServerAction::PickupSource);
    }

    if state.pickup_state_id.is_some()
        && state.drag_start_state_id.is_none()
        && event.mode == ClickMode::Drag
        && event.slot_id == INVENTORY_DRAG_OUTSIDE_SLOT
        && event.button == INVENTORY_DRAG_START_BUTTON
        && event.carried_item.item == INVENTORY_STACK_ITEM
        && event.carried_item.count == INVENTORY_STACK_FULL_COUNT
        && event.slot_changes.is_empty()
    {
        return Some(InventoryDragTransactionsServerAction::DragStart);
    }

    if state.drag_start_state_id.is_some()
        && state.target_a_state_id.is_none()
        && event.mode == ClickMode::Drag
        && event.slot_id == INVENTORY_DRAG_TARGET_SLOT_A
        && event.button == INVENTORY_DRAG_ADD_SLOT_BUTTON
        && event.carried_item.item == INVENTORY_STACK_ITEM
        && event.carried_item.count == INVENTORY_STACK_FULL_COUNT
        && event.slot_changes.is_empty()
    {
        return Some(InventoryDragTransactionsServerAction::AddTargetA);
    }

    if state.target_a_state_id.is_some()
        && state.target_b_state_id.is_none()
        && event.mode == ClickMode::Drag
        && event.slot_id == INVENTORY_DRAG_TARGET_SLOT_B
        && event.button == INVENTORY_DRAG_ADD_SLOT_BUTTON
        && event.carried_item.item == INVENTORY_STACK_ITEM
        && event.carried_item.count == INVENTORY_STACK_FULL_COUNT
        && event.slot_changes.is_empty()
    {
        return Some(InventoryDragTransactionsServerAction::AddTargetB);
    }

    if state.target_b_state_id.is_some()
        && state.drag_end_state_id.is_none()
        && event.mode == ClickMode::Drag
        && event.slot_id == INVENTORY_DRAG_OUTSIDE_SLOT
        && event.button == INVENTORY_DRAG_END_BUTTON
        && event.carried_item.is_empty()
        && inventory_stack_slot_change_matches(
            event,
            INVENTORY_DRAG_TARGET_SLOT_A,
            INVENTORY_STACK_ITEM,
            INVENTORY_STACK_HALF_COUNT,
        )
        && inventory_stack_slot_change_matches(
            event,
            INVENTORY_DRAG_TARGET_SLOT_B,
            INVENTORY_STACK_ITEM,
            INVENTORY_STACK_HALF_COUNT,
        )
    {
        return Some(InventoryDragTransactionsServerAction::DragEnd);
    }

    None
}

fn log_inventory_drag_transactions_event(
    username: &str,
    event: &ClickSlotEvent,
    state: &mut InventoryDragTransactionsProbeState,
) {
    if !inventory_drag_transactions_probe_enabled() {
        return;
    }

    let Some(action) = classify_inventory_drag_transactions_event(username, event, state) else {
        return;
    };

    match action {
        InventoryDragTransactionsServerAction::PickupSource => {
            state.pickup_state_id = Some(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_drag_server_pickup username={} window={} state_id={} source_slot={} button={} mode=Click item={:?} source_count_after={} carried_count={}",
                username,
                event.window_id,
                event.state_id,
                INVENTORY_STACK_SOURCE_SLOT,
                INVENTORY_STACK_LEFT_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_EMPTY_COUNT,
                INVENTORY_STACK_FULL_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
        InventoryDragTransactionsServerAction::DragStart => {
            state.drag_start_state_id = Some(event.state_id);
            let pickup_state_id = state.pickup_state_id.unwrap_or(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_drag_server_start username={} window={} state_id_sequence={}->{} slot={} button={} mode=Drag item={:?} carried_count={}",
                username,
                event.window_id,
                pickup_state_id,
                event.state_id,
                INVENTORY_DRAG_OUTSIDE_SLOT,
                INVENTORY_DRAG_START_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_FULL_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
        InventoryDragTransactionsServerAction::AddTargetA => {
            state.target_a_state_id = Some(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_drag_server_target_a username={} window={} state_id={} target_slot={} button={} mode=Drag item={:?} carried_count={}",
                username,
                event.window_id,
                event.state_id,
                INVENTORY_DRAG_TARGET_SLOT_A,
                INVENTORY_DRAG_ADD_SLOT_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_FULL_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
        InventoryDragTransactionsServerAction::AddTargetB => {
            state.target_b_state_id = Some(event.state_id);
            let target_a_state_id = state.target_a_state_id.unwrap_or(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_drag_server_target_b username={} window={} state_id_sequence={}->{} target_slots={},{} button={} mode=Drag item={:?} carried_count={}",
                username,
                event.window_id,
                target_a_state_id,
                event.state_id,
                INVENTORY_DRAG_TARGET_SLOT_A,
                INVENTORY_DRAG_TARGET_SLOT_B,
                INVENTORY_DRAG_ADD_SLOT_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_FULL_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
        InventoryDragTransactionsServerAction::DragEnd => {
            state.drag_end_state_id = Some(event.state_id);
            let pickup_state_id = state.pickup_state_id.unwrap_or(event.state_id);
            let drag_start_state_id = state.drag_start_state_id.unwrap_or(event.state_id);
            let target_a_state_id = state.target_a_state_id.unwrap_or(event.state_id);
            let target_b_state_id = state.target_b_state_id.unwrap_or(event.state_id);
            let milestone = format!(
                "MC-COMPAT-MILESTONE inventory_drag_server_end username={} window={} state_id_sequence={}->{}->{}->{}->{} source_slot={} target_slots={},{} button={} mode=Drag item={:?} source_count_after={} target_counts={},{} carried_count={}",
                username,
                event.window_id,
                pickup_state_id,
                drag_start_state_id,
                target_a_state_id,
                target_b_state_id,
                event.state_id,
                INVENTORY_STACK_SOURCE_SLOT,
                INVENTORY_DRAG_TARGET_SLOT_A,
                INVENTORY_DRAG_TARGET_SLOT_B,
                INVENTORY_DRAG_END_BUTTON,
                INVENTORY_STACK_ITEM,
                INVENTORY_STACK_EMPTY_COUNT,
                INVENTORY_STACK_HALF_COUNT,
                INVENTORY_STACK_HALF_COUNT,
                INVENTORY_STACK_EMPTY_COUNT
            );
            info!("{}", milestone);
            println!("{}", milestone);
        }
    }
}

fn log_inventory_click_state(
    mut commands: Commands,
    mut compat_container_opened: Local<bool>,
    mut inventory_stack_state: Local<InventoryStackSplitMergeProbeState>,
    mut inventory_drag_state: Local<InventoryDragTransactionsProbeState>,
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
        log_inventory_stack_split_merge_event(
            username.as_str(),
            event,
            &mut *inventory_stack_state,
        );
        log_inventory_drag_transactions_event(username.as_str(), event, &mut *inventory_drag_state);

        if username.as_str() == COMPAT_ACTOR_USERNAME
            && !inventory_stack_split_merge_probe_enabled()
            && !inventory_drag_transactions_probe_enabled()
            && event.window_id == INVENTORY_STACK_WINDOW_ID
            && event.slot_id == INVENTORY_STACK_SOURCE_SLOT
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

        if username.as_str() == COMPAT_ACTOR_USERNAME
            && event.window_id != INVENTORY_STACK_WINDOW_ID
        {
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
    mut spawn_reset_probe: ResMut<CtfSpawnTeamResetProbeState>,
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

        let vanilla_assignment = vanilla_combat_reference_assignment(username.as_str());
        let portal_team = portals
            .portals
            .iter()
            .filter(|(_, area)| area.contains_pos(pos.0))
            .map(|(team, _)| team)
            .next()
            .copied();
        let team = vanilla_assignment.map_or(portal_team, |assignment| Some(assignment.team));

        if let Some(team) = team {
            if vanilla_assignment.is_none()
                && ctf_spawn_team_reset_probe_enabled()
                && ctf_spawn_reset_should_defer_team_assignment(username.as_str(), team)
            {
                continue;
            }
            *game_mode = GameMode::Survival;
            let mut inventory = Inventory::new(InventoryKind::Player);
            let main_hand_item = if vanilla_assignment.is_some()
                && username.as_str() == VANILLA_COMBAT_REFERENCE_ATTACKER
            {
                ItemKind::IronSword
            } else {
                ItemKind::WoodenSword
            };
            inventory.set_slot(36, ItemStack::new(main_hand_item, 1, None));
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
            let armor_reference_probe = vanilla_combat_armor_reference_probe_enabled();
            let armor_mitigation_probe = armor_mitigation_probe_enabled();
            if (armor_mitigation_probe || armor_reference_probe) && team == Team::Blue {
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
            if (armor_mitigation_probe || armor_reference_probe) && team == Team::Blue {
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
            pos.0 = vanilla_assignment
                .map_or_else(|| team.spawn_pos(), |assignment| assignment.position);
            let yaw = vanilla_assignment.map_or_else(
                || match team {
                    Team::Red => TEAM_RED_YAW,
                    Team::Blue => TEAM_BLUE_YAW,
                },
                |assignment| assignment.yaw,
            );
            look.yaw = yaw;
            look.pitch = 0.0;
            head_yaw.0 = yaw;
            if ctf_spawn_team_reset_probe_enabled() {
                spawn_reset_probe.record_assignment(username.as_str(), team);
                let assignment = ctf_spawn_team_assignment_milestone(
                    username.as_str(),
                    team,
                    spawn_reset_probe.red_count,
                    spawn_reset_probe.blue_count,
                );
                info!("{assignment}");
                println!("{assignment}");
                if !spawn_reset_probe.balance_logged {
                    if let Some(balance) = ctf_spawn_team_balance_milestone(&spawn_reset_probe) {
                        spawn_reset_probe.balance_logged = true;
                        info!("{balance}");
                        println!("{balance}");
                    }
                }
            }
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
struct CtfRaceProbeState {
    accepted_username: Option<String>,
    rejected_username: Option<String>,
    final_logged: bool,
}

#[derive(Debug, Default, Resource)]
struct CtfSpawnTeamResetProbeState {
    red_count: u32,
    blue_count: u32,
    red_username: Option<String>,
    blue_username: Option<String>,
    balance_logged: bool,
    reset_logged: bool,
}

impl CtfSpawnTeamResetProbeState {
    fn record_assignment(&mut self, username: &str, team: Team) {
        match team {
            Team::Red => {
                self.red_count += 1;
                self.red_username = Some(username.to_owned());
            }
            Team::Blue => {
                self.blue_count += 1;
                self.blue_username = Some(username.to_owned());
            }
        }
    }
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
    mut players: Query<(Entity, &mut Client, &Team, &Position, &HasFlag, &Username)>,
    mut commands: Commands,
    mut flag_manager: ResMut<FlagManager>,
    mut score: ResMut<Score>,
    mut win_condition: ResMut<WinConditionState>,
    mut race_probe: ResMut<CtfRaceProbeState>,
    mut spawn_reset_probe: ResMut<CtfSpawnTeamResetProbeState>,
) {
    for (ent, mut client, team, position, has_flag, username) in &mut players {
        let capture_trigger = match team {
            Team::Red => &globals.red_capture_trigger,
            Team::Blue => &globals.blue_capture_trigger,
        };

        if capture_trigger.contains_pos(position.0) {
            let red_score_before = score_for_team(&score, Team::Red);
            let blue_score_before = score_for_team(&score, Team::Blue);
            client.send_chat_message("You captured the flag!".italic());
            score
                .scores
                .entry(*team)
                .and_modify(|score| *score += 1)
                .or_insert(1);
            client.send_chat_message(score.render_scores());
            let red_score_after = score_for_team(&score, Team::Red);
            let blue_score_after = score_for_team(&score, Team::Blue);
            log_score_limit_capture_and_win(
                username.as_str(),
                *team,
                has_flag.0,
                red_score_before,
                blue_score_before,
                red_score_after,
                blue_score_after,
                &score,
                &mut win_condition,
            );
            commands.entity(ent).remove::<HasFlag>();
            match has_flag.0 {
                Team::Red => flag_manager.red = None,
                Team::Blue => flag_manager.blue = None,
            }
            log_ctf_race_final_state(
                &mut race_probe,
                username.as_str(),
                *team,
                has_flag.0,
                red_score_after,
                blue_score_after,
                &flag_manager,
            );
            log_ctf_spawn_resource_reset_state(
                &mut spawn_reset_probe,
                username.as_str(),
                *team,
                has_flag.0,
                &score,
            );
        }
    }
}

fn log_ctf_spawn_resource_reset_state(
    state: &mut CtfSpawnTeamResetProbeState,
    capture_username: &str,
    capture_team: Team,
    carried_flag: Team,
    score: &Score,
) {
    if !ctf_spawn_team_reset_probe_enabled() || state.reset_logged {
        return;
    }
    if let Some(milestone) = ctf_spawn_resource_reset_state_milestone(
        state,
        capture_username,
        capture_team,
        carried_flag,
        score,
    ) {
        state.reset_logged = true;
        info!("{milestone}");
        println!("{milestone}");
    }
}

fn log_score_limit_capture_and_win(
    username: &str,
    capture_team: Team,
    carried_flag: Team,
    red_score_before: u32,
    blue_score_before: u32,
    red_score_after: u32,
    blue_score_after: u32,
    score: &Score,
    win_condition: &mut WinConditionState,
) {
    if !score_limit_win_probe_enabled() {
        return;
    }
    let final_capture = score_limit_final_capture_milestone(
        username,
        capture_team,
        carried_flag,
        red_score_before,
        blue_score_before,
        red_score_after,
        blue_score_after,
    );
    info!("{final_capture}");
    println!("{final_capture}");

    let winning_score = score_for_team(score, capture_team);
    if winning_score < CTF_SCORE_LIMIT_CONFIGURED {
        return;
    }
    if win_condition.winner.is_some() {
        let mutation = score_limit_post_win_score_mutation_milestone(username, capture_team);
        info!("{mutation}");
        println!("{mutation}");
        let duplicate = score_limit_duplicate_win_milestone(username, capture_team);
        info!("{duplicate}");
        println!("{duplicate}");
        return;
    }
    win_condition.winner = Some(capture_team);
    win_condition.win_emissions = CTF_SCORE_LIMIT_FIRST_WIN_EMISSION;
    let win = score_limit_win_condition_milestone(
        username,
        capture_team,
        score,
        win_condition.win_emissions,
    );
    info!("{win}");
    println!("{win}");
}

#[derive(Debug, Default, Resource)]
struct Score {
    scores: HashMap<Team, u32>,
}

#[derive(Debug, Default, Resource)]
struct WinConditionState {
    winner: Option<Team>,
    win_emissions: u32,
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
        let vanilla_combat_reference_hit = vanilla_combat_reference_probe_hit(
            attacker.username.as_str(),
            victim.username.as_str(),
        );

        let knockback_velocity =
            vanilla_combat_reference_knockback_velocity_for(vanilla_combat_reference_hit)
                .unwrap_or_else(|| {
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

                    [dir.x * knockback_xz, knockback_y, dir.y * knockback_xz]
                });
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
        if vanilla_combat_reference_hit {
            let reference_knockback = vanilla_combat_reference_knockback_milestone(
                attacker.username.as_str(),
                victim.username.as_str(),
                vanilla_combat_reference_knockback_metric(knockback_velocity),
            );
            info!("{}", reference_knockback);
            println!("{}", reference_knockback);
        }

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
        let armor_mitigation = combat_armor_mitigation_for(
            vanilla_combat_armor_reference_probe_enabled(),
            armor_mitigation_probe_enabled(),
            chest_item,
            base_damage,
        );
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
                "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} item={:?} damage={:.1} \
                 policy={} generation={} clamped={}",
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
                 base_damage={:.1} mitigation={:.1} final_damage={:.1} chest_item={:?} \
                 victim_health_before={:.1} victim_health_after={:.1}",
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
        if vanilla_combat_reference_hit {
            let reference_damage = vanilla_combat_reference_damage_milestone(
                vanilla_combat_reference_row(),
                attacker.username.as_str(),
                victim.username.as_str(),
                vanilla_combat_reference_weapon_name(stack.item),
                vanilla_combat_reference_armor_state(chest_item),
                victim.health.0 + damage,
                victim.health.0,
                damage,
            );
            info!("{}", reference_damage);
            println!("{}", reference_damage);
        }
        if projectile_probe_hit {
            let decision = arrow_damage_decision
                .as_ref()
                .expect("projectile probe hit has arrow decision");
            let projectile_hit = format!(
                "MC-COMPAT-MILESTONE projectile_hit attacker={} victim={} damage={:.1} \
                 victim_health_before={:.1} victim_health_after={:.1} policy={} generation={} \
                 clamped={}",
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

fn vanilla_combat_reference_probe_enabled() -> bool {
    env_flag_enabled(VANILLA_COMBAT_REFERENCE_PROBE_ENV)
        || vanilla_combat_armor_reference_probe_enabled()
}

fn vanilla_combat_armor_reference_probe_enabled() -> bool {
    env_flag_enabled(VANILLA_COMBAT_ARMOR_REFERENCE_PROBE_ENV)
}

fn inventory_stack_split_merge_probe_enabled() -> bool {
    env_flag_enabled(INVENTORY_STACK_SPLIT_MERGE_PROBE_ENV)
}

fn inventory_drag_transactions_probe_enabled() -> bool {
    env_flag_enabled(INVENTORY_DRAG_TRANSACTIONS_PROBE_ENV)
}

fn vanilla_combat_reference_probe_hit(attacker: &str, victim: &str) -> bool {
    vanilla_combat_reference_probe_hit_for(
        vanilla_combat_reference_probe_enabled(),
        attacker,
        victim,
    )
}

fn vanilla_combat_reference_assignment_for(
    enabled: bool,
    username: &str,
) -> Option<VanillaCombatReferenceAssignment> {
    if !enabled {
        return None;
    }
    match username {
        VANILLA_COMBAT_REFERENCE_ATTACKER => Some(VanillaCombatReferenceAssignment {
            team: Team::Red,
            position: [
                VANILLA_COMBAT_REFERENCE_ATTACKER_X,
                VANILLA_COMBAT_REFERENCE_Y,
                VANILLA_COMBAT_REFERENCE_Z,
            ]
            .into(),
            yaw: VANILLA_COMBAT_REFERENCE_ATTACKER_YAW,
        }),
        VANILLA_COMBAT_REFERENCE_VICTIM => Some(VanillaCombatReferenceAssignment {
            team: Team::Blue,
            position: [
                VANILLA_COMBAT_REFERENCE_VICTIM_X,
                VANILLA_COMBAT_REFERENCE_Y,
                VANILLA_COMBAT_REFERENCE_Z,
            ]
            .into(),
            yaw: VANILLA_COMBAT_REFERENCE_VICTIM_YAW,
        }),
        _ => None,
    }
}

fn vanilla_combat_reference_assignment(username: &str) -> Option<VanillaCombatReferenceAssignment> {
    vanilla_combat_reference_assignment_for(vanilla_combat_reference_probe_enabled(), username)
}

fn vanilla_combat_reference_probe_hit_for(enabled: bool, attacker: &str, victim: &str) -> bool {
    enabled
        && attacker == VANILLA_COMBAT_REFERENCE_ATTACKER
        && victim == VANILLA_COMBAT_REFERENCE_VICTIM
}

fn vanilla_combat_reference_weapon_name(item: ItemKind) -> &'static str {
    match item {
        ItemKind::IronSword => VANILLA_COMBAT_REFERENCE_WEAPON_IRON_SWORD,
        ItemKind::WoodenSword => VANILLA_COMBAT_REFERENCE_WEAPON_WOODEN_SWORD,
        ItemKind::StoneSword => VANILLA_COMBAT_REFERENCE_WEAPON_STONE_SWORD,
        ItemKind::DiamondSword => VANILLA_COMBAT_REFERENCE_WEAPON_DIAMOND_SWORD,
        _ => VANILLA_COMBAT_REFERENCE_WEAPON_OTHER,
    }
}

fn vanilla_combat_reference_armor_state(chest_item: ItemKind) -> &'static str {
    match chest_item {
        ItemKind::Air => VANILLA_COMBAT_REFERENCE_ARMOR_NONE,
        ItemKind::DiamondChestplate => VANILLA_COMBAT_REFERENCE_ARMOR_DIAMOND_CHESTPLATE,
        _ => VANILLA_COMBAT_REFERENCE_ARMOR_OTHER,
    }
}

fn vanilla_combat_reference_row() -> &'static str {
    if vanilla_combat_armor_reference_probe_enabled() {
        return VANILLA_COMBAT_ARMOR_REFERENCE_ROW;
    }
    VANILLA_COMBAT_REFERENCE_ROW
}

fn combat_armor_mitigation_for(
    armor_reference_probe: bool,
    armor_mitigation_probe: bool,
    chest_item: ItemKind,
    base_damage: f32,
) -> f32 {
    if chest_item != ItemKind::DiamondChestplate {
        return 0.0;
    }
    if armor_reference_probe {
        return vanilla_armor_mitigation_for(
            base_damage,
            VANILLA_DIAMOND_CHESTPLATE_ARMOR_POINTS,
            VANILLA_DIAMOND_CHESTPLATE_TOUGHNESS,
        );
    }
    if armor_mitigation_probe {
        return DIAMOND_CHESTPLATE_MITIGATION;
    }
    0.0
}

fn vanilla_armor_mitigation_for(base_damage: f32, armor_points: f32, toughness: f32) -> f32 {
    let toughness_term = armor_points
        - base_damage
            / (toughness / VANILLA_ARMOR_TOUGHNESS_QUARTER_DIVISOR + VANILLA_ARMOR_TOUGHNESS_BASE);
    let min_reduction = armor_points / VANILLA_ARMOR_MIN_REDUCTION_DIVISOR;
    let reduction_points = toughness_term
        .max(min_reduction)
        .min(VANILLA_ARMOR_MAX_REDUCTION_POINTS);
    base_damage * reduction_points / VANILLA_COMBAT_ARMOR_REDUCTION_DENOMINATOR
}

fn vanilla_combat_reference_knockback_metric(knockback_velocity: [f32; 3]) -> f64 {
    f64::from(knockback_velocity[0]).hypot(f64::from(knockback_velocity[2]))
        / VANILLA_COMBAT_REFERENCE_KNOCKBACK_SCALE
}

fn vanilla_combat_reference_knockback_velocity_for(hit: bool) -> Option<[f32; 3]> {
    if hit {
        Some(VANILLA_COMBAT_REFERENCE_KNOCKBACK_VELOCITY)
    } else {
        None
    }
}

fn vanilla_combat_reference_damage_milestone(
    row: &str,
    attacker: &str,
    victim: &str,
    weapon: &str,
    armor_state: &str,
    pre_health: f32,
    post_health: f32,
    damage_delta: f32,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE vanilla_combat_reference_damage row={} backend={} \
         reference_oracle={} reference_version={} attacker_identity={} victim_identity={} \
         weapon={} armor_state={} pre_health={:.1} post_health={:.1} damage_delta={:.1} \
         damage_tolerance={:.1}",
        row,
        VANILLA_COMBAT_REFERENCE_BACKEND,
        VANILLA_COMBAT_REFERENCE_ORACLE,
        VANILLA_COMBAT_REFERENCE_VERSION,
        attacker,
        victim,
        weapon,
        armor_state,
        pre_health,
        post_health,
        damage_delta,
        VANILLA_COMBAT_REFERENCE_DAMAGE_TOLERANCE,
    )
}

fn vanilla_combat_reference_knockback_milestone(
    attacker: &str,
    victim: &str,
    knockback_metric: f64,
) -> String {
    format!(
        "MC-COMPAT-MILESTONE vanilla_combat_reference_knockback row={} backend={} \
         reference_oracle={} reference_version={} attacker_identity={} victim_identity={} \
         knockback_metric={:.2} knockback_tolerance={:.2}",
        vanilla_combat_reference_row(),
        VANILLA_COMBAT_REFERENCE_BACKEND,
        VANILLA_COMBAT_REFERENCE_ORACLE,
        VANILLA_COMBAT_REFERENCE_VERSION,
        attacker,
        victim,
        knockback_metric,
        VANILLA_COMBAT_REFERENCE_KNOCKBACK_TOLERANCE,
    )
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
            "MC-COMPAT-MILESTONE projectile_use attacker={} victim={} hand={:?} sequence={} \
             damage={:.1} policy={} generation={} clamped={}",
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
    const TEST_REFERENCE_DAMAGE: f32 = 6.0;
    const TEST_REFERENCE_POST_HEALTH: f32 = 14.0;
    const TEST_ARMOR_REFERENCE_MITIGATION: f32 = 1.344;
    const TEST_ARMOR_REFERENCE_DAMAGE: f32 = 4.656;
    const TEST_FLOAT_TOLERANCE: f32 = 0.0001;
    const TEST_KNOCKBACK_X: f32 = 8.0;
    const TEST_KNOCKBACK_Y: f32 = 6.432;
    const TEST_KNOCKBACK_Z: f32 = 0.0;
    const TEST_NORMALIZED_KNOCKBACK: f64 = 0.40;
    const TEST_RED_SCORE: u32 = 2;
    const TEST_PRE_FINAL_RED_SCORE: u32 = 1;
    const TEST_FINAL_RED_SCORE: u32 = 2;
    const TEST_BLUE_SCORE: u32 = 0;
    const TEST_ACCEPTED_RACE_PLAYER: &str = "compatbota";
    const TEST_REJECTED_RACE_PLAYER: &str = "compatbotb";
    const TEST_CLIENT_ENTITY_ID: u32 = 1;
    const TEST_CLICK_STATE_ID: i32 = 7;

    fn test_inventory_stack(count: i8) -> ItemStack {
        ItemStack::new(INVENTORY_STACK_ITEM, count, None)
    }

    fn test_slot_change(slot: i16, stack: ItemStack) -> valence::inventory::SlotChange {
        valence::inventory::SlotChange { idx: slot, stack }
    }

    fn test_click_slot_event(
        slot_id: i16,
        button: i8,
        carried_item: ItemStack,
        slot_changes: Vec<valence::inventory::SlotChange>,
    ) -> ClickSlotEvent {
        ClickSlotEvent {
            client: Entity::from_raw(TEST_CLIENT_ENTITY_ID),
            window_id: INVENTORY_STACK_WINDOW_ID,
            state_id: TEST_CLICK_STATE_ID,
            slot_id,
            button,
            mode: ClickMode::Click,
            slot_changes,
            carried_item,
        }
    }

    fn test_drag_slot_event(
        slot_id: i16,
        button: i8,
        carried_item: ItemStack,
        slot_changes: Vec<valence::inventory::SlotChange>,
    ) -> ClickSlotEvent {
        ClickSlotEvent {
            client: Entity::from_raw(TEST_CLIENT_ENTITY_ID),
            window_id: INVENTORY_STACK_WINDOW_ID,
            state_id: TEST_CLICK_STATE_ID,
            slot_id,
            button,
            mode: ClickMode::Drag,
            slot_changes,
            carried_item,
        }
    }

    #[test]
    fn inventory_stack_split_merge_classifier_accepts_ordered_clicks() {
        let mut state = InventoryStackSplitMergeProbeState::default();
        let split_pickup = test_click_slot_event(
            INVENTORY_STACK_SOURCE_SLOT,
            INVENTORY_STACK_RIGHT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
            vec![test_slot_change(
                INVENTORY_STACK_SOURCE_SLOT,
                test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
            )],
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                COMPAT_ACTOR_USERNAME,
                &split_pickup,
                &state
            ),
            Some(InventoryStackSplitMergeServerAction::SplitPickup)
        );
        state.split_pickup_state_id = Some(TEST_CLICK_STATE_ID);

        let split_place = test_click_slot_event(
            INVENTORY_STACK_DESTINATION_SLOT,
            INVENTORY_STACK_LEFT_BUTTON,
            ItemStack::EMPTY,
            vec![test_slot_change(
                INVENTORY_STACK_DESTINATION_SLOT,
                test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
            )],
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(COMPAT_ACTOR_USERNAME, &split_place, &state),
            Some(InventoryStackSplitMergeServerAction::SplitPlace)
        );
        state.split_place_state_id = Some(TEST_CLICK_STATE_ID);

        let merge_pickup = test_click_slot_event(
            INVENTORY_STACK_DESTINATION_SLOT,
            INVENTORY_STACK_LEFT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
            vec![test_slot_change(
                INVENTORY_STACK_DESTINATION_SLOT,
                ItemStack::EMPTY,
            )],
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                COMPAT_ACTOR_USERNAME,
                &merge_pickup,
                &state
            ),
            Some(InventoryStackSplitMergeServerAction::MergePickup)
        );
        state.merge_pickup_state_id = Some(TEST_CLICK_STATE_ID);

        let merge_place = test_click_slot_event(
            INVENTORY_STACK_SOURCE_SLOT,
            INVENTORY_STACK_LEFT_BUTTON,
            ItemStack::EMPTY,
            vec![test_slot_change(
                INVENTORY_STACK_SOURCE_SLOT,
                test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            )],
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(COMPAT_ACTOR_USERNAME, &merge_place, &state),
            Some(InventoryStackSplitMergeServerAction::MergePlace)
        );
    }

    #[test]
    fn inventory_stack_split_merge_classifier_rejects_wrong_actor_or_count() {
        let state = InventoryStackSplitMergeProbeState::default();
        let split_pickup = test_click_slot_event(
            INVENTORY_STACK_SOURCE_SLOT,
            INVENTORY_STACK_RIGHT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
            vec![test_slot_change(
                INVENTORY_STACK_SOURCE_SLOT,
                test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
            )],
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event("other", &split_pickup, &state),
            None
        );

        let wrong_count = test_click_slot_event(
            INVENTORY_STACK_SOURCE_SLOT,
            INVENTORY_STACK_RIGHT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            vec![test_slot_change(
                INVENTORY_STACK_SOURCE_SLOT,
                test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            )],
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(COMPAT_ACTOR_USERNAME, &wrong_count, &state),
            None
        );

        let merge_before_split = test_click_slot_event(
            INVENTORY_STACK_DESTINATION_SLOT,
            INVENTORY_STACK_LEFT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
            vec![test_slot_change(
                INVENTORY_STACK_DESTINATION_SLOT,
                ItemStack::EMPTY,
            )],
        );
        assert_eq!(
            classify_inventory_stack_split_merge_event(
                COMPAT_ACTOR_USERNAME,
                &merge_before_split,
                &state
            ),
            None
        );
    }

    #[test]
    fn inventory_drag_transactions_classifier_accepts_ordered_drag() {
        let mut state = InventoryDragTransactionsProbeState::default();
        let pickup = test_click_slot_event(
            INVENTORY_STACK_SOURCE_SLOT,
            INVENTORY_STACK_LEFT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            vec![test_slot_change(
                INVENTORY_STACK_SOURCE_SLOT,
                ItemStack::EMPTY,
            )],
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(COMPAT_ACTOR_USERNAME, &pickup, &state),
            Some(InventoryDragTransactionsServerAction::PickupSource)
        );
        state.pickup_state_id = Some(TEST_CLICK_STATE_ID);

        let drag_start = test_drag_slot_event(
            INVENTORY_DRAG_OUTSIDE_SLOT,
            INVENTORY_DRAG_START_BUTTON,
            test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            Vec::new(),
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(COMPAT_ACTOR_USERNAME, &drag_start, &state),
            Some(InventoryDragTransactionsServerAction::DragStart)
        );
        state.drag_start_state_id = Some(TEST_CLICK_STATE_ID);

        let target_a = test_drag_slot_event(
            INVENTORY_DRAG_TARGET_SLOT_A,
            INVENTORY_DRAG_ADD_SLOT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            Vec::new(),
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(COMPAT_ACTOR_USERNAME, &target_a, &state),
            Some(InventoryDragTransactionsServerAction::AddTargetA)
        );
        state.target_a_state_id = Some(TEST_CLICK_STATE_ID);

        let target_b = test_drag_slot_event(
            INVENTORY_DRAG_TARGET_SLOT_B,
            INVENTORY_DRAG_ADD_SLOT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            Vec::new(),
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(COMPAT_ACTOR_USERNAME, &target_b, &state),
            Some(InventoryDragTransactionsServerAction::AddTargetB)
        );
        state.target_b_state_id = Some(TEST_CLICK_STATE_ID);

        let drag_end = test_drag_slot_event(
            INVENTORY_DRAG_OUTSIDE_SLOT,
            INVENTORY_DRAG_END_BUTTON,
            ItemStack::EMPTY,
            vec![
                test_slot_change(
                    INVENTORY_DRAG_TARGET_SLOT_A,
                    test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
                ),
                test_slot_change(
                    INVENTORY_DRAG_TARGET_SLOT_B,
                    test_inventory_stack(INVENTORY_STACK_HALF_COUNT),
                ),
            ],
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(COMPAT_ACTOR_USERNAME, &drag_end, &state),
            Some(InventoryDragTransactionsServerAction::DragEnd)
        );
    }

    #[test]
    fn inventory_drag_transactions_classifier_rejects_wrong_order_or_distribution() {
        let state = InventoryDragTransactionsProbeState::default();
        let target_before_pickup = test_drag_slot_event(
            INVENTORY_DRAG_TARGET_SLOT_A,
            INVENTORY_DRAG_ADD_SLOT_BUTTON,
            test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            Vec::new(),
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(
                COMPAT_ACTOR_USERNAME,
                &target_before_pickup,
                &state
            ),
            None
        );

        let mut ready_for_end = InventoryDragTransactionsProbeState::default();
        ready_for_end.pickup_state_id = Some(TEST_CLICK_STATE_ID);
        ready_for_end.drag_start_state_id = Some(TEST_CLICK_STATE_ID);
        ready_for_end.target_a_state_id = Some(TEST_CLICK_STATE_ID);
        ready_for_end.target_b_state_id = Some(TEST_CLICK_STATE_ID);
        let wrong_distribution = test_drag_slot_event(
            INVENTORY_DRAG_OUTSIDE_SLOT,
            INVENTORY_DRAG_END_BUTTON,
            ItemStack::EMPTY,
            vec![test_slot_change(
                INVENTORY_DRAG_TARGET_SLOT_A,
                test_inventory_stack(INVENTORY_STACK_FULL_COUNT),
            )],
        );
        assert_eq!(
            classify_inventory_drag_transactions_event(
                COMPAT_ACTOR_USERNAME,
                &wrong_distribution,
                &ready_for_end
            ),
            None
        );
    }

    #[test]
    fn vanilla_combat_reference_milestones_record_normalized_metrics() {
        let damage = vanilla_combat_reference_damage_milestone(
            VANILLA_COMBAT_REFERENCE_ROW,
            VANILLA_COMBAT_REFERENCE_ATTACKER,
            VANILLA_COMBAT_REFERENCE_VICTIM,
            vanilla_combat_reference_weapon_name(ItemKind::IronSword),
            vanilla_combat_reference_armor_state(ItemKind::Air),
            TEST_HEALTH_BEFORE,
            TEST_REFERENCE_POST_HEALTH,
            TEST_REFERENCE_DAMAGE,
        );
        let knockback = vanilla_combat_reference_knockback_milestone(
            VANILLA_COMBAT_REFERENCE_ATTACKER,
            VANILLA_COMBAT_REFERENCE_VICTIM,
            vanilla_combat_reference_knockback_metric([
                TEST_KNOCKBACK_X,
                TEST_KNOCKBACK_Y,
                TEST_KNOCKBACK_Z,
            ]),
        );

        assert!(
            damage.contains("vanilla_combat_reference_damage"),
            "{damage}"
        );
        assert!(damage.contains("backend=valence"), "{damage}");
        assert!(damage.contains("weapon=iron_sword"), "{damage}");
        assert!(damage.contains("armor_state=none"), "{damage}");
        assert!(damage.contains("damage_delta=6.0"), "{damage}");
        assert!(
            knockback.contains("vanilla_combat_reference_knockback"),
            "{knockback}"
        );
        assert!(knockback.contains("knockback_metric=0.40"), "{knockback}");
        assert_eq!(
            vanilla_combat_reference_knockback_metric([
                TEST_KNOCKBACK_X,
                TEST_KNOCKBACK_Y,
                TEST_KNOCKBACK_Z,
            ]),
            TEST_NORMALIZED_KNOCKBACK
        );
    }

    #[test]
    fn vanilla_combat_armor_reference_helpers_apply_bounded_chestplate_formula() {
        let mitigation = combat_armor_mitigation_for(
            true,
            false,
            ItemKind::DiamondChestplate,
            TEST_REFERENCE_DAMAGE,
        );
        assert!((mitigation - TEST_ARMOR_REFERENCE_MITIGATION).abs() < TEST_FLOAT_TOLERANCE);
        let final_damage = TEST_REFERENCE_DAMAGE - mitigation;
        assert!((final_damage - TEST_ARMOR_REFERENCE_DAMAGE).abs() < TEST_FLOAT_TOLERANCE);
        assert_eq!(
            combat_armor_mitigation_for(true, false, ItemKind::Air, TEST_REFERENCE_DAMAGE),
            0.0
        );
        assert_eq!(
            combat_armor_mitigation_for(
                false,
                false,
                ItemKind::DiamondChestplate,
                TEST_REFERENCE_DAMAGE
            ),
            0.0
        );
        assert_eq!(
            combat_armor_mitigation_for(
                false,
                true,
                ItemKind::DiamondChestplate,
                TEST_REFERENCE_DAMAGE
            ),
            DIAMOND_CHESTPLATE_MITIGATION
        );

        let damage = vanilla_combat_reference_damage_milestone(
            VANILLA_COMBAT_ARMOR_REFERENCE_ROW,
            VANILLA_COMBAT_REFERENCE_ATTACKER,
            VANILLA_COMBAT_REFERENCE_VICTIM,
            vanilla_combat_reference_weapon_name(ItemKind::IronSword),
            vanilla_combat_reference_armor_state(ItemKind::DiamondChestplate),
            TEST_HEALTH_BEFORE,
            TEST_HEALTH_BEFORE - TEST_ARMOR_REFERENCE_DAMAGE,
            TEST_ARMOR_REFERENCE_DAMAGE,
        );
        assert!(
            damage.contains("row=vanilla-combat-armor-reference-parity"),
            "{damage}"
        );
        assert!(
            damage.contains("armor_state=diamond_chestplate"),
            "{damage}"
        );
        assert!(damage.contains("post_health=15.3"), "{damage}");
        assert!(damage.contains("damage_delta=4.7"), "{damage}");
    }

    #[test]
    fn vanilla_combat_reference_helpers_fail_closed_for_unbounded_inputs() {
        assert!(vanilla_combat_reference_probe_hit_for(
            true,
            VANILLA_COMBAT_REFERENCE_ATTACKER,
            VANILLA_COMBAT_REFERENCE_VICTIM
        ));
        assert!(!vanilla_combat_reference_probe_hit_for(
            false,
            VANILLA_COMBAT_REFERENCE_ATTACKER,
            VANILLA_COMBAT_REFERENCE_VICTIM
        ));
        assert!(!vanilla_combat_reference_probe_hit_for(
            true,
            VANILLA_COMBAT_REFERENCE_VICTIM,
            VANILLA_COMBAT_REFERENCE_ATTACKER
        ));
        let attacker_assignment =
            vanilla_combat_reference_assignment_for(true, VANILLA_COMBAT_REFERENCE_ATTACKER)
                .expect("attacker assignment exists");
        assert_eq!(attacker_assignment.team, Team::Red);
        assert_eq!(
            attacker_assignment.position.x,
            VANILLA_COMBAT_REFERENCE_ATTACKER_X
        );
        assert_eq!(
            attacker_assignment.yaw,
            VANILLA_COMBAT_REFERENCE_ATTACKER_YAW
        );
        let victim_assignment =
            vanilla_combat_reference_assignment_for(true, VANILLA_COMBAT_REFERENCE_VICTIM)
                .expect("victim assignment exists");
        assert_eq!(victim_assignment.team, Team::Blue);
        assert_eq!(
            victim_assignment.position.x,
            VANILLA_COMBAT_REFERENCE_VICTIM_X
        );
        assert_eq!(victim_assignment.yaw, VANILLA_COMBAT_REFERENCE_VICTIM_YAW);
        assert!(
            vanilla_combat_reference_assignment_for(false, VANILLA_COMBAT_REFERENCE_ATTACKER)
                .is_none()
        );
        assert!(vanilla_combat_reference_assignment_for(true, "compatbotc").is_none());
        let reference_velocity = vanilla_combat_reference_knockback_velocity_for(true)
            .expect("reference velocity exists");
        assert_eq!(
            reference_velocity,
            VANILLA_COMBAT_REFERENCE_KNOCKBACK_VELOCITY
        );
        assert_eq!(
            vanilla_combat_reference_knockback_metric(reference_velocity),
            f64::from(VANILLA_COMBAT_REFERENCE_KNOCKBACK_X)
        );
        assert!(vanilla_combat_reference_knockback_velocity_for(false).is_none());
        assert_eq!(
            vanilla_combat_reference_weapon_name(ItemKind::Bow),
            VANILLA_COMBAT_REFERENCE_WEAPON_OTHER
        );
        assert_eq!(
            vanilla_combat_reference_armor_state(ItemKind::GoldenChestplate),
            VANILLA_COMBAT_REFERENCE_ARMOR_OTHER
        );
        assert_eq!(
            vanilla_combat_reference_armor_state(ItemKind::Air),
            VANILLA_COMBAT_REFERENCE_ARMOR_NONE
        );
    }

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
    fn invalid_flag_return_drop_milestone_records_no_state_mutation_or_score() {
        let mut score = Score::default();
        score.scores.insert(Team::Red, TEST_RED_SCORE);
        let flag_manager = FlagManager {
            red: None,
            blue: None,
        };
        let milestone = invalid_flag_return_drop_rejection_milestone(
            "compatbot",
            Team::Red,
            Team::Red,
            flag_presence_state(&flag_manager, Team::Red),
            flag_presence_state(&flag_manager, Team::Red),
            score_for_team(&score, Team::Red),
            score_for_team(&score, Team::Blue),
        );

        assert!(
            milestone.contains("invalid_flag_return_drop_rejected"),
            "{milestone}"
        );
        assert!(milestone.contains("username=compatbot"), "{milestone}");
        assert!(milestone.contains("actor_team=Red"), "{milestone}");
        assert!(milestone.contains("flag_team=Red"), "{milestone}");
        assert!(milestone.contains("pre_state=at_base"), "{milestone}");
        assert!(milestone.contains("post_state=at_base"), "{milestone}");
        assert!(milestone.contains("red_score=2"), "{milestone}");
        assert!(milestone.contains("blue_score=0"), "{milestone}");
        assert!(
            milestone.contains("outcome=no_flag_state_mutation_no_score"),
            "{milestone}"
        );
    }

    #[test]
    fn invalid_opponent_base_return_drop_milestone_records_no_state_mutation_or_score() {
        let mut score = Score::default();
        score.scores.insert(Team::Red, TEST_RED_SCORE);
        let flag_manager = FlagManager {
            red: None,
            blue: None,
        };
        let milestone = invalid_opponent_base_return_drop_rejection_milestone(
            "compatbot",
            Team::Red,
            Team::Blue,
            flag_presence_state(&flag_manager, Team::Blue),
            flag_presence_state(&flag_manager, Team::Blue),
            score_for_team(&score, Team::Red),
            score_for_team(&score, Team::Blue),
        );

        assert!(
            milestone.contains("invalid_opponent_base_return_drop_rejected"),
            "{milestone}"
        );
        assert!(milestone.contains("username=compatbot"), "{milestone}");
        assert!(milestone.contains("actor_team=Red"), "{milestone}");
        assert!(milestone.contains("flag_team=Blue"), "{milestone}");
        assert!(milestone.contains("pre_state=at_base"), "{milestone}");
        assert!(milestone.contains("post_state=at_base"), "{milestone}");
        assert!(milestone.contains("red_score=2"), "{milestone}");
        assert!(milestone.contains("blue_score=0"), "{milestone}");
        assert!(
            milestone.contains("outcome=no_flag_state_mutation_no_score"),
            "{milestone}"
        );
    }

    #[test]
    fn ctf_race_milestones_record_one_accept_one_reject_and_final_score() {
        let flag_manager = FlagManager {
            red: None,
            blue: None,
        };
        let accepted = ctf_race_accepted_transition_milestone(
            TEST_ACCEPTED_RACE_PLAYER,
            Team::Red,
            Team::Blue,
        );
        let rejected = ctf_race_rejected_transition_milestone(
            TEST_REJECTED_RACE_PLAYER,
            Team::Red,
            Team::Blue,
        );
        let final_state = ctf_race_final_state_milestone(
            TEST_ACCEPTED_RACE_PLAYER,
            TEST_REJECTED_RACE_PLAYER,
            TEST_ACCEPTED_RACE_PLAYER,
            Team::Red,
            Team::Blue,
            CTF_RACE_FINAL_RED_SCORE,
            CTF_RACE_FINAL_BLUE_SCORE,
            &flag_manager,
        )
        .expect("expected race final state milestone");

        assert!(
            accepted.contains("ctf_race_accepted_transition"),
            "{accepted}"
        );
        assert!(accepted.contains("username=compatbota"), "{accepted}");
        assert!(accepted.contains("player_team=Red"), "{accepted}");
        assert!(accepted.contains("flag_team=Blue"), "{accepted}");
        assert!(accepted.contains("transition=pickup"), "{accepted}");
        assert!(
            rejected.contains("ctf_race_rejected_transition"),
            "{rejected}"
        );
        assert!(rejected.contains("username=compatbotb"), "{rejected}");
        assert!(
            rejected.contains("transition=duplicate_pickup"),
            "{rejected}"
        );
        assert!(rejected.contains("reason=flag_already_held"), "{rejected}");
        assert!(
            final_state.contains("ctf_race_final_state"),
            "{final_state}"
        );
        assert!(
            final_state.contains("accepted_username=compatbota"),
            "{final_state}"
        );
        assert!(
            final_state.contains("rejected_username=compatbotb"),
            "{final_state}"
        );
        assert!(final_state.contains("red_score=1"), "{final_state}");
        assert!(final_state.contains("blue_score=0"), "{final_state}");
    }

    #[test]
    fn ctf_race_final_state_rejects_double_score_or_held_flag() {
        let at_base = FlagManager {
            red: None,
            blue: None,
        };
        let held_blue = FlagManager {
            red: None,
            blue: Some(Entity::from_raw(CTF_RACE_FINAL_RED_SCORE)),
        };

        let double_score = ctf_race_final_state_milestone(
            TEST_ACCEPTED_RACE_PLAYER,
            TEST_REJECTED_RACE_PLAYER,
            TEST_ACCEPTED_RACE_PLAYER,
            Team::Red,
            Team::Blue,
            TEST_FINAL_RED_SCORE,
            CTF_RACE_FINAL_BLUE_SCORE,
            &at_base,
        );
        let flag_still_held = ctf_race_final_state_milestone(
            TEST_ACCEPTED_RACE_PLAYER,
            TEST_REJECTED_RACE_PLAYER,
            TEST_ACCEPTED_RACE_PLAYER,
            Team::Red,
            Team::Blue,
            CTF_RACE_FINAL_RED_SCORE,
            CTF_RACE_FINAL_BLUE_SCORE,
            &held_blue,
        );

        assert!(double_score.is_none());
        assert!(flag_still_held.is_none());
    }

    #[test]
    fn ctf_spawn_team_reset_milestones_record_balanced_assignments_and_reset() {
        let mut state = CtfSpawnTeamResetProbeState::default();
        state.record_assignment("compatbota", Team::Red);
        let red = ctf_spawn_team_assignment_milestone(
            "compatbota",
            Team::Red,
            state.red_count,
            state.blue_count,
        );
        state.record_assignment("compatbotb", Team::Blue);
        let blue = ctf_spawn_team_assignment_milestone(
            "compatbotb",
            Team::Blue,
            state.red_count,
            state.blue_count,
        );
        let balance = ctf_spawn_team_balance_milestone(&state).expect("balance milestone");
        let mut score = Score::default();
        score
            .scores
            .insert(Team::Red, CTF_SPAWN_RESET_FINAL_RED_SCORE);
        score
            .scores
            .insert(Team::Blue, CTF_SPAWN_RESET_FINAL_BLUE_SCORE);
        let reset = ctf_spawn_resource_reset_state_milestone(
            &state,
            "compatbota",
            Team::Red,
            Team::Blue,
            &score,
        )
        .expect("reset milestone");

        assert!(red.contains("ctf_spawn_team_assignment"), "{red}");
        assert!(red.contains("username=compatbota"), "{red}");
        assert!(red.contains("team=Red"), "{red}");
        assert!(red.contains("slot37=RedWool:64"), "{red}");
        assert!(blue.contains("username=compatbotb"), "{blue}");
        assert!(blue.contains("team=Blue"), "{blue}");
        assert!(blue.contains("slot37=BlueWool:64"), "{blue}");
        assert!(balance.contains("red_count=1 blue_count=1"), "{balance}");
        assert!(
            balance.contains("selected_teams=compatbota:Red,compatbotb:Blue"),
            "{balance}"
        );
        assert!(reset.contains("ctf_spawn_resource_reset_state"), "{reset}");
        assert!(reset.contains("red_score=1 blue_score=0"), "{reset}");
        assert!(
            reset.contains("reset_state=scoreboard_flags_and_resources_coherent"),
            "{reset}"
        );
    }

    #[test]
    fn ctf_spawn_team_reset_rejects_imbalance_or_wrong_score() {
        let mut imbalanced = CtfSpawnTeamResetProbeState::default();
        imbalanced.record_assignment("compatbota", Team::Red);
        assert!(ctf_spawn_team_balance_milestone(&imbalanced).is_none());

        let mut balanced = CtfSpawnTeamResetProbeState::default();
        balanced.record_assignment("compatbota", Team::Red);
        balanced.record_assignment("compatbotb", Team::Blue);
        let mut wrong_score = Score::default();
        wrong_score
            .scores
            .insert(Team::Red, CTF_SPAWN_RESET_FINAL_RED_SCORE + 1);
        wrong_score
            .scores
            .insert(Team::Blue, CTF_SPAWN_RESET_FINAL_BLUE_SCORE);
        assert!(ctf_spawn_resource_reset_state_milestone(
            &balanced,
            "compatbota",
            Team::Red,
            Team::Blue,
            &wrong_score,
        )
        .is_none());
        assert!(ctf_spawn_reset_should_defer_team_assignment(
            "compatbotb",
            Team::Red
        ));
        assert!(!ctf_spawn_reset_should_defer_team_assignment(
            "compatbotb",
            Team::Blue
        ));
        assert!(!ctf_spawn_reset_should_defer_team_assignment(
            "compatbota",
            Team::Red
        ));
    }

    #[test]
    fn score_limit_milestones_record_pre_state_final_capture_and_win_once() {
        let mut score = Score::default();
        score.scores.insert(Team::Red, TEST_PRE_FINAL_RED_SCORE);
        score.scores.insert(Team::Blue, TEST_BLUE_SCORE);
        let pre_state = score_limit_pre_state_milestone(&score);
        let final_capture = score_limit_final_capture_milestone(
            "compatbot",
            Team::Red,
            Team::Blue,
            TEST_PRE_FINAL_RED_SCORE,
            TEST_BLUE_SCORE,
            TEST_FINAL_RED_SCORE,
            TEST_BLUE_SCORE,
        );
        score.scores.insert(Team::Red, TEST_FINAL_RED_SCORE);
        let win = score_limit_win_condition_milestone(
            "compatbot",
            Team::Red,
            &score,
            CTF_SCORE_LIMIT_FIRST_WIN_EMISSION,
        );

        assert!(pre_state.contains("score_limit_pre_state"), "{pre_state}");
        assert!(pre_state.contains("score_limit=2"), "{pre_state}");
        assert!(pre_state.contains("red_score=1"), "{pre_state}");
        assert!(
            final_capture.contains("score_limit_final_capture"),
            "{final_capture}"
        );
        assert!(
            final_capture.contains("capture_team=Red"),
            "{final_capture}"
        );
        assert!(
            final_capture.contains("carried_flag=Blue"),
            "{final_capture}"
        );
        assert!(
            final_capture.contains("red_score_before=1"),
            "{final_capture}"
        );
        assert!(
            final_capture.contains("red_score_after=2"),
            "{final_capture}"
        );
        assert!(win.contains("score_limit_win_condition"), "{win}");
        assert!(win.contains("winning_team=Red"), "{win}");
        assert!(win.contains("end_state=winner_declared"), "{win}");
        assert!(win.contains("win_emissions=1"), "{win}");
        assert!(win.contains("duplicate_win=false"), "{win}");
        assert!(win.contains("post_win_score_delta=0"), "{win}");
    }

    #[test]
    fn score_limit_forbidden_milestones_are_named_for_duplicate_win_and_late_score_mutation() {
        let duplicate = score_limit_duplicate_win_milestone("compatbot", Team::Red);
        let mutation = score_limit_post_win_score_mutation_milestone("compatbot", Team::Red);

        assert!(
            duplicate.contains("score_limit_duplicate_win"),
            "{duplicate}"
        );
        assert!(
            duplicate.contains("outcome=forbidden_duplicate_win"),
            "{duplicate}"
        );
        assert!(
            mutation.contains("score_limit_post_win_score_mutation"),
            "{mutation}"
        );
        assert!(
            mutation.contains("outcome=forbidden_score_after_win"),
            "{mutation}"
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
