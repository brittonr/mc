mod runtime_config;
#[allow(dead_code)]
mod scenario_manifest_generated;

use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const DEFAULT_VALENCE_REV: &str = "8ad9c85";
const DEFAULT_VALENCE_EXAMPLE: &str = "terrain";
const DEFAULT_SERVER_VERSION: &str = "1.18.2";
const DEFAULT_SERVER_PROTOCOL: u32 = 758;
const DEFAULT_CLIENT_USERNAME: &str = "compatbot";
const DEFAULT_CLIENT_TIMEOUT_SECS: u64 = 20;
const MULTI_CLIENT_LOAD_PEER_TIMEOUT_SECS: u64 = 10;
const PAPER_PLUGIN_CONTAINER_DIR: &str = "/plugins";
const PAPER_VIEW_DISTANCE: u32 = 2;
const PAPER_SIMULATION_DISTANCE: u32 = 2;
const SAFETY_MAX_LOCAL_CLIENTS: usize = 2;
const SAFETY_MAX_DURATION_SECS: u64 = 600;
const SAFETY_SINGLE_SESSION_COUNT: usize = 1;
const SAFETY_RECONNECT_SESSION_COUNT: usize = 2;
const RECONNECT_SEQUENCE_SESSION_COUNT: usize = 2;
const RECONNECT_SEQUENCE_PAUSE_SECS: u64 = 4;
const SAFETY_ZERO_VALUE: &str = "0";
const SAFETY_OWNED_LOCAL_SCOPE: &str = "owned-local-loopback";
const PINNED_PROJECTILE_DAMAGE_VALENCE_REV: &str = "e5d18ad04010d92881267ac1ea43922ae91821f5";
const PROJECTILE_DAMAGE_ATTACKER_SUFFIX: &str = "a";
const PROJECTILE_DAMAGE_VICTIM_SUFFIX: &str = "b";
const PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE: &str = "projectile_probe_use_item_sent";
const PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE: &str = "projectile_probe_swing_sent";
const PROJECTILE_DAMAGE_CLIENT_HEALTH_NEEDLE: &str = "update_health health=17.0";
const PROJECTILE_DAMAGE_SERVER_USE_NEEDLE: &str = "MC-COMPAT-MILESTONE projectile_use";
const PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE: &str = "MC-COMPAT-MILESTONE projectile_hit";
const PROJECTILE_DAMAGE_SEQUENCE_NEEDLE: &str = "sequence=303";
const PROJECTILE_DAMAGE_AMOUNT_NEEDLE: &str = "damage=3.0";
const DEFAULT_ARROW_DAMAGE: f64 = 3.0;
const DEFAULT_ARROW_VELOCITY_MULTIPLIER: f64 = 1.0;
const DEFAULT_ARROW_MAX_DAMAGE: f64 = 10.0;
const GIT_REV_DRY_RUN_PLACEHOLDER: &str = "dry-run";
const GIT_STATUS_CLEAN: &str = "clean";
const GIT_STATUS_DIRTY: &str = "dirty";
const GIT_STATUS_DRY_RUN: &str = "dry-run";
const GIT_STATUS_UNAVAILABLE: &str = "unavailable";
const GIT_STATUS_PORCELAIN_FLAG: &str = "--porcelain";
const PROJECTILE_DAMAGE_CONTEXT_VELOCITY: f64 = 0.0;
const PROJECTILE_DAMAGE_CONTEXT_PULL_STRENGTH: f64 = 1.0;
const PROJECTILE_DAMAGE_VICTIM_START_HEALTH: f64 = 20.0;
const SURVIVAL_CHEST_CLIENT_OPEN_NEEDLE: &str = "survival_chest_open_seen window=1 position=8,64,0";
const SURVIVAL_CHEST_CLIENT_STORE_NEEDLE: &str =
    "survival_chest_store_sent window=1 slot=0 item=Dirt count=1";
const SURVIVAL_CHEST_CLIENT_CLOSE_NEEDLE: &str = "survival_chest_close_sent window=1";
const SURVIVAL_CHEST_CLIENT_RECONNECT_NEEDLE: &str = "survival_chest_reconnect_sent session=1";
const SURVIVAL_CHEST_CLIENT_REOPEN_NEEDLE: &str =
    "survival_chest_reopen_seen window=1 position=8,64,0";
const SURVIVAL_CHEST_CLIENT_PERSISTED_NEEDLE: &str =
    "survival_chest_persisted_seen window=1 slot=0 item=Dirt count=1";
const SURVIVAL_CHEST_SERVER_OPEN_NEEDLE: &str =
    "survival_chest_open username=compatbot position=8,64,0 window=1";
const SURVIVAL_CHEST_SERVER_STORE_NEEDLE: &str =
    "survival_chest_store username=compatbot window=1 slot=0 item=Dirt count=1";
const SURVIVAL_CHEST_SERVER_CLOSE_NEEDLE: &str = "survival_chest_close username=compatbot window=1";
const SURVIVAL_CHEST_SERVER_REOPEN_NEEDLE: &str =
    "survival_chest_reopen username=compatbot position=8,64,0 window=1";
const SURVIVAL_CHEST_SERVER_PERSISTED_NEEDLE: &str =
    "survival_chest_persisted username=compatbot slot=0 item=Dirt count=1";
const SURVIVAL_CHEST_FIXTURE_ENV: &str = "MC_COMPAT_SURVIVAL_CHEST_FIXTURE";
const SUPPORTED_SCENARIO_USAGE: &str = "smoke|valence-compat-bot-probe|flag-score-repeat|blue-flag-score|inventory-interaction|survival-break-place-pickup|survival-chest-persistence|combat-damage|combat-knockback|armor-equipment-mitigation|armor-loadout-enchantment-status-matrix|equipment-update-observation|equipment-slot-item-matrix-expansion|projectile-hit|projectile-damage-attribution|flag-carrier-death-return|reconnect-flag-state|reconnect-flag-score|multi-client-load-score|negative-inventory-stale-state|negative-inventory-invalid-click|negative-custom-payload|negative-reconnect-race|negative-ctf-wrong-score";
const DEFAULT_SUCCESS_PATTERN: &[&str] = &[
    "Detected server protocol version",
    "Dimension type:",
    "Received chat message",
];
const TRIAGE_MAX_TIMELINE_EVENTS: usize = 6;
const TRIAGE_MAX_EXCERPT_CHARS: usize = 160;
const TRIAGE_CONFIDENCE_HIGH: &str = "high";
const TRIAGE_CONFIDENCE_MEDIUM: &str = "medium";
const TRIAGE_CONFIDENCE_NONE: &str = "none";
const TRIAGE_REDACTED: &str = "[redacted]";
const TYPED_EVENT_PREFIX: &str = "MC-COMPAT-EVENT";
const TYPED_EVENT_SCHEMA_VERSION: u32 = 1;
const TYPED_EVENT_MIGRATION_FALLBACK: &str = "substring-fallback";
const TYPED_EVENT_MIGRATION_DERIVED_FROM_MILESTONES: &str = "derived-from-milestones";
const TYPED_EVENT_LOG_EXTENSION: &str = "typed-events.log";
const TYPED_EVENT_DEFAULT_SESSION_ID: &str = "mc_compat_session";
const TYPED_EVENT_MAX_FIELD_CHARS: usize = 128;
const TYPED_EVENT_SEQUENCE_INDEX_OFFSET: usize = 1;
const TYPED_EVENT_SINGLE_USERNAME_COUNT: usize = 1;
const NEGATIVE_LIVE_RAIL_MAX_CLIENTS: usize = 2;
const NEGATIVE_LIVE_RAIL_MIN_TIMEOUT_SECS: u64 = 1;
const NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME: &str = "containment_or_disconnect_without_promotion";
const NEGATIVE_LIVE_RAIL_OBSERVED_OUTCOME_CONTAINMENT: &str = "containment_observed";
const NEGATIVE_LIVE_RAIL_OUTCOME_SOURCE_PREFIX: &str = "client_milestone:";
const NEGATIVE_LIVE_RAIL_NON_CLAIMS: &[&str] = &[
    "broad_invalid_input_coverage",
    "adversarial_security",
    "public_server_safety",
    "production_readiness",
    "full_inventory_transaction_semantics",
    "broad_plugin_message_semantics",
    "full_ctf_correctness",
];
const NEGATIVE_LIVE_RAIL_EVIDENCE_FIELDS: &[&str] = &[
    "invalid_action",
    "expected_outcome",
    "target_scope",
    "planned_clients",
    "timeout_secs",
    "client_milestone",
    "server_forbidden_matches",
    "postcondition",
];
const ARMOR_MATRIX_ROW_ID: &str = "chest_diamond_none_none_melee";
const ARMOR_MATRIX_LOADOUT_ID: &str = "armor_loadout_chest_only";
const ARMOR_MATRIX_EQUIPMENT_SLOT: &str = "chest=DiamondChestplate";
const ARMOR_MATRIX_ENCHANTMENT_NONE: &str = "enchantment_none";
const ARMOR_MATRIX_STATUS_EFFECT_NONE: &str = "status_effect_none";
const ARMOR_MATRIX_ATTACK_TYPE_MELEE: &str = "melee";
const ARMOR_MATRIX_REFERENCE_RECEIPT_NONE: &str = "none";
const ARMOR_MATRIX_NON_CLAIMS: &[&str] = &[
    "all_armor_permutations",
    "all_enchantments",
    "all_status_effects",
    "exact_vanilla_balancing",
    "production_readiness",
    "full_combat_correctness",
];
const EQUIPMENT_MATRIX_ROW_ID: &str = "remote_main_hand_slot4_item829_count1_non_empty";
const EQUIPMENT_MATRIX_ACTOR: &str = "compatbotb";
const EQUIPMENT_MATRIX_OBSERVER: &str = "compatbota";
const EQUIPMENT_MATRIX_REMOTE_ENTITY_ID: &str = "4";
const EQUIPMENT_MATRIX_SEMANTIC_SLOT: &str = "main_hand_remote_entity";
const EQUIPMENT_MATRIX_WIRE_SLOT: &str = "slot4";
const EQUIPMENT_MATRIX_ITEM_ID: &str = "829";
const EQUIPMENT_MATRIX_ITEM_COUNT: &str = "1";
const EQUIPMENT_MATRIX_TRANSITION: &str = "non_empty_update";
const EQUIPMENT_MATRIX_UPDATE_ORDER: &str = "after_remote_spawn";
const EQUIPMENT_MATRIX_REFERENCE_RECEIPT_NONE: &str = "none";
const EQUIPMENT_MATRIX_NON_CLAIMS: &[&str] = &[
    "all_equipment_slots",
    "all_item_types",
    "all_transition_orders",
    "equipment_packet_permutations",
    "armor_mitigation",
    "enchantment_status_effects",
    "production_readiness",
    "full_equipment_semantics",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    DryRun,
    Run,
    RunMatrix,
    BuildClient,
    StatusOnly,
    HarnessStatus,
    Cleanup,
    Stop,
    CompareReceipts,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ServerBackend {
    Valence,
    Paper,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scenario {
    Smoke,
    CompatBotProbe,
    FlagScoreRepeat,
    BlueFlagScore,
    InventoryInteraction,
    SurvivalBreakPlacePickup,
    SurvivalChestPersistence,
    CombatDamage,
    CombatKnockback,
    ArmorEquipmentMitigation,
    ArmorLoadoutEnchantmentStatusMatrix,
    EquipmentUpdateObservation,
    EquipmentSlotItemMatrixExpansion,
    ProjectileHit,
    ProjectileDamageAttribution,
    FlagCarrierDeathReturn,
    ReconnectFlagState,
    ReconnectFlagScore,
    MultiClientLoadScore,
    NegativeInventoryStaleState,
    NegativeInventoryInvalidClick,
    NegativeCustomPayload,
    NegativeReconnectRace,
    NegativeCtfWrongScore,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScenarioEvidence {
    observed_milestones: Vec<&'static str>,
    missing_milestones: Vec<&'static str>,
    forbidden_matches: Vec<&'static str>,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ServerScenarioEvidence {
    observed_milestones: Vec<&'static str>,
    missing_milestones: Vec<&'static str>,
    forbidden_matches: Vec<&'static str>,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProjectileDamageCausalityEvidence {
    required_steps: Vec<&'static str>,
    observed_steps: Vec<&'static str>,
    missing_steps: Vec<&'static str>,
    order_violations: Vec<&'static str>,
    attacker_username: String,
    victim_username: String,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ArmorLoadoutEnchantmentStatusMatrixEvidence {
    selected: bool,
    row_id: &'static str,
    loadout_id: &'static str,
    equipment_slots: Vec<&'static str>,
    enchantments: Vec<&'static str>,
    status_effects: Vec<&'static str>,
    attack_type: &'static str,
    reference_required: bool,
    reference_receipt: &'static str,
    live_receipt: bool,
    promotion_ready: bool,
    required_client_milestones: Vec<&'static str>,
    observed_client_milestones: Vec<&'static str>,
    required_server_milestones: Vec<&'static str>,
    observed_server_milestones: Vec<&'static str>,
    non_claims: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EquipmentSlotItemMatrixExpansionEvidence {
    selected: bool,
    row_id: &'static str,
    actor_username: &'static str,
    observer_username: &'static str,
    remote_entity_id: &'static str,
    semantic_slot: &'static str,
    wire_slot: &'static str,
    item_id: &'static str,
    item_count: &'static str,
    transition_kind: &'static str,
    update_order: &'static str,
    reference_required: bool,
    reference_receipt: &'static str,
    live_receipt: bool,
    promotion_ready: bool,
    required_client_milestones: Vec<&'static str>,
    observed_client_milestones: Vec<&'static str>,
    required_server_milestones: Vec<&'static str>,
    observed_server_milestones: Vec<&'static str>,
    non_claims: Vec<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NegativeLiveRailEvidence {
    selected: bool,
    rail: Option<&'static str>,
    invalid_action: Option<&'static str>,
    expected_outcome: Option<&'static str>,
    observed_outcome: Option<&'static str>,
    observed_outcome_source: Option<String>,
    postcondition_milestone: Option<&'static str>,
    telemetry_present: bool,
    target_scope: &'static str,
    owned_local_target: bool,
    explicit_authorization: bool,
    public_target: bool,
    planned_clients: usize,
    max_clients: usize,
    timeout_secs: u64,
    missing_fields: Vec<&'static str>,
    bound_violations: Vec<&'static str>,
    preflight_passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NegativeLiveRailInputs {
    selected: bool,
    rail: Option<&'static str>,
    invalid_action: Option<&'static str>,
    expected_outcome: Option<&'static str>,
    observed_outcome: Option<&'static str>,
    observed_outcome_source: Option<String>,
    postcondition_milestone: Option<&'static str>,
    telemetry_required: bool,
    telemetry_present: bool,
    target_scope: &'static str,
    explicit_authorization: bool,
    public_target: bool,
    planned_clients: usize,
    max_clients: usize,
    timeout_secs: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GitRevisionEvidence {
    requested_rev: Option<String>,
    resolved_rev: Option<String>,
    status: &'static str,
    dirty: bool,
    diagnostics: Vec<String>,
}

impl GitRevisionEvidence {
    fn dry_run(requested_rev: Option<String>) -> Self {
        Self {
            requested_rev,
            resolved_rev: Some(GIT_REV_DRY_RUN_PLACEHOLDER.to_string()),
            status: GIT_STATUS_DRY_RUN,
            dirty: false,
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ChildRevisionEvidence {
    client: GitRevisionEvidence,
    valence: GitRevisionEvidence,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LoadNetworkSafetyInputs {
    target_scope: &'static str,
    owned_local_target: bool,
    explicit_authorization: bool,
    public_target: bool,
    planned_clients: usize,
    max_clients: usize,
    duration_secs: u64,
    max_duration_secs: u64,
    reconnect_sessions: usize,
    latency_ms: String,
    jitter_ms: String,
    loss_percent: String,
    telemetry_present: bool,
    live_receipt: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LoadNetworkSafetyEvidence {
    target_scope: &'static str,
    owned_local_target: bool,
    explicit_authorization: bool,
    public_target: bool,
    authorized: bool,
    planned_clients: usize,
    max_clients: usize,
    duration_secs: u64,
    max_duration_secs: u64,
    reconnect_sessions: usize,
    latency_ms: String,
    jitter_ms: String,
    loss_percent: String,
    telemetry_present: bool,
    live_receipt: bool,
    missing_fields: Vec<&'static str>,
    bound_violations: Vec<&'static str>,
    preflight_passed: bool,
    promotion_ready: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ClientLogSlice<'a> {
    username: &'a str,
    output: &'a str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct EnrichedTriage {
    last_client_event: Option<String>,
    last_server_event: Option<String>,
    correlation_ids: Vec<String>,
    timeline_excerpt: Vec<String>,
    boundary_confidence: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TypedEvent {
    schema_version: u32,
    source: String,
    scenario: String,
    session: String,
    username: Option<String>,
    sequence: u64,
    kind: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TypedEventGraphEvaluation {
    observed_events: Vec<String>,
    missing_events: Vec<String>,
    forbidden_events: Vec<String>,
    order_violations: Vec<String>,
    passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TypedEventOracleArtifact {
    event_log_path: PathBuf,
    timeline_blake3: String,
    event_count: usize,
    contributes_to_pass_fail: bool,
}

#[derive(Debug, Clone)]
struct Config {
    root: PathBuf,
    client_dir: PathBuf,
    valence_repo: PathBuf,
    valence_rev: String,
    valence_worktree: PathBuf,
    valence_example: String,
    valence_log: PathBuf,
    valence_target_dir: PathBuf,
    valence_pid_file: PathBuf,
    server_backend: ServerBackend,
    target_dir: PathBuf,
    server_name: String,
    server_version: String,
    server_protocol: u32,
    server_port: u16,
    client_username: String,
    docker_image: String,
    paper_plugin_jar: Option<PathBuf>,
    mode: Mode,
    keep_server: bool,
    client_timeout: Duration,
    client_success_needles: Vec<String>,
    scenario: Scenario,
    expected_status_description: Option<String>,
    expected_status_version_name: Option<String>,
    expected_status_sample: Vec<String>,
    packet_capture_summary: bool,
    proxy_route: Option<String>,
    proxy_forwarding_mode: Option<String>,
    receipt_path: Option<PathBuf>,
    receipt_dir: Option<PathBuf>,
    compare_receipts: Option<(PathBuf, PathBuf)>,
    config_path: Option<PathBuf>,
    steel_config_path: Option<PathBuf>,
    matrix_dry_run: bool,
    cleanup_apply: bool,
    negative_public_target: bool,
    negative_external_authorized: bool,
    arrow_damage_policy: runtime_config::ArrowDamagePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClientRunEvidence {
    log_path: Option<PathBuf>,
    log_paths: Vec<PathBuf>,
    usernames: Vec<String>,
    exit_code: Option<i32>,
    classification: &'static str,
    matched_success_pattern: Option<String>,
    scenario: Option<ScenarioEvidence>,
    server_scenario: Option<ServerScenarioEvidence>,
    projectile_damage_causality: Option<ProjectileDamageCausalityEvidence>,
}

struct ManagedServer {
    child: Option<Child>,
    pid_file: PathBuf,
    paper_container: Option<String>,
    keep: bool,
}

impl Drop for ManagedServer {
    fn drop(&mut self) {
        if self.keep {
            return;
        }
        if let Some(mut child) = self.child.take() {
            eprintln!(
                "[mc-compat] stopping managed Valence server process {}",
                child.id()
            );
            let _ = child.kill();
            let _ = child.wait();
            let _ = fs::remove_file(&self.pid_file);
        }
        if let Some(container) = self.paper_container.take() {
            eprintln!("[mc-compat] stopping managed Paper container {container}");
            let _ = Command::new("docker")
                .arg("rm")
                .arg("-f")
                .arg(container)
                .status();
        }
    }
}

fn main() -> ExitCode {
    match real_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("[mc-compat] error: {err}");
            ExitCode::from(1)
        }
    }
}

fn real_main() -> Result<(), String> {
    let cfg = Config::from_env_and_args()?;
    let result = execute(&cfg);
    if cfg.receipt_path.is_some() {
        if let Err(receipt_err) = write_smoke_receipt(&cfg, result.as_ref()) {
            return match result {
                Ok(_) => Err(receipt_err),
                Err(err) => Err(format!(
                    "{err}; additionally failed to write receipt: {receipt_err}"
                )),
            };
        }
    }
    result.map(|_| ())
}

fn execute(cfg: &Config) -> Result<Option<ClientRunEvidence>, String> {
    validate_projectile_damage_dependency(cfg)?;
    validate_load_network_safety_preflight(cfg)?;
    validate_negative_live_rail_preflight(cfg)?;
    if matches!(cfg.mode, Mode::DryRun | Mode::Run | Mode::BuildClient) {
        ensure_client_dir_ready(cfg)?;
    }
    if cfg.server_backend == ServerBackend::Valence && matches!(cfg.mode, Mode::DryRun | Mode::Run)
    {
        ensure_valence_repo_ready(cfg)?;
    }
    match cfg.mode {
        Mode::DryRun => {
            log(format_args!(
                "plan: build client, start {:?} server, wait for protocol {}, run client under isolated Xvfb/X11",
                cfg.server_backend, cfg.server_protocol
            ));
            build_client(&cfg)?;
            if cfg.server_backend == ServerBackend::Paper {
                log(format_args!(
                    "server start will set EULA=TRUE using recorded user acceptance"
                ));
            }
            let _server = start_server(cfg)?;
            probe_status(cfg)?;
            let client = run_client(cfg)?;
            Ok(Some(client))
        }
        Mode::BuildClient => {
            build_client(cfg)?;
            Ok(None)
        }
        Mode::StatusOnly => {
            probe_status(cfg)?;
            Ok(None)
        }
        Mode::HarnessStatus => {
            print_harness_status(cfg)?;
            Ok(None)
        }
        Mode::Cleanup => {
            cleanup_harness_state(cfg)?;
            Ok(None)
        }
        Mode::Stop => {
            stop_server(cfg)?;
            Ok(None)
        }
        Mode::CompareReceipts => {
            compare_receipts(cfg)?;
            Ok(None)
        }
        Mode::RunMatrix => {
            run_matrix(cfg)?;
            Ok(None)
        }
        Mode::Run => {
            build_client(cfg)?;
            let _server = start_server(cfg)?;
            probe_status(cfg)?;
            let client = run_client(cfg)?;
            Ok(Some(client))
        }
    }
}

fn validate_projectile_damage_dependency(cfg: &Config) -> Result<(), String> {
    if cfg.server_backend != ServerBackend::Valence
        || cfg.scenario != Scenario::ProjectileDamageAttribution
        || !matches!(cfg.mode, Mode::DryRun | Mode::Run)
    {
        return Ok(());
    }
    if cfg.valence_rev == PINNED_PROJECTILE_DAMAGE_VALENCE_REV {
        return Ok(());
    }
    Err(format!(
        "projectile-damage-attribution requires pinned Valence revision {PINNED_PROJECTILE_DAMAGE_VALENCE_REV}; got {}. Do not use VALENCE_REV=HEAD for promoted evidence.",
        cfg.valence_rev
    ))
}

fn validate_load_network_safety_preflight(cfg: &Config) -> Result<(), String> {
    if !matches!(cfg.mode, Mode::DryRun | Mode::Run | Mode::RunMatrix) {
        return Ok(());
    }
    let evidence = evaluate_load_network_safety(load_network_safety_inputs(cfg, false, false));
    if evidence.preflight_passed {
        return Ok(());
    }
    Err(format!(
        "load/network safety preflight failed: missing={:?} bound_violations={:?}",
        evidence.missing_fields, evidence.bound_violations
    ))
}

fn load_network_safety_inputs(
    cfg: &Config,
    telemetry_present: bool,
    live_receipt: bool,
) -> LoadNetworkSafetyInputs {
    let explicit_authorization = env::var("MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED")
        .map(|value| value == "1")
        .unwrap_or(false);
    let public_target = env::var("MC_COMPAT_PUBLIC_TARGET")
        .map(|value| value == "1")
        .unwrap_or(false);
    LoadNetworkSafetyInputs {
        target_scope: SAFETY_OWNED_LOCAL_SCOPE,
        owned_local_target: !public_target,
        explicit_authorization,
        public_target,
        planned_clients: planned_client_usernames(cfg).len(),
        max_clients: SAFETY_MAX_LOCAL_CLIENTS,
        duration_secs: cfg.client_timeout.as_secs(),
        max_duration_secs: SAFETY_MAX_DURATION_SECS,
        reconnect_sessions: safety_reconnect_sessions(cfg.scenario),
        latency_ms: env::var("MC_COMPAT_LATENCY_MS")
            .unwrap_or_else(|_| SAFETY_ZERO_VALUE.to_string()),
        jitter_ms: env::var("MC_COMPAT_JITTER_MS")
            .unwrap_or_else(|_| SAFETY_ZERO_VALUE.to_string()),
        loss_percent: env::var("MC_COMPAT_LOSS_PERCENT")
            .unwrap_or_else(|_| SAFETY_ZERO_VALUE.to_string()),
        telemetry_present,
        live_receipt,
    }
}

fn safety_reconnect_sessions(scenario: Scenario) -> usize {
    match scenario {
        Scenario::ReconnectFlagState
        | Scenario::ReconnectFlagScore
        | Scenario::SurvivalChestPersistence
        | Scenario::NegativeReconnectRace => SAFETY_RECONNECT_SESSION_COUNT,
        _ => SAFETY_SINGLE_SESSION_COUNT,
    }
}

fn evaluate_load_network_safety(input: LoadNetworkSafetyInputs) -> LoadNetworkSafetyEvidence {
    let authorized = input.owned_local_target || input.explicit_authorization;
    let mut missing_fields = Vec::new();
    push_missing_safety_field(
        &mut missing_fields,
        "target_scope",
        !input.target_scope.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "latency_ms",
        !input.latency_ms.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "jitter_ms",
        !input.jitter_ms.is_empty(),
    );
    push_missing_safety_field(
        &mut missing_fields,
        "loss_percent",
        !input.loss_percent.is_empty(),
    );

    let mut bound_violations = Vec::new();
    if input.public_target && !input.explicit_authorization {
        bound_violations.push("public_target_without_authorization");
    }
    if input.planned_clients == 0 {
        bound_violations.push("planned_clients_empty");
    }
    if input.planned_clients > input.max_clients {
        bound_violations.push("planned_clients_exceed_max");
    }
    if input.duration_secs == 0 {
        bound_violations.push("duration_empty");
    }
    if input.duration_secs > input.max_duration_secs {
        bound_violations.push("duration_exceeds_max");
    }

    let preflight_passed = authorized && missing_fields.is_empty() && bound_violations.is_empty();
    let promotion_ready = preflight_passed && input.telemetry_present && input.live_receipt;
    LoadNetworkSafetyEvidence {
        target_scope: input.target_scope,
        owned_local_target: input.owned_local_target,
        explicit_authorization: input.explicit_authorization,
        public_target: input.public_target,
        authorized,
        planned_clients: input.planned_clients,
        max_clients: input.max_clients,
        duration_secs: input.duration_secs,
        max_duration_secs: input.max_duration_secs,
        reconnect_sessions: input.reconnect_sessions,
        latency_ms: input.latency_ms,
        jitter_ms: input.jitter_ms,
        loss_percent: input.loss_percent,
        telemetry_present: input.telemetry_present,
        live_receipt: input.live_receipt,
        missing_fields,
        bound_violations,
        preflight_passed,
        promotion_ready,
    }
}

fn push_missing_safety_field(
    missing_fields: &mut Vec<&'static str>,
    field: &'static str,
    present: bool,
) {
    if !present {
        missing_fields.push(field);
    }
}

fn is_negative_live_rail(scenario: Scenario) -> bool {
    matches!(
        scenario,
        Scenario::NegativeInventoryStaleState
            | Scenario::NegativeInventoryInvalidClick
            | Scenario::NegativeCustomPayload
            | Scenario::NegativeReconnectRace
            | Scenario::NegativeCtfWrongScore
    )
}

fn negative_live_rail_invalid_action(scenario: Scenario) -> Option<&'static str> {
    match scenario {
        Scenario::NegativeInventoryStaleState => Some("stale_inventory_state_id"),
        Scenario::NegativeInventoryInvalidClick => Some("invalid_slot_or_window_click"),
        Scenario::NegativeCustomPayload => Some("malformed_custom_payload"),
        Scenario::NegativeReconnectRace => Some("duplicate_reconnect_flag_transition"),
        Scenario::NegativeCtfWrongScore => Some("wrong_team_or_wrong_portal_score_attempt"),
        _ => None,
    }
}

fn negative_live_rail_postcondition_milestone(scenario: Scenario) -> Option<&'static str> {
    match scenario {
        Scenario::NegativeInventoryStaleState => Some("negative_inventory_stale_state_contained"),
        Scenario::NegativeInventoryInvalidClick => {
            Some("negative_inventory_invalid_click_restored")
        }
        Scenario::NegativeCustomPayload => Some("negative_custom_payload_contained"),
        Scenario::NegativeReconnectRace => Some("negative_reconnect_race_contained"),
        Scenario::NegativeCtfWrongScore => Some("negative_wrong_score_contained"),
        _ => None,
    }
}

fn observed_negative_live_rail_outcome(
    scenario: Scenario,
    scenario_evidence: &ScenarioEvidence,
) -> (Option<&'static str>, Option<String>, bool) {
    let Some(postcondition) = negative_live_rail_postcondition_milestone(scenario) else {
        return (None, None, false);
    };
    let observed = scenario_evidence
        .observed_milestones
        .iter()
        .any(|milestone| *milestone == postcondition);
    if !observed {
        return (None, None, false);
    }
    (
        Some(NEGATIVE_LIVE_RAIL_OBSERVED_OUTCOME_CONTAINMENT),
        Some(format!(
            "{NEGATIVE_LIVE_RAIL_OUTCOME_SOURCE_PREFIX}{postcondition}"
        )),
        true,
    )
}

fn negative_live_rail_inputs_from_config(
    cfg: &Config,
    scenario_evidence: Option<&ScenarioEvidence>,
    telemetry_required: bool,
) -> NegativeLiveRailInputs {
    let selected = is_negative_live_rail(cfg.scenario);
    let (observed_outcome, observed_outcome_source, telemetry_present) = scenario_evidence
        .map(|scenario| observed_negative_live_rail_outcome(cfg.scenario, scenario))
        .unwrap_or((None, None, false));
    NegativeLiveRailInputs {
        selected,
        rail: selected.then(|| scenario_name(cfg.scenario)),
        invalid_action: negative_live_rail_invalid_action(cfg.scenario),
        expected_outcome: selected.then_some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME),
        observed_outcome,
        observed_outcome_source,
        postcondition_milestone: negative_live_rail_postcondition_milestone(cfg.scenario),
        telemetry_required,
        telemetry_present,
        target_scope: SAFETY_OWNED_LOCAL_SCOPE,
        explicit_authorization: cfg.negative_external_authorized,
        public_target: cfg.negative_public_target,
        planned_clients: planned_client_usernames(cfg).len(),
        max_clients: NEGATIVE_LIVE_RAIL_MAX_CLIENTS,
        timeout_secs: cfg.client_timeout.as_secs(),
    }
}

fn evaluate_negative_live_rail_safety_from_inputs(
    input: NegativeLiveRailInputs,
) -> NegativeLiveRailEvidence {
    let owned_local_target = !input.public_target;
    let mut missing_fields = Vec::new();
    if input.selected {
        push_missing_safety_field(
            &mut missing_fields,
            "invalid_action",
            input.invalid_action.is_some(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "expected_outcome",
            input.expected_outcome.is_some(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "target_scope",
            !input.target_scope.is_empty(),
        );
        push_missing_safety_field(
            &mut missing_fields,
            "postcondition_milestone",
            input.postcondition_milestone.is_some(),
        );
        if input.telemetry_required {
            push_missing_safety_field(
                &mut missing_fields,
                "telemetry",
                input.telemetry_present && input.observed_outcome.is_some(),
            );
        }
    }
    let mut bound_violations = Vec::new();
    if input.selected && input.public_target && !input.explicit_authorization {
        bound_violations.push("public_target_without_authorization");
    }
    if input.selected && input.planned_clients == 0 {
        bound_violations.push("planned_clients_empty");
    }
    if input.selected && input.planned_clients > input.max_clients {
        bound_violations.push("planned_clients_exceed_negative_max");
    }
    if input.selected && input.timeout_secs < NEGATIVE_LIVE_RAIL_MIN_TIMEOUT_SECS {
        bound_violations.push("timeout_empty");
    }
    let preflight_passed = !input.selected
        || ((owned_local_target || input.explicit_authorization)
            && missing_fields.is_empty()
            && bound_violations.is_empty());
    NegativeLiveRailEvidence {
        selected: input.selected,
        rail: input.rail,
        invalid_action: input.invalid_action,
        expected_outcome: input.expected_outcome,
        observed_outcome: input.observed_outcome,
        observed_outcome_source: input.observed_outcome_source,
        postcondition_milestone: input.postcondition_milestone,
        telemetry_present: input.telemetry_present,
        target_scope: input.target_scope,
        owned_local_target,
        explicit_authorization: input.explicit_authorization,
        public_target: input.public_target,
        planned_clients: input.planned_clients,
        max_clients: input.max_clients,
        timeout_secs: input.timeout_secs,
        missing_fields,
        bound_violations,
        preflight_passed,
    }
}

fn evaluate_negative_live_rail_safety(cfg: &Config) -> NegativeLiveRailEvidence {
    evaluate_negative_live_rail_safety_from_inputs(negative_live_rail_inputs_from_config(
        cfg, None, false,
    ))
}

fn uses_armor_mitigation_probe(scenario: Scenario) -> bool {
    matches!(
        scenario,
        Scenario::ArmorEquipmentMitigation | Scenario::ArmorLoadoutEnchantmentStatusMatrix
    )
}

fn evaluate_armor_loadout_enchantment_status_matrix(
    cfg: &Config,
    scenario: &ScenarioEvidence,
    server_scenario: &ServerScenarioEvidence,
) -> ArmorLoadoutEnchantmentStatusMatrixEvidence {
    let selected = cfg.scenario == Scenario::ArmorLoadoutEnchantmentStatusMatrix;
    let observed_live_evidence =
        selected && cfg.mode == Mode::Run && scenario.passed && server_scenario.passed;
    ArmorLoadoutEnchantmentStatusMatrixEvidence {
        selected,
        row_id: ARMOR_MATRIX_ROW_ID,
        loadout_id: ARMOR_MATRIX_LOADOUT_ID,
        equipment_slots: vec![ARMOR_MATRIX_EQUIPMENT_SLOT],
        enchantments: vec![ARMOR_MATRIX_ENCHANTMENT_NONE],
        status_effects: vec![ARMOR_MATRIX_STATUS_EFFECT_NONE],
        attack_type: ARMOR_MATRIX_ATTACK_TYPE_MELEE,
        reference_required: false,
        reference_receipt: ARMOR_MATRIX_REFERENCE_RECEIPT_NONE,
        live_receipt: observed_live_evidence,
        promotion_ready: observed_live_evidence,
        required_client_milestones: scenario_required_milestones(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        )
        .iter()
        .map(|(name, _)| *name)
        .collect(),
        observed_client_milestones: scenario.observed_milestones.clone(),
        required_server_milestones: server_required_milestones(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        )
        .iter()
        .map(|(name, _)| *name)
        .collect(),
        observed_server_milestones: server_scenario.observed_milestones.clone(),
        non_claims: ARMOR_MATRIX_NON_CLAIMS.to_vec(),
    }
}

fn evaluate_equipment_slot_item_matrix_expansion(
    cfg: &Config,
    scenario: &ScenarioEvidence,
    server_scenario: &ServerScenarioEvidence,
) -> EquipmentSlotItemMatrixExpansionEvidence {
    let selected = cfg.scenario == Scenario::EquipmentSlotItemMatrixExpansion;
    let observed_live_evidence =
        selected && cfg.mode == Mode::Run && scenario.passed && server_scenario.passed;
    EquipmentSlotItemMatrixExpansionEvidence {
        selected,
        row_id: EQUIPMENT_MATRIX_ROW_ID,
        actor_username: EQUIPMENT_MATRIX_ACTOR,
        observer_username: EQUIPMENT_MATRIX_OBSERVER,
        remote_entity_id: EQUIPMENT_MATRIX_REMOTE_ENTITY_ID,
        semantic_slot: EQUIPMENT_MATRIX_SEMANTIC_SLOT,
        wire_slot: EQUIPMENT_MATRIX_WIRE_SLOT,
        item_id: EQUIPMENT_MATRIX_ITEM_ID,
        item_count: EQUIPMENT_MATRIX_ITEM_COUNT,
        transition_kind: EQUIPMENT_MATRIX_TRANSITION,
        update_order: EQUIPMENT_MATRIX_UPDATE_ORDER,
        reference_required: false,
        reference_receipt: EQUIPMENT_MATRIX_REFERENCE_RECEIPT_NONE,
        live_receipt: observed_live_evidence,
        promotion_ready: observed_live_evidence,
        required_client_milestones: scenario_required_milestones(
            Scenario::EquipmentSlotItemMatrixExpansion,
        )
        .iter()
        .map(|(name, _)| *name)
        .collect(),
        observed_client_milestones: scenario.observed_milestones.clone(),
        required_server_milestones: server_required_milestones(
            Scenario::EquipmentSlotItemMatrixExpansion,
        )
        .iter()
        .map(|(name, _)| *name)
        .collect(),
        observed_server_milestones: server_scenario.observed_milestones.clone(),
        non_claims: EQUIPMENT_MATRIX_NON_CLAIMS.to_vec(),
    }
}

fn validate_negative_live_rail_preflight(cfg: &Config) -> Result<(), String> {
    let evidence = evaluate_negative_live_rail_safety(cfg);
    if evidence.preflight_passed {
        return Ok(());
    }
    Err(format!(
        "negative live rail preflight failed: missing={:?} bound_violations={:?}",
        evidence.missing_fields, evidence.bound_violations
    ))
}

impl Config {
    fn defaults(root: PathBuf) -> Self {
        Config {
            client_dir: root.join("stevenarella"),
            valence_repo: root.join("valence"),
            valence_rev: DEFAULT_VALENCE_REV.to_string(),
            valence_worktree: PathBuf::from("/tmp/valence-compat-758"),
            valence_example: DEFAULT_VALENCE_EXAMPLE.to_string(),
            valence_log: PathBuf::from("/tmp/mc-compat-valence.log"),
            valence_target_dir: PathBuf::from("/tmp/valence-compat-758-target"),
            valence_pid_file: PathBuf::from("/tmp/mc-compat-valence.pid"),
            server_backend: ServerBackend::Valence,
            target_dir: PathBuf::from("/tmp/stevenarella-target2"),
            server_name: "mc-compat-1-18-2".to_string(),
            server_version: DEFAULT_SERVER_VERSION.to_string(),
            server_protocol: DEFAULT_SERVER_PROTOCOL,
            server_port: 25565,
            client_username: DEFAULT_CLIENT_USERNAME.to_string(),
            docker_image: "itzg/minecraft-server:java17".to_string(),
            paper_plugin_jar: None,
            mode: Mode::DryRun,
            keep_server: false,
            client_timeout: Duration::from_secs(DEFAULT_CLIENT_TIMEOUT_SECS),
            client_success_needles: DEFAULT_SUCCESS_PATTERN
                .iter()
                .map(|s| s.to_string())
                .collect(),
            scenario: Scenario::Smoke,
            expected_status_description: None,
            expected_status_version_name: None,
            expected_status_sample: Vec::new(),
            packet_capture_summary: false,
            proxy_route: None,
            proxy_forwarding_mode: None,
            receipt_path: None,
            receipt_dir: None,
            compare_receipts: None,
            config_path: None,
            steel_config_path: None,
            matrix_dry_run: false,
            cleanup_apply: false,
            negative_public_target: false,
            negative_external_authorized: false,
            arrow_damage_policy: default_arrow_damage_policy(),
            root,
        }
    }

    fn from_env_and_args() -> Result<Self, String> {
        Self::from_sources(
            env::current_dir().map_err(|e| format!("current dir: {e}"))?,
            |name| env::var(name).ok().filter(|s| !s.is_empty()),
            env::args().skip(1),
        )
    }

    fn from_sources<I, F>(current_dir: PathBuf, mut get_env: F, args: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = String>,
        F: FnMut(&str) -> Option<String>,
    {
        let args_vec: Vec<String> = args.into_iter().collect();
        let root = get_env("MC_COMPAT_ROOT")
            .or_else(|| get_env("ROOT"))
            .map(PathBuf::from)
            .unwrap_or(current_dir);
        let mut cfg = Config::defaults(root);

        let config_path = find_config_path(get_env("MC_COMPAT_CONFIG"), &args_vec)?;
        let steel_config_path = find_named_config_path(
            "--steel-config",
            "MC_COMPAT_STEEL_CONFIG",
            get_env("MC_COMPAT_STEEL_CONFIG"),
            &args_vec,
        )?;
        let mut server_port_was_set = false;
        if let Some(path) = config_path {
            server_port_was_set |= apply_config_file(&mut cfg, &path)?;
            cfg.config_path = Some(path);
        }
        if let Some(path) = steel_config_path {
            server_port_was_set |= apply_steel_config_file(&mut cfg, &path)?;
            cfg.steel_config_path = Some(path);
        }

        apply_env_overrides(&mut cfg, &mut get_env, &mut server_port_was_set)?;

        let mut args = args_vec.into_iter();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--dry-run" => {
                    if cfg.mode == Mode::RunMatrix {
                        cfg.matrix_dry_run = true;
                    } else if cfg.mode == Mode::Cleanup {
                        cfg.cleanup_apply = false;
                    } else {
                        cfg.mode = Mode::DryRun;
                    }
                }
                "--run" => cfg.mode = Mode::Run,
                "--run-matrix" => {
                    cfg.mode = Mode::RunMatrix;
                    cfg.matrix_dry_run = false;
                }
                "--build-client" => cfg.mode = Mode::BuildClient,
                "--status-only" => cfg.mode = Mode::StatusOnly,
                "--status" => cfg.mode = Mode::HarnessStatus,
                "--cleanup" => cfg.mode = Mode::Cleanup,
                "--apply" => cfg.cleanup_apply = true,
                "--stop" => cfg.mode = Mode::Stop,
                "--config" => {
                    let path = PathBuf::from(args.next().ok_or_else(|| {
                        "--config requires a Nickel-exported JSON path".to_string()
                    })?);
                    server_port_was_set |= apply_config_file(&mut cfg, &path)?;
                    cfg.config_path = Some(path);
                }
                "--steel-config" => {
                    let path = PathBuf::from(args.next().ok_or_else(|| {
                        "--steel-config requires a Steel module path".to_string()
                    })?);
                    server_port_was_set |= apply_steel_config_file(&mut cfg, &path)?;
                    cfg.steel_config_path = Some(path);
                }
                "--compare-receipts" => {
                    let left = PathBuf::from(args.next().ok_or_else(|| {
                        "--compare-receipts requires PAPER_RECEIPT and VALENCE_RECEIPT".to_string()
                    })?);
                    let right = PathBuf::from(args.next().ok_or_else(|| {
                        "--compare-receipts requires PAPER_RECEIPT and VALENCE_RECEIPT".to_string()
                    })?);
                    cfg.mode = Mode::CompareReceipts;
                    cfg.compare_receipts = Some((left, right));
                }
                "--accept-eula" => {}
                "--keep-server" => cfg.keep_server = true,
                "--server-backend" => {
                    let value = args
                        .next()
                        .ok_or_else(|| "--server-backend requires valence or paper".to_string())?;
                    cfg.server_backend = parse_backend(&value)?;
                }
                "--client-dir" => {
                    cfg.client_dir = PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--client-dir requires a path".to_string())?,
                    );
                }
                "--receipt" => {
                    cfg.receipt_path = Some(PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--receipt requires a path".to_string())?,
                    ));
                }
                "--receipt-dir" => {
                    cfg.receipt_dir =
                        Some(PathBuf::from(args.next().ok_or_else(|| {
                            "--receipt-dir requires a path".to_string()
                        })?));
                }
                "--scenario" => {
                    let value = args.next().ok_or_else(|| {
                        format!("--scenario requires one of: {SUPPORTED_SCENARIO_USAGE}")
                    })?;
                    cfg.scenario = parse_scenario(&value)?;
                }
                "--expect-status-description" => {
                    cfg.expected_status_description = Some(args.next().ok_or_else(|| {
                        "--expect-status-description requires a string".to_string()
                    })?);
                }
                "--expect-status-version" => {
                    cfg.expected_status_version_name =
                        Some(args.next().ok_or_else(|| {
                            "--expect-status-version requires a string".to_string()
                        })?);
                }
                "--expect-status-sample" => {
                    cfg.expected_status_sample = args
                        .next()
                        .ok_or_else(|| {
                            "--expect-status-sample requires comma-separated names".to_string()
                        })?
                        .split(',')
                        .filter(|value| !value.is_empty())
                        .map(str::to_string)
                        .collect();
                }
                "--packet-capture-summary" => cfg.packet_capture_summary = true,
                "--proxy-route" => {
                    cfg.proxy_route = Some(
                        args.next()
                            .ok_or_else(|| "--proxy-route requires a route label".to_string())?,
                    );
                }
                "--proxy-forwarding-mode" => {
                    cfg.proxy_forwarding_mode = Some(args.next().ok_or_else(|| {
                        "--proxy-forwarding-mode requires a mode label".to_string()
                    })?);
                }
                "--valence-repo" => {
                    cfg.valence_repo = PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--valence-repo requires a path".to_string())?,
                    );
                }
                "--valence-rev" => {
                    cfg.valence_rev = args
                        .next()
                        .ok_or_else(|| "--valence-rev requires a git revision".to_string())?;
                }
                "-h" | "--help" => {
                    print_usage(&cfg);
                    std::process::exit(0);
                }
                _ if arg.starts_with("--config=") => {
                    let path = PathBuf::from(&arg[9..]);
                    server_port_was_set |= apply_config_file(&mut cfg, &path)?;
                    cfg.config_path = Some(path);
                }
                _ if arg.starts_with("--steel-config=") => {
                    let path = PathBuf::from(&arg[15..]);
                    server_port_was_set |= apply_steel_config_file(&mut cfg, &path)?;
                    cfg.steel_config_path = Some(path);
                }
                _ if arg.starts_with("--server-backend=") => {
                    cfg.server_backend = parse_backend(&arg[17..])?;
                }
                _ if arg.starts_with("--client-dir=") => {
                    cfg.client_dir = PathBuf::from(&arg[13..]);
                }
                _ if arg.starts_with("--receipt=") => {
                    cfg.receipt_path = Some(PathBuf::from(&arg[10..]));
                }
                _ if arg.starts_with("--receipt-dir=") => {
                    cfg.receipt_dir = Some(PathBuf::from(&arg[14..]));
                }
                _ if arg.starts_with("--scenario=") => {
                    cfg.scenario = parse_scenario(&arg[11..])?;
                }
                _ if arg.starts_with("--expect-status-description=") => {
                    cfg.expected_status_description = Some(arg[28..].to_string());
                }
                _ if arg.starts_with("--expect-status-version=") => {
                    cfg.expected_status_version_name = Some(arg[24..].to_string());
                }
                _ if arg.starts_with("--expect-status-sample=") => {
                    cfg.expected_status_sample = arg[23..]
                        .split(',')
                        .filter(|value| !value.is_empty())
                        .map(str::to_string)
                        .collect();
                }
                _ if arg == "--packet-capture-summary" => {
                    cfg.packet_capture_summary = true;
                }
                _ if arg.starts_with("--proxy-route=") => {
                    cfg.proxy_route = Some(arg[14..].to_string());
                }
                _ if arg.starts_with("--proxy-forwarding-mode=") => {
                    cfg.proxy_forwarding_mode = Some(arg[24..].to_string());
                }
                _ if arg.starts_with("--valence-repo=") => {
                    cfg.valence_repo = PathBuf::from(&arg[15..]);
                }
                _ if arg.starts_with("--valence-rev=") => {
                    cfg.valence_rev = arg[14..].to_string();
                }
                _ => return Err(format!("unknown arg: {arg}")),
            }
        }

        if !server_port_was_set {
            cfg.server_port = default_port(cfg.server_backend);
        }
        if cfg.mode == Mode::RunMatrix && cfg.receipt_path.is_some() {
            return Err("--run-matrix writes backend receipts under --receipt-dir; do not combine it with --receipt/SMOKE_RECEIPT".to_string());
        }
        Ok(cfg)
    }
}

fn find_config_path(env_path: Option<String>, args: &[String]) -> Result<Option<PathBuf>, String> {
    find_named_config_path("--config", "MC_COMPAT_CONFIG", env_path, args)
}

fn find_named_config_path(
    flag: &'static str,
    env_name: &'static str,
    env_path: Option<String>,
    args: &[String],
) -> Result<Option<PathBuf>, String> {
    let mut config_path = env_path.map(PathBuf::from);
    let equals_prefix = format!("{flag}=");
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        if arg == flag {
            let value = iter
                .next()
                .ok_or_else(|| format!("{flag} requires a path; env alternative is {env_name}"))?;
            config_path = Some(PathBuf::from(value));
        } else if let Some(value) = arg.strip_prefix(&equals_prefix) {
            config_path = Some(PathBuf::from(value));
        }
    }
    Ok(config_path)
}

fn apply_env_overrides<F>(
    cfg: &mut Config,
    get_env: &mut F,
    server_port_was_set: &mut bool,
) -> Result<(), String>
where
    F: FnMut(&str) -> Option<String>,
{
    if let Some(value) = get_env("CLIENT_DIR") {
        cfg.client_dir = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_REPO") {
        cfg.valence_repo = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_REV") {
        cfg.valence_rev = value;
    }
    if let Some(value) = get_env("VALENCE_WORKTREE") {
        cfg.valence_worktree = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_EXAMPLE") {
        cfg.valence_example = value;
    }
    if let Some(value) = get_env("VALENCE_LOG") {
        cfg.valence_log = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_TARGET_DIR") {
        cfg.valence_target_dir = PathBuf::from(value);
    }
    if let Some(value) = get_env("VALENCE_PID_FILE") {
        cfg.valence_pid_file = PathBuf::from(value);
    }
    if let Some(value) = get_env("SERVER_BACKEND") {
        cfg.server_backend = parse_backend(&value)?;
    }
    if let Some(value) = get_env("TARGET_DIR") {
        cfg.target_dir = PathBuf::from(value);
    }
    if let Some(value) = get_env("SERVER_NAME") {
        cfg.server_name = value;
    }
    if let Some(value) = get_env("SERVER_VERSION") {
        cfg.server_version = value;
    }
    if let Some(value) = get_env("SERVER_PROTOCOL") {
        cfg.server_protocol = value
            .parse()
            .map_err(|e| format!("parse SERVER_PROTOCOL: {e}"))?;
    }
    if let Some(value) = get_env("SERVER_PORT") {
        cfg.server_port = value
            .parse()
            .map_err(|e| format!("parse SERVER_PORT: {e}"))?;
        *server_port_was_set = true;
    }
    if let Some(value) = get_env("CLIENT_USERNAME") {
        cfg.client_username = value;
    }
    if let Some(value) = get_env("DOCKER_IMAGE") {
        cfg.docker_image = value;
    }
    if let Some(value) = get_env("PAPER_PLUGIN_JAR") {
        cfg.paper_plugin_jar = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("CLIENT_TIMEOUT") {
        cfg.client_timeout = Duration::from_secs(
            value
                .parse()
                .map_err(|e| format!("parse CLIENT_TIMEOUT: {e}"))?,
        );
    }
    if let Some(value) = get_env("CLIENT_SUCCESS_PATTERN") {
        cfg.client_success_needles = value.split('|').map(str::to_string).collect();
    }
    if let Some(value) = get_env("MC_COMPAT_SCENARIO") {
        cfg.scenario = parse_scenario(&value)?;
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_DESCRIPTION") {
        cfg.expected_status_description = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_VERSION") {
        cfg.expected_status_version_name = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_EXPECT_STATUS_SAMPLE") {
        cfg.expected_status_sample = value
            .split(',')
            .filter(|sample| !sample.is_empty())
            .map(str::to_string)
            .collect();
    }
    if get_env("MC_COMPAT_PACKET_CAPTURE_SUMMARY").is_some() {
        cfg.packet_capture_summary = true;
    }
    if let Some(value) = get_env("MC_COMPAT_PUBLIC_TARGET") {
        cfg.negative_public_target = value == "1";
    }
    if let Some(value) = get_env("MC_COMPAT_EXTERNAL_LOAD_AUTHORIZED") {
        cfg.negative_external_authorized = value == "1";
    }
    if let Some(value) = get_env("MC_COMPAT_PROXY_ROUTE") {
        cfg.proxy_route = Some(value);
    }
    if let Some(value) = get_env("MC_COMPAT_PROXY_FORWARDING_MODE") {
        cfg.proxy_forwarding_mode = Some(value);
    }
    if let Some(value) = get_env("SMOKE_RECEIPT") {
        cfg.receipt_path = Some(PathBuf::from(value));
    }
    if let Some(value) = get_env("SMOKE_RECEIPT_DIR") {
        cfg.receipt_dir = Some(PathBuf::from(value));
    }
    Ok(())
}

fn apply_config_file(cfg: &mut Config, path: &Path) -> Result<bool, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("read config {}: {e}", path.display()))?;
    apply_config_json(cfg, &text).map_err(|e| format!("config {}: {e}", path.display()))
}

fn apply_steel_config_file(cfg: &mut Config, path: &Path) -> Result<bool, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("read Steel config {}: {e}", path.display()))?;
    let source = runtime_config::SteelSource {
        path: path.display().to_string(),
        module_blake3: "runtime-unverified".to_string(),
        sandbox_profile: "mc-compat/pure-v1".to_string(),
    };
    let snapshot = runtime_config::evaluate_steel_module(source, &text).map_err(|diagnostics| {
        let details = diagnostics
            .into_iter()
            .map(|diagnostic| format!("{}: {}", diagnostic.path, diagnostic.message))
            .collect::<Vec<_>>()
            .join("; ");
        format!("Steel config {}: {details}", path.display())
    })?;
    cfg.server_backend = parse_backend(&snapshot.server_backend)?;
    cfg.server_version = snapshot.server_version;
    cfg.server_protocol = snapshot.server_protocol;
    cfg.server_port = snapshot.server_port;
    cfg.valence_rev = snapshot.valence_rev;
    cfg.valence_example = snapshot.valence_example;
    cfg.valence_worktree = PathBuf::from(snapshot.valence_worktree);
    cfg.valence_target_dir = PathBuf::from(snapshot.valence_target_dir);
    cfg.valence_log = PathBuf::from(snapshot.valence_log);
    cfg.valence_pid_file = PathBuf::from(snapshot.valence_pid_file);
    cfg.client_username = snapshot.client_username;
    cfg.client_timeout = Duration::from_secs(u64::from(snapshot.client_timeout_secs));
    cfg.client_success_needles = snapshot.client_success_patterns;
    cfg.receipt_dir = Some(PathBuf::from(snapshot.receipt_dir));
    cfg.scenario = parse_scenario(&snapshot.scenario)?;
    cfg.arrow_damage_policy = snapshot.arrow_damage;
    Ok(true)
}

fn apply_config_json(cfg: &mut Config, text: &str) -> Result<bool, String> {
    let mut server_port_was_set = false;
    if let Some(value) = json_optional_string_field(text, "client_dir")? {
        cfg.client_dir = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_repo")? {
        cfg.valence_repo = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_rev")? {
        cfg.valence_rev = value;
    }
    if let Some(value) = json_optional_string_field(text, "valence_worktree")? {
        cfg.valence_worktree = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_example")? {
        cfg.valence_example = value;
    }
    if let Some(value) = json_optional_string_field(text, "valence_log")? {
        cfg.valence_log = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_target_dir")? {
        cfg.valence_target_dir = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "valence_pid_file")? {
        cfg.valence_pid_file = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "server_backend")? {
        cfg.server_backend = parse_backend(&value)?;
        cfg.server_port = default_port(cfg.server_backend);
    }
    if let Some(value) = json_optional_string_field(text, "target_dir")? {
        cfg.target_dir = PathBuf::from(value);
    }
    if let Some(value) = json_optional_string_field(text, "server_name")? {
        cfg.server_name = value;
    }
    if let Some(value) = json_optional_string_field(text, "server_version")? {
        cfg.server_version = value;
    }
    if let Some(value) = json_optional_u32_field(text, "server_protocol")? {
        cfg.server_protocol = value;
    }
    if let Some(value) = json_optional_u32_field(text, "server_port")? {
        cfg.server_port =
            u16::try_from(value).map_err(|_| format!("server_port {value} exceeds u16"))?;
        server_port_was_set = true;
    }
    if let Some(value) = json_optional_string_field(text, "client_username")? {
        cfg.client_username = value;
    }
    if let Some(value) = json_optional_string_field(text, "docker_image")? {
        cfg.docker_image = value;
    }
    if let Some(value) = json_optional_string_field(text, "paper_plugin_jar")? {
        cfg.paper_plugin_jar = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_u32_field(text, "client_timeout_secs")? {
        cfg.client_timeout = Duration::from_secs(u64::from(value));
    }
    if let Some(value) = json_optional_string_array_field(text, "client_success_patterns")? {
        cfg.client_success_needles = value;
    }
    if let Some(value) = json_optional_string_field(text, "scenario")? {
        cfg.scenario = parse_scenario(&value)?;
    }
    if let Some(value) = json_optional_string_field(text, "expected_status_description")? {
        cfg.expected_status_description = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, "expected_status_version_name")? {
        cfg.expected_status_version_name = Some(value);
    }
    if let Some(value) = json_optional_string_array_field(text, "expected_status_sample")? {
        cfg.expected_status_sample = value;
    }
    if let Some(value) = json_optional_bool_field(text, "packet_capture_summary")? {
        cfg.packet_capture_summary = value;
    }
    if let Some(value) = json_optional_string_field(text, "proxy_route")? {
        cfg.proxy_route = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, "proxy_forwarding_mode")? {
        cfg.proxy_forwarding_mode = Some(value);
    }
    if let Some(value) = json_optional_string_field(text, "receipt_path")? {
        cfg.receipt_path = Some(PathBuf::from(value));
    }
    if let Some(value) = json_optional_string_field(text, "receipt_dir")? {
        cfg.receipt_dir = Some(PathBuf::from(value));
    }
    Ok(server_port_was_set)
}

fn parse_scenario(value: &str) -> Result<Scenario, String> {
    match value {
        "smoke" => Ok(Scenario::Smoke),
        "valence-compat-bot-probe" | "compat-bot-probe" => Ok(Scenario::CompatBotProbe),
        "flag-score-repeat" => Ok(Scenario::FlagScoreRepeat),
        "blue-flag-score" => Ok(Scenario::BlueFlagScore),
        "inventory-interaction" => Ok(Scenario::InventoryInteraction),
        "survival-break-place-pickup" => Ok(Scenario::SurvivalBreakPlacePickup),
        "survival-chest-persistence" => Ok(Scenario::SurvivalChestPersistence),
        "combat-damage" => Ok(Scenario::CombatDamage),
        "combat-knockback" => Ok(Scenario::CombatKnockback),
        "armor-equipment-mitigation" => Ok(Scenario::ArmorEquipmentMitigation),
        "armor-loadout-enchantment-status-matrix" => {
            Ok(Scenario::ArmorLoadoutEnchantmentStatusMatrix)
        }
        "equipment-update-observation" => Ok(Scenario::EquipmentUpdateObservation),
        "equipment-slot-item-matrix-expansion" => Ok(Scenario::EquipmentSlotItemMatrixExpansion),
        "projectile-hit" => Ok(Scenario::ProjectileHit),
        "projectile-damage-attribution" => Ok(Scenario::ProjectileDamageAttribution),
        "flag-carrier-death-return" => Ok(Scenario::FlagCarrierDeathReturn),
        "reconnect-flag-state" => Ok(Scenario::ReconnectFlagState),
        "reconnect-flag-score" => Ok(Scenario::ReconnectFlagScore),
        "multi-client-load-score" => Ok(Scenario::MultiClientLoadScore),
        "negative-inventory-stale-state" => Ok(Scenario::NegativeInventoryStaleState),
        "negative-inventory-invalid-click" => Ok(Scenario::NegativeInventoryInvalidClick),
        "negative-custom-payload" => Ok(Scenario::NegativeCustomPayload),
        "negative-reconnect-race" => Ok(Scenario::NegativeReconnectRace),
        "negative-ctf-wrong-score" => Ok(Scenario::NegativeCtfWrongScore),
        other => Err(format!("unknown scenario: {other}")),
    }
}

fn scenario_name(scenario: Scenario) -> &'static str {
    match scenario {
        Scenario::Smoke => "smoke",
        Scenario::CompatBotProbe => "valence-compat-bot-probe",
        Scenario::FlagScoreRepeat => "flag-score-repeat",
        Scenario::BlueFlagScore => "blue-flag-score",
        Scenario::InventoryInteraction => "inventory-interaction",
        Scenario::SurvivalBreakPlacePickup => "survival-break-place-pickup",
        Scenario::SurvivalChestPersistence => "survival-chest-persistence",
        Scenario::CombatDamage => "combat-damage",
        Scenario::CombatKnockback => "combat-knockback",
        Scenario::ArmorEquipmentMitigation => "armor-equipment-mitigation",
        Scenario::ArmorLoadoutEnchantmentStatusMatrix => "armor-loadout-enchantment-status-matrix",
        Scenario::EquipmentUpdateObservation => "equipment-update-observation",
        Scenario::EquipmentSlotItemMatrixExpansion => "equipment-slot-item-matrix-expansion",
        Scenario::ProjectileHit => "projectile-hit",
        Scenario::ProjectileDamageAttribution => "projectile-damage-attribution",
        Scenario::FlagCarrierDeathReturn => "flag-carrier-death-return",
        Scenario::ReconnectFlagState => "reconnect-flag-state",
        Scenario::ReconnectFlagScore => "reconnect-flag-score",
        Scenario::MultiClientLoadScore => "multi-client-load-score",
        Scenario::NegativeInventoryStaleState => "negative-inventory-stale-state",
        Scenario::NegativeInventoryInvalidClick => "negative-inventory-invalid-click",
        Scenario::NegativeCustomPayload => "negative-custom-payload",
        Scenario::NegativeReconnectRace => "negative-reconnect-race",
        Scenario::NegativeCtfWrongScore => "negative-ctf-wrong-score",
    }
}

fn scenario_required_milestones(scenario: Scenario) -> &'static [(&'static str, &'static str)] {
    match scenario {
        Scenario::Smoke => &[("protocol_detected", "Detected server protocol version")],
        Scenario::CompatBotProbe => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
        ],
        Scenario::FlagScoreRepeat => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_1", "RED: 1"),
            ("score_red_2", "RED: 2"),
        ],
        Scenario::BlueFlagScore => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_blue", "You are on team BLUE!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_blue_1", "BLUE: 1"),
        ],
        Scenario::InventoryInteraction => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("inventory_slot_update", "inventory_probe_set_slot"),
            ("inventory_sword_slot", "inventory_probe_slot36_nonempty"),
            ("inventory_wool_slot", "inventory_probe_slot37_stack"),
            ("inventory_drop_sent", "inventory_probe_drop_item_sent"),
            ("inventory_pickup_seen", "inventory_probe_collect_item"),
            ("inventory_click_sent", "inventory_probe_click_slot_sent"),
            (
                "inventory_open_container_seen",
                "inventory_probe_open_container",
            ),
            (
                "inventory_container_click_sent",
                "inventory_probe_container_click_sent",
            ),
            (
                "inventory_block_place_sent",
                "inventory_probe_place_block_sent",
            ),
        ],
        Scenario::SurvivalBreakPlacePickup => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("survival_break_sent", "survival_probe_break_block_sent"),
            ("survival_break_update", "survival_probe_block_update"),
            ("survival_pickup_seen", "survival_probe_pickup_seen"),
            ("survival_place_sent", "survival_probe_place_block_sent"),
            ("survival_place_update", "survival_probe_place_update"),
        ],
        Scenario::SurvivalChestPersistence => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_chest_open_seen",
                SURVIVAL_CHEST_CLIENT_OPEN_NEEDLE,
            ),
            (
                "survival_chest_store_sent",
                SURVIVAL_CHEST_CLIENT_STORE_NEEDLE,
            ),
            (
                "survival_chest_close_sent",
                SURVIVAL_CHEST_CLIENT_CLOSE_NEEDLE,
            ),
            (
                "survival_chest_reconnect_sent",
                SURVIVAL_CHEST_CLIENT_RECONNECT_NEEDLE,
            ),
            (
                "survival_chest_reopen_seen",
                SURVIVAL_CHEST_CLIENT_REOPEN_NEEDLE,
            ),
            (
                "survival_chest_persisted_seen",
                SURVIVAL_CHEST_CLIENT_PERSISTED_NEEDLE,
            ),
        ],
        Scenario::CombatDamage => &[
            ("multi_client_count", "mc_compat_combat_client_count=2"),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("team_blue", "You are on team BLUE!"),
            ("remote_player_spawn", "remote_player_spawn"),
            ("combat_attack_sent", "combat_probe_attack_sent"),
            ("combat_health_update", "update_health health=16.0"),
        ],
        Scenario::CombatKnockback => &[
            ("multi_client_count", "mc_compat_combat_client_count=2"),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("team_blue", "You are on team BLUE!"),
            ("remote_player_spawn", "remote_player_spawn"),
            ("combat_attack_sent", "combat_probe_attack_sent"),
            ("combat_health_update", "update_health health=16.0"),
            ("combat_velocity_update", "combat_probe_velocity_observed"),
        ],
        Scenario::ArmorEquipmentMitigation | Scenario::ArmorLoadoutEnchantmentStatusMatrix => &[
            ("multi_client_count", "mc_compat_combat_client_count=2"),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("team_blue", "You are on team BLUE!"),
            ("remote_player_spawn", "remote_player_spawn"),
            ("armor_inventory_slot", "inventory_probe_set_slot"),
            ("combat_attack_sent", "combat_probe_attack_sent"),
            ("combat_health_update", "update_health health=18.0"),
        ],
        Scenario::EquipmentUpdateObservation | Scenario::EquipmentSlotItemMatrixExpansion => &[
            (
                "multi_client_count",
                "mc_compat_equipment_update_client_count=2",
            ),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("team_blue", "You are on team BLUE!"),
            ("remote_player_spawn", "remote_player_spawn"),
            (
                "entity_equipment_update",
                "equipment_probe_entity_equipment",
            ),
        ],
        Scenario::ProjectileHit => &[
            (
                "multi_client_count",
                "mc_compat_projectile_hit_client_count=2",
            ),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("remote_player_spawn", "remote_player_spawn"),
            ("projectile_use_sent", "projectile_probe_use_item_sent"),
            ("projectile_swing_sent", "projectile_probe_swing_sent"),
        ],
        Scenario::ProjectileDamageAttribution => &[
            (
                "multi_client_count",
                "mc_compat_projectile_damage_client_count=2",
            ),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("team_blue", "You are on team BLUE!"),
            ("remote_player_spawn", "remote_player_spawn"),
            ("projectile_use_sent", "projectile_probe_use_item_sent"),
            ("projectile_swing_sent", "projectile_probe_swing_sent"),
            ("projectile_damage_update", "update_health health=17.0"),
        ],
        Scenario::FlagCarrierDeathReturn => &[
            (
                "multi_client_count",
                "mc_compat_flag_carrier_death_client_count=2",
            ),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("team_blue", "You are on team BLUE!"),
            ("flag_pickup", "You have the flag!"),
            ("remote_player_spawn", "remote_player_spawn"),
            ("combat_attack_sent", "combat_probe_attack_sent"),
            ("combat_death_observed", "combat_probe_death_observed"),
            ("respawn_request_sent", "respawn_probe_request_sent"),
            ("respawn_health_restored", "respawn_probe_health_restored"),
        ],
        Scenario::ReconnectFlagState => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("reconnect_session", "mc_compat_reconnect_session=2"),
        ],
        Scenario::ReconnectFlagScore => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_1", "RED: 1"),
            ("reconnect_session", "mc_compat_reconnect_session=2"),
        ],
        Scenario::MultiClientLoadScore => &[
            ("multi_client_count", "mc_compat_multi_client_count=2"),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_1", "RED: 1"),
        ],
        Scenario::NegativeInventoryStaleState => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "negative_inventory_stale_state_sent",
                "negative_inventory_stale_state_sent",
            ),
            (
                "negative_inventory_stale_state_contained",
                "negative_inventory_stale_state_contained",
            ),
        ],
        Scenario::NegativeInventoryInvalidClick => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "negative_inventory_invalid_click_sent",
                "negative_inventory_invalid_click_sent",
            ),
            (
                "negative_inventory_invalid_click_restored",
                "negative_inventory_invalid_click_restored",
            ),
        ],
        Scenario::NegativeCustomPayload => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "negative_custom_payload_sent",
                "negative_custom_payload_sent",
            ),
            (
                "negative_custom_payload_contained",
                "negative_custom_payload_contained",
            ),
        ],
        Scenario::NegativeReconnectRace => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("flag_pickup", "You have the flag!"),
            ("reconnect_session", "mc_compat_reconnect_session=2"),
            (
                "negative_reconnect_race_attempted",
                "negative_reconnect_race_attempted",
            ),
            (
                "negative_reconnect_race_contained",
                "negative_reconnect_race_contained",
            ),
        ],
        Scenario::NegativeCtfWrongScore => &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "negative_wrong_score_attempted",
                "negative_wrong_score_attempted",
            ),
            (
                "negative_wrong_score_contained",
                "negative_wrong_score_contained",
            ),
        ],
    }
}

fn scenario_forbidden_patterns(scenario: Scenario) -> &'static [(&'static str, &'static str)] {
    match scenario {
        Scenario::FlagCarrierDeathReturn
        | Scenario::ReconnectFlagState
        | Scenario::NegativeReconnectRace
        | Scenario::NegativeCtfWrongScore => &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("unexpected_flag_capture", "You captured the flag!"),
            ("unexpected_flag_capture_milestone", "flag_capture"),
            ("unexpected_red_score", "RED: 1"),
            ("unexpected_blue_score", "BLUE: 1"),
        ],
        _ => &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
    }
}

fn server_required_milestones(scenario: Scenario) -> &'static [(&'static str, &'static str)] {
    match scenario {
        Scenario::Smoke | Scenario::CompatBotProbe => &[],
        Scenario::FlagScoreRepeat | Scenario::BlueFlagScore | Scenario::ReconnectFlagScore => &[
            ("server_username_seen", "compatbot"),
            ("server_flag_or_score", "flag"),
        ],
        Scenario::ReconnectFlagState => &[
            ("server_username_seen", "compatbot"),
            ("server_flag_pickup", "flag_pickup"),
            ("server_flag_disconnect_return", "flag_disconnect_return"),
            (
                "server_reconnect_state_coherent",
                "reconnect_state_coherent",
            ),
        ],
        Scenario::MultiClientLoadScore => &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_flag_or_score", "flag"),
        ],
        Scenario::InventoryInteraction => &[
            ("server_username_seen", "compatbot"),
            ("server_inventory_hotbar_select", "inventory_hotbar_select"),
            ("server_inventory_drop", "inventory_drop_item"),
            ("server_inventory_pickup", "inventory_pickup_item"),
            ("server_inventory_click", "inventory_click_slot"),
            (
                "server_inventory_open_container",
                "inventory_open_container",
            ),
            (
                "server_inventory_container_click",
                "inventory_container_click",
            ),
            ("server_block_place", "block_place_item"),
        ],
        Scenario::SurvivalBreakPlacePickup => &[
            ("server_username_seen", "compatbot"),
            ("server_survival_join", "survival_join"),
            ("server_survival_break", "survival_block_break"),
            ("server_survival_pickup", "survival_pickup_item"),
            ("server_survival_place", "survival_block_place"),
        ],
        Scenario::SurvivalChestPersistence => &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_chest_open",
                SURVIVAL_CHEST_SERVER_OPEN_NEEDLE,
            ),
            (
                "server_survival_chest_store",
                SURVIVAL_CHEST_SERVER_STORE_NEEDLE,
            ),
            (
                "server_survival_chest_close",
                SURVIVAL_CHEST_SERVER_CLOSE_NEEDLE,
            ),
            (
                "server_survival_chest_reopen",
                SURVIVAL_CHEST_SERVER_REOPEN_NEEDLE,
            ),
            (
                "server_survival_chest_persisted",
                SURVIVAL_CHEST_SERVER_PERSISTED_NEEDLE,
            ),
        ],
        Scenario::CombatDamage => &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_combat_damage", "combat_damage"),
        ],
        Scenario::CombatKnockback => &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_combat_damage", "combat_damage"),
            ("server_combat_knockback", "combat_knockback"),
        ],
        Scenario::ArmorEquipmentMitigation | Scenario::ArmorLoadoutEnchantmentStatusMatrix => &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_equipment_state", "armor_equipment_state"),
            ("server_combat_damage", "combat_damage"),
            ("server_armor_mitigation", "combat_armor_mitigation"),
        ],
        Scenario::EquipmentUpdateObservation | Scenario::EquipmentSlotItemMatrixExpansion => &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_equipment_update_state", "equipment_update_state"),
        ],
        Scenario::ProjectileHit => &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_projectile_loadout", "projectile_loadout"),
        ],
        Scenario::ProjectileDamageAttribution => &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_projectile_loadout", "projectile_loadout"),
            ("server_projectile_use", "projectile_use"),
            ("server_projectile_hit", "projectile_hit"),
        ],
        Scenario::FlagCarrierDeathReturn => &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_flag_pickup", "flag_pickup"),
            ("server_flag_carrier_death", "flag_carrier_death"),
            ("server_flag_return", "flag_return"),
        ],
        Scenario::NegativeInventoryStaleState
        | Scenario::NegativeInventoryInvalidClick
        | Scenario::NegativeCustomPayload
        | Scenario::NegativeReconnectRace
        | Scenario::NegativeCtfWrongScore => &[("server_username_seen", "compatbot")],
    }
}

fn evaluate_scenario(scenario: Scenario, output: &str) -> ScenarioEvidence {
    evaluate_scenario_with_projectile_health(
        scenario,
        output,
        PROJECTILE_DAMAGE_CLIENT_HEALTH_NEEDLE,
    )
}

fn evaluate_scenario_for_config(cfg: &Config, output: &str) -> ScenarioEvidence {
    let health_needle = projectile_damage_client_health_needle(cfg);
    evaluate_scenario_with_projectile_health(cfg.scenario, output, &health_needle)
}

fn evaluate_scenario_with_projectile_health(
    scenario: Scenario,
    output: &str,
    projectile_health_needle: &str,
) -> ScenarioEvidence {
    let mut observed_milestones = Vec::new();
    let mut missing_milestones = Vec::new();
    for (name, needle) in scenario_required_milestones(scenario) {
        let effective_needle = if scenario == Scenario::ProjectileDamageAttribution
            && *name == "projectile_damage_update"
        {
            projectile_health_needle
        } else {
            needle
        };
        if output.contains(effective_needle) {
            observed_milestones.push(*name);
        } else {
            missing_milestones.push(*name);
        }
    }
    let mut forbidden_matches = Vec::new();
    for (name, needle) in scenario_forbidden_patterns(scenario) {
        if output.contains(needle) {
            forbidden_matches.push(*name);
        }
    }
    let passed = missing_milestones.is_empty() && forbidden_matches.is_empty();
    ScenarioEvidence {
        observed_milestones,
        missing_milestones,
        forbidden_matches,
        passed,
    }
}

fn evaluate_server_scenario(
    scenario: Scenario,
    server_log: &str,
    username: &str,
) -> ServerScenarioEvidence {
    let normalized = server_log.to_lowercase();
    let dynamic_username = username.to_lowercase();
    let mut observed_milestones = Vec::new();
    let mut missing_milestones = Vec::new();
    for (name, needle) in server_required_milestones(scenario) {
        let found = match *name {
            "server_username_seen" => normalized.contains(&dynamic_username),
            "server_client_a_seen" => normalized.contains(&format!("{dynamic_username}a")),
            "server_client_b_seen" => normalized.contains(&format!("{dynamic_username}b")),
            "server_flag_or_score" => normalized.contains("flag") || normalized.contains("score"),
            _ => normalized.contains(&needle.to_lowercase()),
        };
        if found {
            observed_milestones.push(*name);
        } else {
            missing_milestones.push(*name);
        }
    }
    let mut forbidden_matches = Vec::new();
    for (name, needle) in scenario_forbidden_patterns(scenario) {
        if normalized.contains(&needle.to_lowercase()) {
            forbidden_matches.push(*name);
        }
    }
    let passed = missing_milestones.is_empty() && forbidden_matches.is_empty();
    ServerScenarioEvidence {
        observed_milestones,
        missing_milestones,
        forbidden_matches,
        passed,
    }
}

fn parse_typed_event_line(line: &str) -> Result<TypedEvent, String> {
    let line = line.trim();
    let Some(rest) = line.strip_prefix(TYPED_EVENT_PREFIX) else {
        return Err("typed event line missing prefix".to_string());
    };
    let fields = parse_typed_event_fields(rest.trim())?;
    let schema_version = typed_event_required_u32(&fields, "schema")?;
    if schema_version != TYPED_EVENT_SCHEMA_VERSION {
        return Err(format!(
            "unsupported typed event schema {schema_version}, expected {TYPED_EVENT_SCHEMA_VERSION}"
        ));
    }
    Ok(TypedEvent {
        schema_version,
        source: typed_event_required_string(&fields, "source")?,
        scenario: typed_event_required_string(&fields, "scenario")?,
        session: typed_event_required_string(&fields, "session")?,
        username: typed_event_optional_string(&fields, "username"),
        sequence: u64::from(typed_event_required_u32(&fields, "seq")?),
        kind: typed_event_required_string(&fields, "event")?,
    })
}

fn parse_typed_event_fields(text: &str) -> Result<Vec<(&str, &str)>, String> {
    let mut fields = Vec::new();
    for token in text.split_whitespace() {
        let Some((key, value)) = token.split_once('=') else {
            return Err(format!("typed event token missing '=': {token}"));
        };
        fields.push((key, value));
    }
    Ok(fields)
}

fn typed_event_required_string(fields: &[(&str, &str)], key: &str) -> Result<String, String> {
    typed_event_optional_string(fields, key)
        .ok_or_else(|| format!("missing typed event field {key}"))
}

fn typed_event_optional_string(fields: &[(&str, &str)], key: &str) -> Option<String> {
    fields
        .iter()
        .find_map(|(field_key, value)| (*field_key == key).then(|| (*value).to_string()))
}

fn typed_event_required_u32(fields: &[(&str, &str)], key: &str) -> Result<u32, String> {
    let value = typed_event_required_string(fields, key)?;
    value
        .parse::<u32>()
        .map_err(|err| format!("parse typed event field {key}: {err}"))
}

fn evaluate_typed_event_graph(
    events: &[TypedEvent],
    scenario: &str,
    session: &str,
    username: Option<&str>,
    required_events: &[&str],
    forbidden_events: &[&str],
    ordered_edges: &[(&str, &str)],
) -> TypedEventGraphEvaluation {
    let relevant: Vec<&TypedEvent> = events
        .iter()
        .filter(|event| {
            event.scenario == scenario
                && event.session == session
                && username.map_or(true, |name| event.username.as_deref() == Some(name))
        })
        .collect();
    let mut observed_events = Vec::new();
    let mut missing_events = Vec::new();
    for required in required_events {
        if relevant.iter().any(|event| event.kind == *required) {
            observed_events.push((*required).to_string());
        } else {
            missing_events.push((*required).to_string());
        }
    }
    let mut forbidden_matches = Vec::new();
    for forbidden in forbidden_events {
        if relevant.iter().any(|event| event.kind == *forbidden) {
            forbidden_matches.push((*forbidden).to_string());
        }
    }
    let mut order_violations = Vec::new();
    for (before, after) in ordered_edges {
        if let (Some(before_seq), Some(after_seq)) = (
            first_typed_event_sequence(&relevant, before),
            first_typed_event_sequence(&relevant, after),
        ) {
            if before_seq >= after_seq {
                order_violations.push(format!("{before}_before_{after}"));
            }
        }
    }
    let passed =
        missing_events.is_empty() && forbidden_matches.is_empty() && order_violations.is_empty();
    TypedEventGraphEvaluation {
        observed_events,
        missing_events,
        forbidden_events: forbidden_matches,
        order_violations,
        passed,
    }
}

fn first_typed_event_sequence(events: &[&TypedEvent], kind: &str) -> Option<u64> {
    events
        .iter()
        .filter(|event| event.kind == kind)
        .map(|event| event.sequence)
        .min()
}

fn typed_events_from_receipt_evidence(
    cfg: &Config,
    client: &ClientRunEvidence,
) -> Result<Vec<TypedEvent>, String> {
    let scenario_label = scenario_name(cfg.scenario).to_string();
    let session = typed_event_session_id(cfg);
    let default_username = single_typed_event_username(client);
    let mut events = Vec::new();
    if let Some(scenario) = &client.scenario {
        for milestone in &scenario.observed_milestones {
            push_typed_event(
                &mut events,
                "client",
                &scenario_label,
                &session,
                default_username,
                milestone,
            )?;
        }
    }
    if let Some(server) = &client.server_scenario {
        for milestone in &server.observed_milestones {
            push_typed_event(
                &mut events,
                "server",
                &scenario_label,
                &session,
                default_username,
                milestone,
            )?;
        }
    }
    if let Some(causality) = &client.projectile_damage_causality {
        for step in &causality.observed_steps {
            let (source, username) = typed_event_projectile_step_source_username(
                step,
                &causality.attacker_username,
                &causality.victim_username,
            );
            push_typed_event(
                &mut events,
                source,
                &scenario_label,
                &session,
                username,
                step,
            )?;
        }
    }
    Ok(events)
}

fn push_typed_event(
    events: &mut Vec<TypedEvent>,
    source: &str,
    scenario: &str,
    session: &str,
    username: Option<&str>,
    kind: &str,
) -> Result<(), String> {
    let sequence_index = events.len() + TYPED_EVENT_SEQUENCE_INDEX_OFFSET;
    let sequence = u64::try_from(sequence_index)
        .map_err(|err| format!("typed event sequence overflow at {sequence_index}: {err}"))?;
    events.push(TypedEvent {
        schema_version: TYPED_EVENT_SCHEMA_VERSION,
        source: source.to_string(),
        scenario: scenario.to_string(),
        session: session.to_string(),
        username: username.map(sanitize_typed_event_field),
        sequence,
        kind: kind.to_string(),
    });
    Ok(())
}

fn single_typed_event_username(client: &ClientRunEvidence) -> Option<&str> {
    if client.usernames.len() == TYPED_EVENT_SINGLE_USERNAME_COUNT {
        client.usernames.first().map(String::as_str)
    } else {
        None
    }
}

fn typed_event_projectile_step_source_username<'a>(
    step: &str,
    attacker_username: &'a str,
    victim_username: &'a str,
) -> (&'static str, Option<&'a str>) {
    if step.starts_with("attacker_client") {
        ("client", Some(attacker_username))
    } else if step.starts_with("victim_client") {
        ("client", Some(victim_username))
    } else {
        ("server", None)
    }
}

fn typed_event_session_id(cfg: &Config) -> String {
    cfg.receipt_path
        .as_ref()
        .and_then(|path| path.file_stem())
        .and_then(|stem| stem.to_str())
        .map(sanitize_typed_event_field)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| TYPED_EVENT_DEFAULT_SESSION_ID.to_string())
}

fn sanitize_typed_event_field(value: &str) -> String {
    let mut sanitized = String::with_capacity(value.len().min(TYPED_EVENT_MAX_FIELD_CHARS));
    for ch in value.chars().take(TYPED_EVENT_MAX_FIELD_CHARS) {
        if ch.is_ascii_alphanumeric() || matches!(ch, '_' | '-' | '.') {
            sanitized.push(ch);
        } else {
            sanitized.push('_');
        }
    }
    sanitized
}

fn normalize_typed_event_timeline(events: &[TypedEvent]) -> String {
    let mut timeline = events.to_vec();
    timeline.sort_by(|left, right| {
        left.sequence
            .cmp(&right.sequence)
            .then_with(|| left.source.cmp(&right.source))
            .then_with(|| left.kind.cmp(&right.kind))
    });
    let mut output = String::new();
    for event in &timeline {
        output.push_str(&render_typed_event_line(event));
        output.push('\n');
    }
    output
}

fn render_typed_event_line(event: &TypedEvent) -> String {
    let username_field = event
        .username
        .as_ref()
        .map(|username| format!(" username={username}"))
        .unwrap_or_default();
    format!(
        "{TYPED_EVENT_PREFIX} schema={} source={} scenario={} session={}{} seq={} event={}",
        event.schema_version,
        event.source,
        event.scenario,
        event.session,
        username_field,
        event.sequence,
        event.kind
    )
}

fn typed_event_timeline_blake3(timeline: &str) -> String {
    blake3::hash(timeline.as_bytes()).to_hex().to_string()
}

fn typed_event_oracle_receipt_json(artifact: Option<&TypedEventOracleArtifact>) -> String {
    let selected = artifact.is_some();
    let migration_status = if selected {
        TYPED_EVENT_MIGRATION_DERIVED_FROM_MILESTONES
    } else {
        TYPED_EVENT_MIGRATION_FALLBACK
    };
    let event_log_path_json = artifact
        .map(|evidence| json_string(&evidence.event_log_path.display().to_string()))
        .unwrap_or_else(|| "null".to_string());
    let timeline_blake3_json = artifact
        .map(|evidence| json_string(&evidence.timeline_blake3))
        .unwrap_or_else(|| "null".to_string());
    let event_count = artifact
        .map(|evidence| evidence.event_count)
        .unwrap_or_default();
    let contributes_to_pass_fail = artifact
        .map(|evidence| evidence.contributes_to_pass_fail)
        .unwrap_or(false);
    format!(
        r#"{{
    "schema_version": {schema_version},
    "selected": {selected},
    "migration_status": {migration_status_json},
    "event_log_path": {event_log_path_json},
    "timeline_blake3": {timeline_blake3_json},
    "event_count": {event_count},
    "contributes_to_pass_fail": {contributes_to_pass_fail},
    "raw_payloads_recorded": false
  }}"#,
        schema_version = TYPED_EVENT_SCHEMA_VERSION,
        selected = selected,
        migration_status_json = json_string(migration_status),
        event_log_path_json = event_log_path_json,
        timeline_blake3_json = timeline_blake3_json,
        event_count = event_count,
        contributes_to_pass_fail = contributes_to_pass_fail,
    )
}

fn typed_event_oracle_contributes_to_pass_fail(scenario: Scenario) -> bool {
    matches!(scenario, Scenario::Smoke | Scenario::InventoryInteraction)
}

fn validate_typed_event_oracle_for_migrated_scenario(
    cfg: &Config,
    client: &ClientRunEvidence,
) -> Result<(), String> {
    if !typed_event_oracle_contributes_to_pass_fail(cfg.scenario) {
        return Ok(());
    }
    let events = typed_events_from_receipt_evidence(cfg, client)?;
    let required = typed_event_required_events_for_graph(cfg.scenario);
    let required_refs = required.iter().map(String::as_str).collect::<Vec<_>>();
    let forbidden = scenario_forbidden_patterns(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect::<Vec<_>>();
    let ordered_edges = typed_event_ordered_edges_for_scenario(cfg.scenario);
    let username = single_typed_event_username(client);
    let session = typed_event_session_id(cfg);
    let result = evaluate_typed_event_graph(
        &events,
        scenario_name(cfg.scenario),
        &session,
        username,
        &required_refs,
        &forbidden,
        &ordered_edges,
    );
    if result.passed {
        return Ok(());
    }
    Err(format!(
        "typed event oracle for scenario {} failed: missing={:?} forbidden={:?} order_violations={:?}",
        scenario_name(cfg.scenario),
        result.missing_events,
        result.forbidden_events,
        result.order_violations
    ))
}

fn typed_event_required_events_for_graph(scenario: Scenario) -> Vec<String> {
    let mut required = scenario_required_milestones(scenario)
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect::<Vec<_>>();
    required.extend(
        server_required_milestones(scenario)
            .iter()
            .map(|(name, _)| (*name).to_string()),
    );
    required
}

fn typed_event_ordered_edges_for_scenario(scenario: Scenario) -> Vec<(&'static str, &'static str)> {
    match scenario {
        Scenario::Smoke => vec![],
        Scenario::InventoryInteraction => vec![
            ("protocol_detected", "inventory_drop_sent"),
            ("inventory_drop_sent", "inventory_pickup_seen"),
            ("inventory_pickup_seen", "inventory_click_sent"),
            ("inventory_click_sent", "inventory_container_click_sent"),
            (
                "inventory_container_click_sent",
                "inventory_block_place_sent",
            ),
            ("server_inventory_drop", "server_inventory_pickup"),
            ("server_inventory_pickup", "server_inventory_click"),
            ("server_inventory_container_click", "server_block_place"),
        ],
        _ => vec![],
    }
}

fn projectile_damage_required_steps() -> Vec<&'static str> {
    vec![
        "attacker_client_projectile_use_sent",
        "attacker_client_projectile_swing_sent",
        "server_projectile_use_attacker_victim",
        "server_projectile_hit_attacker_victim_health_delta",
        "victim_client_damage_update",
    ]
}

fn evaluate_projectile_damage_causality(
    client_logs: &[ClientLogSlice<'_>],
    server_log: &str,
    base_username: &str,
) -> ProjectileDamageCausalityEvidence {
    evaluate_projectile_damage_causality_for_damage(
        client_logs,
        server_log,
        base_username,
        PROJECTILE_DAMAGE_AMOUNT_NEEDLE,
    )
}

fn evaluate_projectile_damage_causality_for_damage(
    client_logs: &[ClientLogSlice<'_>],
    server_log: &str,
    base_username: &str,
    expected_damage_needle: &str,
) -> ProjectileDamageCausalityEvidence {
    let fallback_attacker = format!("{base_username}{PROJECTILE_DAMAGE_ATTACKER_SUFFIX}");
    let fallback_victim = format!("{base_username}{PROJECTILE_DAMAGE_VICTIM_SUFFIX}");
    let server_use = first_projectile_server_use(server_log, expected_damage_needle);
    let (attacker_username, victim_username) = server_use
        .as_ref()
        .map(|event| (event.attacker.clone(), event.victim.clone()))
        .unwrap_or_else(|| (fallback_attacker, fallback_victim));
    let server_hit = first_projectile_server_hit(
        server_log,
        &attacker_username,
        &victim_username,
        server_use.as_ref().map(|event| event.line),
    );
    let attacker_log = client_log_for(client_logs, &attacker_username);
    let victim_log = client_log_for(client_logs, &victim_username);

    let attacker_use = first_line_index(attacker_log, PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE);
    let attacker_swing = first_line_index(attacker_log, PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE);
    let victim_health = server_hit
        .as_ref()
        .and_then(|hit| first_line_index(victim_log, &client_health_needle(&hit.health_after)));

    let mut observed_steps = Vec::new();
    let mut missing_steps = Vec::new();
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "attacker_client_projectile_use_sent",
        attacker_use,
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "attacker_client_projectile_swing_sent",
        attacker_swing,
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "server_projectile_use_attacker_victim",
        server_use.as_ref().map(|event| event.line),
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "server_projectile_hit_attacker_victim_health_delta",
        server_hit.as_ref().map(|event| event.line),
    );
    push_step_presence(
        &mut observed_steps,
        &mut missing_steps,
        "victim_client_damage_update",
        victim_health,
    );

    let mut order_violations = Vec::new();
    if let (Some(use_line), Some(swing_line)) = (attacker_use, attacker_swing) {
        if use_line >= swing_line {
            order_violations.push("attacker_client_use_before_swing");
        }
    }
    if let Some(use_event) = &server_use {
        if let Some(hit_event) = &server_hit {
            if use_event.line >= hit_event.line {
                order_violations.push("server_projectile_use_before_hit");
            }
        } else if first_projectile_server_hit(
            server_log,
            &attacker_username,
            &victim_username,
            None,
        )
        .is_some_and(|hit_event| hit_event.line < use_event.line)
        {
            order_violations.push("server_projectile_use_before_hit");
        }
    }

    let passed = missing_steps.is_empty() && order_violations.is_empty();
    ProjectileDamageCausalityEvidence {
        required_steps: projectile_damage_required_steps(),
        observed_steps,
        missing_steps,
        order_violations,
        attacker_username,
        victim_username,
        passed,
    }
}

fn push_step_presence(
    observed_steps: &mut Vec<&'static str>,
    missing_steps: &mut Vec<&'static str>,
    step: &'static str,
    line: Option<usize>,
) {
    if line.is_some() {
        observed_steps.push(step);
    } else {
        missing_steps.push(step);
    }
}

fn first_line_index(output: &str, needle: &str) -> Option<usize> {
    output.lines().position(|line| line.contains(needle))
}

fn client_log_for<'a>(client_logs: &'a [ClientLogSlice<'a>], username: &str) -> &'a str {
    client_logs
        .iter()
        .find(|log| log.username == username)
        .map(|log| log.output)
        .unwrap_or("")
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProjectileServerUse {
    line: usize,
    attacker: String,
    victim: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ProjectileServerHit {
    line: usize,
    health_after: String,
}

fn first_projectile_server_use(
    server_log: &str,
    expected_damage_needle: &str,
) -> Option<ProjectileServerUse> {
    server_log.lines().enumerate().find_map(|(line, text)| {
        if !text.contains(PROJECTILE_DAMAGE_SERVER_USE_NEEDLE)
            || !text.contains(PROJECTILE_DAMAGE_SEQUENCE_NEEDLE)
            || !text.contains(expected_damage_needle)
        {
            return None;
        }
        Some(ProjectileServerUse {
            line,
            attacker: field_value(text, "attacker=")?.to_string(),
            victim: field_value(text, "victim=")?.to_string(),
        })
    })
}

fn first_projectile_server_hit(
    server_log: &str,
    attacker_username: &str,
    victim_username: &str,
    after_line: Option<usize>,
) -> Option<ProjectileServerHit> {
    let attacker_needle = format!("attacker={attacker_username}");
    let victim_needle = format!("victim={victim_username}");
    server_log.lines().enumerate().find_map(|(line, text)| {
        if after_line.is_some_and(|minimum_line| line <= minimum_line)
            || !text.contains(PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE)
            || !text.contains(&attacker_needle)
            || !text.contains(&victim_needle)
        {
            return None;
        }
        Some(ProjectileServerHit {
            line,
            health_after: field_value(text, "victim_health_after=")?.to_string(),
        })
    })
}

fn field_value<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    let value_start = line.find(field)? + field.len();
    let value = &line[value_start..];
    let value_end = value.find(char::is_whitespace).unwrap_or(value.len());
    Some(&value[..value_end])
}

fn client_health_needle(health_after: &str) -> String {
    format!("update_health health={health_after}")
}

fn default_port(backend: ServerBackend) -> u16 {
    match backend {
        ServerBackend::Valence => 25565,
        ServerBackend::Paper => 25566,
    }
}

fn default_arrow_damage_policy() -> runtime_config::ArrowDamagePolicy {
    runtime_config::ArrowDamagePolicy {
        base_damage: DEFAULT_ARROW_DAMAGE,
        velocity_multiplier: DEFAULT_ARROW_VELOCITY_MULTIPLIER,
        max_damage: DEFAULT_ARROW_MAX_DAMAGE,
    }
}

fn projectile_damage_decision(cfg: &Config) -> runtime_config::ArrowDamageDecision {
    runtime_config::evaluate_arrow_damage(
        &cfg.arrow_damage_policy,
        &runtime_config::ProjectileDamageContext {
            projectile_velocity: PROJECTILE_DAMAGE_CONTEXT_VELOCITY,
            pull_strength: PROJECTILE_DAMAGE_CONTEXT_PULL_STRENGTH,
        },
    )
}

fn projectile_damage_amount_text(cfg: &Config) -> String {
    format_one_decimal(projectile_damage_decision(cfg).damage)
}

fn projectile_damage_amount_needle(cfg: &Config) -> String {
    format!("damage={}", projectile_damage_amount_text(cfg))
}

fn projectile_damage_client_health_needle(cfg: &Config) -> String {
    format!(
        "update_health health={}",
        projectile_damage_victim_health_after_text(cfg)
    )
}

fn projectile_damage_server_health_after_needle(cfg: &Config) -> String {
    format!(
        "victim_health_after={}",
        projectile_damage_victim_health_after_text(cfg)
    )
}

fn projectile_damage_victim_health_after_text(cfg: &Config) -> String {
    let after = PROJECTILE_DAMAGE_VICTIM_START_HEALTH - projectile_damage_decision(cfg).damage;
    format_one_decimal(after.max(0.0))
}

fn format_one_decimal(value: f64) -> String {
    format!("{value:.1}")
}

fn print_usage(cfg: &Config) {
    println!(
        "Usage: mc-compat-runner [--config PATH] [--steel-config PATH] [--dry-run|--run|--run-matrix] [--build-client] [--status-only] [--status] [--cleanup [--dry-run|--apply]] [--stop] [--compare-receipts PAPER_RECEIPT VALENCE_RECEIPT] [--scenario {}] [--keep-server] [--server-backend valence|paper] [--client-dir PATH] [--receipt PATH] [--receipt-dir DIR] [--valence-repo PATH] [--valence-rev REV]\n\n\
Automates a local Stevenarella compatibility smoke against a Minecraft {} / protocol {} server.\n\
Default client checkout is the editable local Stevenarella sibling at ./stevenarella; pass --client-dir/CLIENT_DIR to use another checkout.\n\
Pass --config/MC_COMPAT_CONFIG a JSON file exported from legacy Nickel config, or --steel-config/MC_COMPAT_STEEL_CONFIG a restricted Steel module; env vars and later CLI flags override either config source.\n\
Pass --receipt/SMOKE_RECEIPT to write a machine-readable mc.compat.scenario.receipt.v2 JSON receipt for Cairn/Octet evidence flows.
Use --scenario valence-compat-bot-probe for a bounded one-client Valence probe with status/login/render milestones and safe non-load receipt fields. Use --scenario flag-score-repeat to require explicit protocol/login/render/team/flag/two-score milestones and forbidden-pattern checks. Use --scenario blue-flag-score to exercise the mirrored BLUE-team flag path. Use --scenario survival-break-place-pickup for the bounded survival fixture. Use --scenario survival-chest-persistence for the two-session chest open/store/close/reconnect/reopen probe. Use --scenario reconnect-flag-state to require disconnect/return state coherence while holding a flag. Use --scenario reconnect-flag-score to add reconnect evidence; use --scenario multi-client-load-score for two concurrent clients plus server-side correlation.\n\
Use --expect-status-description/--expect-status-version/--expect-status-sample to assert status response fixture data, --packet-capture-summary for redacted capture summary metadata, and --proxy-route/--proxy-forwarding-mode for proxied-route receipt fields.\n\
Use --compare-receipts PAPER_RECEIPT VALENCE_RECEIPT to check the fallback/control and default-backend receipts agree on protocol and headless isolation.\n\
Use --run-matrix --receipt-dir DIR to run Paper and Valence receipts then compare them; add --dry-run after --run-matrix for a non-side-effecting matrix fixture.\n\
Use --status to inspect harness-owned Paper/Valence/tmp state; use --cleanup --dry-run to preview cleanup and --cleanup --apply to remove it.\n\
Default server backend is Valence, using an editable local Valence checkout plus an isolated protocol-758 worktree so the dirty/current checkout is untouched.\n\
If the Stevenarella or Valence checkout is missing, clone/fetch it or pass --client-dir/CLIENT_DIR and --valence-repo/VALENCE_REPO to editable checkouts.\n\
Client runs are forced through Xvfb/X11 with software GL and no inherited Wayland socket.\n\
Paper fallback runs set EULA=TRUE based on recorded user acceptance.\n\n\
Env: MC_COMPAT_ROOT={} MC_COMPAT_CONFIG={} MC_COMPAT_STEEL_CONFIG={} MC_COMPAT_SCENARIO={} CLIENT_DIR={} TARGET_DIR={} SMOKE_RECEIPT={} SMOKE_RECEIPT_DIR={} VALENCE_REPO={} VALENCE_REV={} VALENCE_WORKTREE={} VALENCE_TARGET_DIR={} CLIENT_TIMEOUT={} PAPER_PLUGIN_JAR={}\n",
        SUPPORTED_SCENARIO_USAGE,
        cfg.server_version,
        cfg.server_protocol,
        cfg.root.display(),
        cfg.config_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        cfg.steel_config_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        scenario_name(cfg.scenario),
        cfg.client_dir.display(),
        cfg.target_dir.display(),
        cfg.receipt_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        cfg.receipt_dir
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string()),
        cfg.valence_repo.display(),
        cfg.valence_rev,
        cfg.valence_worktree.display(),
        cfg.valence_target_dir.display(),
        cfg.client_timeout.as_secs(),
        cfg.paper_plugin_jar
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<unset>".to_string())
    );
}

fn build_client(cfg: &Config) -> Result<(), String> {
    ensure_client_dir_ready(cfg)?;
    log(format_args!("building Stevenarella client"));
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&cfg.client_dir)
        .arg("build")
        .arg("--bin")
        .arg("stevenarella");
    apply_build_env(&mut cmd, &cfg.target_dir);
    run_cmd(cfg, &mut cmd)
}

fn ensure_client_dir_ready(cfg: &Config) -> Result<(), String> {
    if !cfg.client_dir.exists() {
        return Err(format!(
            "Stevenarella checkout not found at {}. Keep an editable sibling checkout with `git clone https://github.com/iceiix/stevenarella {}` or pass --client-dir/CLIENT_DIR to another checkout.",
            cfg.client_dir.display(),
            cfg.client_dir.display()
        ));
    }

    let manifest = cfg.client_dir.join("Cargo.toml");
    if !manifest.exists() {
        return Err(format!(
            "Stevenarella checkout {} is missing Cargo.toml. Point --client-dir/CLIENT_DIR at the Stevenarella repository root.",
            cfg.client_dir.display()
        ));
    }

    Ok(())
}

fn start_server(cfg: &Config) -> Result<ManagedServer, String> {
    match cfg.server_backend {
        ServerBackend::Valence => start_valence_server(cfg),
        ServerBackend::Paper => {
            start_paper_server(cfg)?;
            Ok(ManagedServer {
                child: None,
                pid_file: cfg.valence_pid_file.clone(),
                paper_container: Some(cfg.server_name.clone()),
                keep: cfg.keep_server || cfg.mode == Mode::DryRun,
            })
        }
    }
}

fn stop_server(cfg: &Config) -> Result<(), String> {
    stop_valence_server(cfg)?;
    log(format_args!(
        "stopping managed Paper container {}",
        cfg.server_name
    ));
    let mut cmd = Command::new("docker");
    cmd.arg("rm").arg("-f").arg(&cfg.server_name);
    run_cmd(cfg, &mut cmd)
}

fn print_harness_status(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "harness status for server '{}'",
        cfg.server_name
    ));
    let docker = docker_container_status(&cfg.server_name)?;
    println!("paper_container={docker}");

    let pid_state = valence_pid_state(&cfg.valence_pid_file)?;
    println!("valence_pid={pid_state}");
    println!(
        "valence_worktree={} exists={}",
        cfg.valence_worktree.display(),
        cfg.valence_worktree.exists()
    );
    println!(
        "valence_target_dir={} exists={}",
        cfg.valence_target_dir.display(),
        cfg.valence_target_dir.exists()
    );
    println!(
        "valence_log={} exists={}",
        cfg.valence_log.display(),
        cfg.valence_log.exists()
    );
    let logs = client_log_paths()?;
    println!("client_logs={}", logs.len());
    for path in logs.iter().take(20) {
        println!("client_log={}", path.display());
    }
    if logs.len() > 20 {
        println!("client_logs_omitted={}", logs.len() - 20);
    }
    Ok(())
}

fn cleanup_harness_state(cfg: &Config) -> Result<(), String> {
    let apply = cfg.cleanup_apply;
    if apply {
        log(format_args!("cleaning harness-owned state"));
    } else {
        log(format_args!(
            "cleanup dry-run; pass --cleanup --apply to remove harness-owned state"
        ));
    }

    cleanup_paper_container(&cfg.server_name, apply)?;
    cleanup_valence_pid(&cfg.valence_pid_file, apply)?;
    cleanup_path("valence target dir", &cfg.valence_target_dir, apply)?;
    cleanup_path("valence log", &cfg.valence_log, apply)?;
    for path in client_log_paths()? {
        cleanup_path("client log", &path, apply)?;
    }
    Ok(())
}

fn docker_container_status(name: &str) -> Result<String, String> {
    let output = Command::new("docker")
        .arg("ps")
        .arg("-a")
        .arg("--filter")
        .arg(format!("name={name}"))
        .arg("--format")
        .arg("{{.Names}} {{.Status}}")
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if text.is_empty() {
                Ok("absent".to_string())
            } else {
                Ok(text)
            }
        }
        Ok(out) => Ok(format!(
            "unavailable: docker ps exited {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr).trim()
        )),
        Err(err) => Ok(format!("unavailable: {err}")),
    }
}

fn cleanup_paper_container(name: &str, apply: bool) -> Result<(), String> {
    let state = docker_container_status(name)?;
    if state == "absent" || state.starts_with("unavailable:") {
        println!("cleanup paper_container {name}: {state}");
        return Ok(());
    }
    if apply {
        log(format_args!("removing Paper container {name}"));
        let status = Command::new("docker")
            .arg("rm")
            .arg("-f")
            .arg(name)
            .status()
            .map_err(|e| format!("docker rm -f {name}: {e}"))?;
        if !status.success() {
            return Err(format!("docker rm -f {name} failed with {status}"));
        }
    } else {
        println!("would remove Paper container {name}: {state}");
    }
    Ok(())
}

fn valence_pid_state(pid_file: &Path) -> Result<String, String> {
    let pid = match fs::read_to_string(pid_file) {
        Ok(pid) => pid.trim().to_string(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok("absent".to_string()),
        Err(err) => return Err(format!("read {}: {err}", pid_file.display())),
    };
    if pid.is_empty() {
        return Ok(format!("empty pid file {}", pid_file.display()));
    }
    let alive = Command::new("kill")
        .arg("-0")
        .arg(&pid)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    Ok(format!(
        "pid={} alive={} file={}",
        pid,
        alive,
        pid_file.display()
    ))
}

fn cleanup_valence_pid(pid_file: &Path, apply: bool) -> Result<(), String> {
    let pid = match fs::read_to_string(pid_file) {
        Ok(pid) => pid.trim().to_string(),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            println!("cleanup valence_pid_file {}: absent", pid_file.display());
            return Ok(());
        }
        Err(err) => return Err(format!("read {}: {err}", pid_file.display())),
    };
    if !pid.is_empty() {
        if apply {
            log(format_args!("stopping stale Valence process {pid}"));
            let _ = Command::new("kill").arg(&pid).status();
        } else {
            println!("would stop Valence process {pid}");
        }
    }
    if apply {
        fs::remove_file(pid_file).map_err(|e| format!("remove {}: {e}", pid_file.display()))?;
    } else {
        println!("would remove Valence pid file {}", pid_file.display());
    }
    Ok(())
}

fn cleanup_path(label: &str, path: &Path, apply: bool) -> Result<(), String> {
    if !path.exists() {
        println!("cleanup {label} {}: absent", path.display());
        return Ok(());
    }
    if apply {
        log(format_args!("removing {label} {}", path.display()));
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| format!("remove {}: {e}", path.display()))?;
        } else {
            fs::remove_file(path).map_err(|e| format!("remove {}: {e}", path.display()))?;
        }
    } else {
        println!("would remove {label} {}", path.display());
    }
    Ok(())
}

fn client_log_paths() -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    let entries = match fs::read_dir("/tmp") {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(paths),
        Err(err) => return Err(format!("read /tmp: {err}")),
    };
    for entry in entries {
        let entry = entry.map_err(|e| format!("read /tmp entry: {e}"))?;
        let name = entry.file_name();
        if is_mc_compat_client_log(&name.to_string_lossy()) {
            paths.push(entry.path());
        }
    }
    paths.sort();
    Ok(paths)
}

fn is_mc_compat_client_log(name: &str) -> bool {
    name.starts_with("mc-compat-client.") && name.ends_with(".log")
}

fn prepare_valence_worktree(cfg: &Config) -> Result<(), String> {
    ensure_valence_repo_ready(cfg)?;
    if !cfg.valence_worktree.join(".git").exists() {
        prune_stale_valence_worktrees(cfg)?;
        log(format_args!(
            "creating isolated Valence worktree {} at {}",
            cfg.valence_worktree.display(),
            cfg.valence_rev
        ));
        let mut cmd = Command::new("git");
        cmd.arg("-C")
            .arg(&cfg.valence_repo)
            .arg("worktree")
            .arg("add")
            .arg("--detach")
            .arg(&cfg.valence_worktree)
            .arg(&cfg.valence_rev);
        run_cmd(cfg, &mut cmd)?;
    } else {
        ensure_valence_worktree_at_requested_rev(cfg)?;
        log(format_args!(
            "using existing Valence worktree {}",
            cfg.valence_worktree.display()
        ));
    }
    Ok(())
}

fn ensure_valence_worktree_at_requested_rev(cfg: &Config) -> Result<(), String> {
    if cfg.mode == Mode::DryRun {
        return Ok(());
    }
    let current = git_rev_parse(&cfg.valence_worktree, "HEAD")?;
    let requested = git_rev_parse(
        &cfg.valence_repo,
        &format!("{}^{{commit}}", cfg.valence_rev),
    )?;
    if current == requested {
        return Ok(());
    }
    Err(format!(
        "Valence worktree {} is at {current}, but requested {} resolves to {requested}. Remove the stale worktree or pass VALENCE_WORKTREE to a fresh path.",
        cfg.valence_worktree.display(),
        cfg.valence_rev
    ))
}

fn git_rev_parse(repo: &Path, rev: &str) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("rev-parse")
        .arg(rev)
        .output()
        .map_err(|e| format!("git rev-parse {rev} in {}: {e}", repo.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git rev-parse {rev} in {} failed with {}",
            repo.display(),
            output.status
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_string())
        .map_err(|e| {
            format!(
                "git rev-parse {rev} output in {} was not UTF-8: {e}",
                repo.display()
            )
        })
}

fn git_worktree_dirty(repo: &Path) -> Result<bool, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .arg("status")
        .arg(GIT_STATUS_PORCELAIN_FLAG)
        .output()
        .map_err(|e| format!("git status in {}: {e}", repo.display()))?;
    if !output.status.success() {
        return Err(format!(
            "git status in {} failed with {}",
            repo.display(),
            output.status
        ));
    }
    String::from_utf8(output.stdout)
        .map(|text| !text.trim().is_empty())
        .map_err(|e| format!("git status output in {} was not UTF-8: {e}", repo.display()))
}

fn build_git_revision_evidence(
    requested_rev: Option<&str>,
    resolved_rev: Result<String, String>,
    dirty: Result<bool, String>,
) -> GitRevisionEvidence {
    match (resolved_rev, dirty) {
        (Ok(resolved_rev), Ok(dirty)) => GitRevisionEvidence {
            requested_rev: requested_rev.map(str::to_string),
            resolved_rev: Some(resolved_rev),
            status: if dirty {
                GIT_STATUS_DIRTY
            } else {
                GIT_STATUS_CLEAN
            },
            dirty,
            diagnostics: Vec::new(),
        },
        (resolved_rev, dirty) => {
            let mut diagnostics = Vec::new();
            if let Err(err) = resolved_rev {
                diagnostics.push(err);
            }
            if let Err(err) = dirty {
                diagnostics.push(err);
            }
            GitRevisionEvidence {
                requested_rev: requested_rev.map(str::to_string),
                resolved_rev: None,
                status: GIT_STATUS_UNAVAILABLE,
                dirty: true,
                diagnostics,
            }
        }
    }
}

fn git_revision_evidence(repo: &Path, requested_rev: Option<&str>) -> GitRevisionEvidence {
    build_git_revision_evidence(
        requested_rev,
        git_rev_parse(repo, "HEAD"),
        git_worktree_dirty(repo),
    )
}

fn child_revision_evidence_for_receipt(cfg: &Config) -> ChildRevisionEvidence {
    if cfg.mode == Mode::DryRun {
        return ChildRevisionEvidence {
            client: GitRevisionEvidence::dry_run(None),
            valence: GitRevisionEvidence::dry_run(Some(cfg.valence_rev.clone())),
        };
    }
    ChildRevisionEvidence {
        client: git_revision_evidence(&cfg.client_dir, None),
        valence: git_revision_evidence(&cfg.valence_worktree, Some(&cfg.valence_rev)),
    }
}

fn prune_stale_valence_worktrees(cfg: &Config) -> Result<(), String> {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(&cfg.valence_repo)
        .arg("worktree")
        .arg("prune");
    run_cmd(cfg, &mut cmd)
}

fn ensure_valence_repo_ready(cfg: &Config) -> Result<(), String> {
    if !cfg.valence_repo.exists() {
        return Err(format!(
            "Valence checkout not found at {}. Keep an editable sibling checkout with `git clone https://github.com/valence-rs/valence {}` or pass --valence-repo/VALENCE_REPO to another checkout.",
            cfg.valence_repo.display(),
            cfg.valence_repo.display()
        ));
    }
    if cfg.mode == Mode::DryRun {
        return Ok(());
    }

    let status = Command::new("git")
        .arg("-C")
        .arg(&cfg.valence_repo)
        .arg("rev-parse")
        .arg("--verify")
        .arg(format!("{}^{{commit}}", cfg.valence_rev))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|e| format!("check Valence checkout {}: {e}", cfg.valence_repo.display()))?;

    if !status.success() {
        return Err(format!(
            "Valence checkout {} does not contain compatible revision {}. Run `git -C {} fetch --all --tags` or pass --valence-repo/VALENCE_REPO to an editable checkout that has it.",
            cfg.valence_repo.display(),
            cfg.valence_rev,
            cfg.valence_repo.display()
        ));
    }

    Ok(())
}

fn start_valence_server(cfg: &Config) -> Result<ManagedServer, String> {
    prepare_valence_worktree(cfg)?;
    log(format_args!(
        "starting Valence {} example '{}' on 127.0.0.1:{}; log: {}",
        cfg.valence_rev,
        cfg.valence_example,
        cfg.server_port,
        cfg.valence_log.display()
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!(
            "would run Valence example from {}",
            cfg.valence_worktree.display()
        ));
        return Ok(ManagedServer {
            child: None,
            pid_file: cfg.valence_pid_file.clone(),
            paper_container: None,
            keep: true,
        });
    }
    if cfg.server_port != 25565 {
        log(format_args!(
            "warning: Valence revision {} defaults to 127.0.0.1:25565; SERVER_PORT={} may only work if the example overrides Config::address",
            cfg.valence_rev, cfg.server_port
        ));
    }
    stop_valence_server(cfg)?;
    let log_file = File::create(&cfg.valence_log)
        .map_err(|e| format!("create {}: {e}", cfg.valence_log.display()))?;
    let err_file = log_file
        .try_clone()
        .map_err(|e| format!("clone valence log handle: {e}"))?;
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&cfg.valence_worktree)
        .arg("run")
        .arg("--example")
        .arg(&cfg.valence_example)
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    cmd.env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", &cfg.valence_target_dir);
    if uses_armor_mitigation_probe(cfg.scenario) {
        cmd.env("MC_COMPAT_ARMOR_MITIGATION_PROBE", "1");
    }
    if matches!(
        cfg.scenario,
        Scenario::EquipmentUpdateObservation | Scenario::EquipmentSlotItemMatrixExpansion
    ) {
        cmd.env("MC_COMPAT_EQUIPMENT_UPDATE_PROBE", "1");
    }
    if matches!(
        cfg.scenario,
        Scenario::ProjectileHit | Scenario::ProjectileDamageAttribution
    ) {
        cmd.env("MC_COMPAT_PROJECTILE_PROBE", "1");
    }
    if cfg.scenario == Scenario::SurvivalChestPersistence {
        cmd.env(SURVIVAL_CHEST_FIXTURE_ENV, "1");
    }
    if let Some(path) = &cfg.steel_config_path {
        cmd.env("MC_COMPAT_STEEL_CONFIG", path);
    }
    let child = cmd.spawn().map_err(|e| format!("spawn Valence: {e}"))?;
    fs::write(&cfg.valence_pid_file, child.id().to_string())
        .map_err(|e| format!("write {}: {e}", cfg.valence_pid_file.display()))?;
    Ok(ManagedServer {
        child: Some(child),
        pid_file: cfg.valence_pid_file.clone(),
        paper_container: None,
        keep: cfg.keep_server,
    })
}

fn start_paper_server(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "starting Paper {} server on 127.0.0.1:{} via {} with EULA=TRUE",
        cfg.server_version, cfg.server_port, cfg.docker_image
    ));
    if cfg.mode == Mode::DryRun {
        let mut cmd = Command::new("docker");
        configure_paper_run_command(cfg, &mut cmd)?;
        return run_cmd(cfg, &mut cmd);
    }
    let _ = Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(&cfg.server_name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    let mut cmd = Command::new("docker");
    configure_paper_run_command(cfg, &mut cmd)?;
    run_cmd(cfg, &mut cmd)
}

fn configure_paper_run_command(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    cmd.arg("run")
        .arg("-d")
        .arg("--name")
        .arg(&cfg.server_name)
        .arg("-p")
        .arg(format!("127.0.0.1:{}:25565", cfg.server_port))
        .arg("-e")
        .arg("EULA=TRUE")
        .arg("-e")
        .arg("TYPE=PAPER")
        .arg("-e")
        .arg(format!("VERSION={}", cfg.server_version))
        .arg("-e")
        .arg("ONLINE_MODE=FALSE")
        .arg("-e")
        .arg("MEMORY=1G")
        .arg("-e")
        .arg(format!("VIEW_DISTANCE={PAPER_VIEW_DISTANCE}"))
        .arg("-e")
        .arg(format!("SIMULATION_DISTANCE={PAPER_SIMULATION_DISTANCE}"));
    if cfg.scenario == Scenario::SurvivalChestPersistence {
        cmd.arg("-e").arg(format!("{SURVIVAL_CHEST_FIXTURE_ENV}=1"));
    }
    add_paper_plugin_mount(cfg, cmd)?;
    cmd.arg(&cfg.docker_image);
    Ok(())
}

fn add_paper_plugin_mount(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    let Some(plugin_jar) = &cfg.paper_plugin_jar else {
        return Ok(());
    };
    let absolute_jar = fs::canonicalize(plugin_jar).map_err(|e| {
        format!(
            "canonicalize PAPER_PLUGIN_JAR {}: {e}",
            plugin_jar.display()
        )
    })?;
    let file_name = absolute_jar.file_name().ok_or_else(|| {
        format!(
            "PAPER_PLUGIN_JAR {} has no file name",
            absolute_jar.display()
        )
    })?;
    let container_path = Path::new(PAPER_PLUGIN_CONTAINER_DIR).join(file_name);
    cmd.arg("-v").arg(format!(
        "{}:{}:ro",
        absolute_jar.display(),
        container_path.display()
    ));
    Ok(())
}

fn stop_valence_server(cfg: &Config) -> Result<(), String> {
    if let Ok(pid) = fs::read_to_string(&cfg.valence_pid_file) {
        let pid = pid.trim();
        if !pid.is_empty() {
            log(format_args!(
                "stopping managed Valence server process {pid}"
            ));
            let _ = Command::new("kill").arg(pid).status();
        }
        fs::remove_file(&cfg.valence_pid_file)
            .map_err(|e| format!("remove {}: {e}", cfg.valence_pid_file.display()))?;
    }
    Ok(())
}

fn probe_status(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "probing status 127.0.0.1:{} expecting protocol {}",
        cfg.server_port, cfg.server_protocol
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!("would run Rust protocol status probe"));
        return Ok(());
    }
    let mut last = String::new();
    for _ in 0..90 {
        match read_status(cfg.server_port, cfg.server_protocol) {
            Ok(status) => {
                println!("{status}");
                let needle = format!("\"protocol\":{}", cfg.server_protocol);
                let spaced = format!("\"protocol\": {}", cfg.server_protocol);
                if status.contains(&needle) || status.contains(&spaced) {
                    assert_status_expectations(cfg, &status)?;
                    return Ok(());
                }
                return Err(format!(
                    "protocol mismatch in status response; expected {}",
                    cfg.server_protocol
                ));
            }
            Err(err) => last = err,
        }
        thread::sleep(Duration::from_secs(2));
    }
    Err(format!("server status probe failed: {last}"))
}

fn assert_status_expectations(cfg: &Config, status: &str) -> Result<(), String> {
    if let Some(expected) = &cfg.expected_status_description {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected description {expected:?}"
            ));
        }
    }
    if let Some(expected) = &cfg.expected_status_version_name {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected version {expected:?}"
            ));
        }
    }
    for expected in &cfg.expected_status_sample {
        if !status.contains(expected) {
            return Err(format!(
                "status response missing expected sample {expected:?}"
            ));
        }
    }
    Ok(())
}

fn read_status(port: u16, protocol: u32) -> Result<String, String> {
    let mut stream = TcpStream::connect(("127.0.0.1", port)).map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(2)))
        .map_err(|e| e.to_string())?;
    let mut payload = Vec::new();
    write_varint(protocol, &mut payload);
    write_string("127.0.0.1", &mut payload);
    payload.extend_from_slice(&port.to_be_bytes());
    write_varint(1, &mut payload);
    write_packet(0, &payload, &mut stream)?;
    write_packet(0, &[], &mut stream)?;
    let _packet_len = read_varint(&mut stream)?;
    let packet_id = read_varint(&mut stream)?;
    if packet_id != 0 {
        return Err(format!("unexpected status packet id {packet_id}"));
    }
    let string_len = read_varint(&mut stream)? as usize;
    let mut buf = vec![0; string_len];
    stream.read_exact(&mut buf).map_err(|e| e.to_string())?;
    String::from_utf8(buf).map_err(|e| e.to_string())
}

#[derive(Debug)]
struct SingleClientRun {
    username: String,
    log_path: PathBuf,
    exit_code: Option<i32>,
    output: String,
    matched_success_pattern: Option<String>,
}

fn run_client(cfg: &Config) -> Result<ClientRunEvidence, String> {
    log(format_args!(
        "running Stevenarella headless scenario '{}' isolated from host Wayland compositor",
        scenario_name(cfg.scenario)
    ));
    if cfg.mode == Mode::DryRun {
        log(format_args!("would run Stevenarella under xvfb-run"));
        if cfg.scenario == Scenario::ProjectileDamageAttribution {
            return Ok(projectile_damage_dry_run_evidence(cfg));
        }
        let scenario = evaluate_scenario_for_config(cfg, "");
        let server_scenario = Some(evaluate_server_scenario(
            cfg.scenario,
            "",
            &cfg.client_username,
        ));
        return Ok(ClientRunEvidence {
            log_path: None,
            log_paths: Vec::new(),
            usernames: planned_client_usernames(cfg),
            exit_code: None,
            classification: "dry-run",
            matched_success_pattern: None,
            scenario: Some(scenario),
            server_scenario,
            projectile_damage_causality: None,
        });
    }

    let runs = if matches!(
        cfg.scenario,
        Scenario::ReconnectFlagState
            | Scenario::SurvivalChestPersistence
            | Scenario::NegativeReconnectRace
    ) {
        run_reconnect_sequence_scenario(cfg)?
    } else if matches!(
        cfg.scenario,
        Scenario::MultiClientLoadScore
            | Scenario::CombatDamage
            | Scenario::CombatKnockback
            | Scenario::ArmorEquipmentMitigation
            | Scenario::ArmorLoadoutEnchantmentStatusMatrix
            | Scenario::EquipmentUpdateObservation
            | Scenario::EquipmentSlotItemMatrixExpansion
            | Scenario::ProjectileHit
            | Scenario::ProjectileDamageAttribution
            | Scenario::FlagCarrierDeathReturn
    ) {
        run_multi_client_load_scenario(cfg)?
    } else {
        vec![run_single_client(cfg, &cfg.client_username, 0)?]
    };

    let mut combined_output = String::new();
    if cfg.scenario == Scenario::MultiClientLoadScore && runs.len() >= 2 {
        combined_output.push_str("mc_compat_multi_client_count=2\n");
    }
    if matches!(
        cfg.scenario,
        Scenario::CombatDamage
            | Scenario::CombatKnockback
            | Scenario::ArmorEquipmentMitigation
            | Scenario::ArmorLoadoutEnchantmentStatusMatrix
    ) && runs.len() >= 2
    {
        combined_output.push_str("mc_compat_combat_client_count=2\n");
    }
    if cfg.scenario == Scenario::FlagCarrierDeathReturn && runs.len() >= 2 {
        combined_output.push_str("mc_compat_flag_carrier_death_client_count=2\n");
    }
    if matches!(
        cfg.scenario,
        Scenario::EquipmentUpdateObservation | Scenario::EquipmentSlotItemMatrixExpansion
    ) && runs.len() >= 2
    {
        combined_output.push_str("mc_compat_equipment_update_client_count=2\n");
    }
    if cfg.scenario == Scenario::ProjectileHit && runs.len() >= 2 {
        combined_output.push_str("mc_compat_projectile_hit_client_count=2\n");
    }
    if cfg.scenario == Scenario::ProjectileDamageAttribution && runs.len() >= 2 {
        combined_output.push_str("mc_compat_projectile_damage_client_count=2\n");
    }
    if matches!(
        cfg.scenario,
        Scenario::ReconnectFlagScore
            | Scenario::ReconnectFlagState
            | Scenario::SurvivalChestPersistence
            | Scenario::NegativeReconnectRace
    ) {
        combined_output.push_str("mc_compat_reconnect_session=2\n");
    }
    for run in &runs {
        combined_output.push_str(&run.output);
        if !combined_output.ends_with('\n') {
            combined_output.push('\n');
        }
    }
    print!("{combined_output}");
    io::stdout().flush().map_err(|e| e.to_string())?;

    let matched_success_pattern = cfg
        .client_success_needles
        .iter()
        .find(|needle| combined_output.contains(needle.as_str()))
        .cloned();
    let scenario = evaluate_scenario_for_config(cfg, &combined_output);
    if cfg.scenario != Scenario::Smoke && !scenario.passed {
        return Err(format!(
            "scenario {} failed: missing={:?} forbidden={:?}; logs={}",
            scenario_name(cfg.scenario),
            scenario.missing_milestones,
            scenario.forbidden_matches,
            runs.iter()
                .map(|run| run.log_path.display().to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
    }

    let server_scenario = read_server_scenario_evidence(cfg, &runs)?;
    if requires_server_correlation(cfg) {
        if let Some(server) = &server_scenario {
            if !server.passed {
                return Err(format!(
                    "server correlation for scenario {} failed: missing={:?} forbidden={:?}; log={}",
                    scenario_name(cfg.scenario),
                    server.missing_milestones,
                    server.forbidden_matches,
                    server_log_label(cfg)
                ));
            }
        }
    }

    let projectile_damage_causality = if cfg.scenario == Scenario::ProjectileDamageAttribution {
        let server_log = read_valence_log(cfg)?;
        let client_logs = runs
            .iter()
            .map(|run| ClientLogSlice {
                username: &run.username,
                output: &run.output,
            })
            .collect::<Vec<_>>();
        let expected_damage = projectile_damage_amount_needle(cfg);
        let causality = evaluate_projectile_damage_causality_for_damage(
            &client_logs,
            &server_log,
            &cfg.client_username,
            &expected_damage,
        );
        if !causality.passed {
            return Err(format!(
                "projectile damage causality failed: missing={:?} order_violations={:?}; client_logs={}; server_log={}",
                causality.missing_steps,
                causality.order_violations,
                runs.iter()
                    .map(|run| run.log_path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(","),
                cfg.valence_log.display()
            ));
        }
        Some(causality)
    } else {
        None
    };

    let all_success = runs.iter().all(|run| run.exit_code == Some(0));
    let timeout_success = runs
        .iter()
        .all(|run| run.exit_code == Some(124) && run.matched_success_pattern.is_some());
    let mixed_success = runs.iter().all(|run| {
        run.exit_code == Some(0)
            || (run.exit_code == Some(124) && run.matched_success_pattern.is_some())
    });
    let classification = if matches!(
        cfg.scenario,
        Scenario::MultiClientLoadScore
            | Scenario::CombatDamage
            | Scenario::CombatKnockback
            | Scenario::ArmorEquipmentMitigation
            | Scenario::ArmorLoadoutEnchantmentStatusMatrix
            | Scenario::EquipmentUpdateObservation
            | Scenario::EquipmentSlotItemMatrixExpansion
            | Scenario::ProjectileHit
            | Scenario::ProjectileDamageAttribution
            | Scenario::FlagCarrierDeathReturn
            | Scenario::ReconnectFlagState
            | Scenario::SurvivalChestPersistence
            | Scenario::NegativeReconnectRace
    ) && mixed_success
    {
        "multi-client-load-evidence"
    } else if all_success {
        "client-exited-success"
    } else if timeout_success {
        "timeout-success-evidence"
    } else {
        return Err(format!(
            "client scenario failed; exits={:?}; logs={}",
            runs.iter().map(|run| run.exit_code).collect::<Vec<_>>(),
            runs.iter()
                .map(|run| run.log_path.display().to_string())
                .collect::<Vec<_>>()
                .join(",")
        ));
    };

    for run in &runs {
        log(format_args!(
            "client {} finished {:?}; log: {}",
            run.username,
            run.exit_code,
            run.log_path.display()
        ));
    }
    let log_paths = runs
        .iter()
        .map(|run| run.log_path.clone())
        .collect::<Vec<_>>();
    let usernames = runs
        .iter()
        .map(|run| run.username.clone())
        .collect::<Vec<_>>();
    let evidence = ClientRunEvidence {
        log_path: log_paths.first().cloned(),
        log_paths,
        usernames,
        exit_code: runs.first().and_then(|run| run.exit_code),
        classification,
        matched_success_pattern,
        scenario: Some(scenario),
        server_scenario,
        projectile_damage_causality,
    };
    validate_typed_event_oracle_for_migrated_scenario(cfg, &evidence)?;
    Ok(evidence)
}

fn projectile_damage_dry_run_evidence(cfg: &Config) -> ClientRunEvidence {
    let attacker_username = format!(
        "{}{}",
        cfg.client_username, PROJECTILE_DAMAGE_ATTACKER_SUFFIX
    );
    let victim_username = format!("{}{}", cfg.client_username, PROJECTILE_DAMAGE_VICTIM_SUFFIX);
    let attacker_log = format!(
        "Detected server protocol version {}\njoin_game\nrender_tick_with_player\nYou are on team RED!\nremote_player_spawn\n{} hand=main {}\n{} hand=main\n",
        cfg.server_protocol,
        PROJECTILE_DAMAGE_CLIENT_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        PROJECTILE_DAMAGE_CLIENT_SWING_NEEDLE
    );
    let client_health_needle = projectile_damage_client_health_needle(cfg);
    let server_damage_needle = projectile_damage_amount_needle(cfg);
    let server_health_after_needle = projectile_damage_server_health_after_needle(cfg);
    let victim_log = format!(
        "Detected server protocol version {}\njoin_game\nrender_tick_with_player\nYou are on team BLUE!\nremote_player_spawn\n{}\n",
        cfg.server_protocol,
        client_health_needle
    );
    let server_log = format!(
        "{attacker_username} joined\n{victim_username} joined\nMC-COMPAT-MILESTONE projectile_loadout username={attacker_username} slot=0 item=Bow arrows=16\n{} attacker={attacker_username} victim={victim_username} hand=Main {} {}\n{} attacker={attacker_username} victim={victim_username} victim_health_before=20.0 {}\n",
        PROJECTILE_DAMAGE_SERVER_USE_NEEDLE,
        PROJECTILE_DAMAGE_SEQUENCE_NEEDLE,
        server_damage_needle,
        PROJECTILE_DAMAGE_SERVER_HIT_NEEDLE,
        server_health_after_needle
    );
    let combined_output =
        format!("mc_compat_projectile_damage_client_count=2\n{attacker_log}{victim_log}");
    let client_logs = [
        ClientLogSlice {
            username: &attacker_username,
            output: &attacker_log,
        },
        ClientLogSlice {
            username: &victim_username,
            output: &victim_log,
        },
    ];
    let scenario = evaluate_scenario_for_config(cfg, &combined_output);
    let server_scenario = evaluate_server_scenario(cfg.scenario, &server_log, &cfg.client_username);
    let projectile_damage_causality = evaluate_projectile_damage_causality_for_damage(
        &client_logs,
        &server_log,
        &cfg.client_username,
        &server_damage_needle,
    );
    ClientRunEvidence {
        log_path: None,
        log_paths: Vec::new(),
        usernames: vec![attacker_username, victim_username],
        exit_code: None,
        classification: "dry-run",
        matched_success_pattern: Some("Detected server protocol version".to_string()),
        scenario: Some(scenario),
        server_scenario: Some(server_scenario),
        projectile_damage_causality: Some(projectile_damage_causality),
    }
}

fn run_reconnect_sequence_scenario(cfg: &Config) -> Result<Vec<SingleClientRun>, String> {
    let username = cfg.client_username.clone();
    let scenario = scenario_name(cfg.scenario);
    let mut runs = Vec::new();
    for idx in 0..RECONNECT_SEQUENCE_SESSION_COUNT {
        let log_path = std::env::temp_dir().join(format!(
            "mc-compat-client.{username}.{scenario}-session-{}.{}.log",
            idx + 1,
            std::process::id()
        ));
        let mut child = spawn_client_process(cfg, &username, idx, &log_path)?;
        let status = child
            .wait()
            .map_err(|e| format!("wait {scenario} client session {}: {e}", idx + 1))?;
        let output = fs::read_to_string(&log_path)
            .map_err(|e| format!("read {}: {e}", log_path.display()))?;
        let matched_success_pattern = cfg
            .client_success_needles
            .iter()
            .find(|needle| output.contains(needle.as_str()))
            .cloned();
        runs.push(SingleClientRun {
            username: username.clone(),
            log_path,
            exit_code: status.code(),
            output,
            matched_success_pattern,
        });
        thread::sleep(Duration::from_secs(RECONNECT_SEQUENCE_PAUSE_SECS));
    }
    Ok(runs)
}

fn run_multi_client_load_scenario(cfg: &Config) -> Result<Vec<SingleClientRun>, String> {
    let usernames = planned_client_usernames(cfg);
    let mut children = Vec::new();
    for (idx, username) in usernames.iter().enumerate() {
        let log_path = temp_client_log_for(username);
        let child = spawn_client_process(cfg, username, idx, &log_path)?;
        children.push((username.clone(), log_path, child));
        thread::sleep(Duration::from_secs(2));
    }
    let mut runs = Vec::new();
    for (username, log_path, mut child) in children {
        let status = child
            .wait()
            .map_err(|e| format!("wait client {username}: {e}"))?;
        let output = fs::read_to_string(&log_path)
            .map_err(|e| format!("read {}: {e}", log_path.display()))?;
        let matched_success_pattern = cfg
            .client_success_needles
            .iter()
            .find(|needle| output.contains(needle.as_str()))
            .cloned();
        runs.push(SingleClientRun {
            username,
            log_path,
            exit_code: status.code(),
            output,
            matched_success_pattern,
        });
    }
    Ok(runs)
}

fn run_single_client(
    cfg: &Config,
    username: &str,
    client_index: usize,
) -> Result<SingleClientRun, String> {
    let log_path = env_path("CLIENT_LOG").unwrap_or_else(|| temp_client_log_for(username));
    let mut child = spawn_client_process(cfg, username, client_index, &log_path)?;
    let status = child.wait().map_err(|e| format!("wait client: {e}"))?;
    let output =
        fs::read_to_string(&log_path).map_err(|e| format!("read {}: {e}", log_path.display()))?;
    let matched_success_pattern = cfg
        .client_success_needles
        .iter()
        .find(|needle| output.contains(needle.as_str()))
        .cloned();
    Ok(SingleClientRun {
        username: username.to_string(),
        log_path,
        exit_code: status.code(),
        output,
        matched_success_pattern,
    })
}

fn spawn_client_process(
    cfg: &Config,
    username: &str,
    client_index: usize,
    log_path: &Path,
) -> Result<Child, String> {
    let log_file =
        File::create(log_path).map_err(|e| format!("create {}: {e}", log_path.display()))?;
    let err_file = log_file
        .try_clone()
        .map_err(|e| format!("clone client log handle: {e}"))?;
    let mut cmd = Command::new("timeout");
    cmd.arg(client_timeout_secs(cfg, client_index).to_string())
        .arg("xvfb-run")
        .arg("-a")
        .arg("-s")
        .arg("-screen 0 1280x720x24 +extension GLX +render -noreset")
        .arg(cfg.target_dir.join("debug/stevenarella"))
        .arg("--server")
        .arg(format!("127.0.0.1:{}", cfg.server_port))
        .arg("--username")
        .arg(username)
        .arg("--default-protocol-version")
        .arg(cfg.server_protocol.to_string())
        .stdout(Stdio::from(log_file))
        .stderr(Stdio::from(err_file));
    apply_build_env(&mut cmd, &cfg.target_dir);
    apply_headless_env(&mut cmd);
    apply_scenario_probe_env(&mut cmd, cfg.scenario, client_index);
    cmd.spawn()
        .map_err(|e| format!("run client {username}: {e}"))
}

fn client_timeout_secs(cfg: &Config, client_index: usize) -> u64 {
    if cfg.scenario == Scenario::MultiClientLoadScore && client_index > 0 {
        cfg.client_timeout
            .as_secs()
            .min(MULTI_CLIENT_LOAD_PEER_TIMEOUT_SECS)
    } else {
        cfg.client_timeout.as_secs()
    }
}

fn apply_scenario_probe_env(cmd: &mut Command, scenario: Scenario, client_index: usize) {
    match scenario {
        Scenario::Smoke => {}
        Scenario::CompatBotProbe => {
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1");
        }
        Scenario::FlagScoreRepeat | Scenario::BlueFlagScore | Scenario::ReconnectFlagScore => {
            let team = if scenario == Scenario::BlueFlagScore {
                "blue"
            } else {
                "red"
            };
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE_TEAM", team)
                .env("MC_COMPAT_FLAG_PROBE", "1")
                .env("MC_COMPAT_FLAG_PROBE_TEAM", team)
                .env("MC_COMPAT_FLAG_PROBE_REPEAT", "2");
            if scenario == Scenario::ReconnectFlagScore {
                cmd.env("MC_COMPAT_RECONNECT_PROBE", "1");
            }
        }
        Scenario::ReconnectFlagState | Scenario::NegativeReconnectRace => {
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE_TEAM", "red");
            if client_index == 0 {
                cmd.env("MC_COMPAT_FLAG_PROBE", "1")
                    .env("MC_COMPAT_FLAG_PROBE_TEAM", "red")
                    .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", "1")
                    .env("MC_COMPAT_FLAG_PROBE_REPEAT", "1");
            }
            if scenario == Scenario::NegativeReconnectRace {
                cmd.env("MC_COMPAT_NEGATIVE_PROBE", "reconnect_race");
            }
        }
        Scenario::InventoryInteraction => {
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE_TEAM", "red")
                .env("MC_COMPAT_INVENTORY_PROBE", "1");
        }
        Scenario::NegativeInventoryStaleState | Scenario::NegativeInventoryInvalidClick => {
            let negative_probe = match scenario {
                Scenario::NegativeInventoryStaleState => "inventory_stale_state",
                Scenario::NegativeInventoryInvalidClick => "inventory_invalid_click",
                _ => "",
            };
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE_TEAM", "red")
                .env("MC_COMPAT_INVENTORY_PROBE", "1")
                .env("MC_COMPAT_NEGATIVE_PROBE", negative_probe);
        }
        Scenario::NegativeCustomPayload => {
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_NEGATIVE_PROBE", "custom_payload_malformed");
        }
        Scenario::SurvivalBreakPlacePickup => {
            cmd.env("MC_COMPAT_SURVIVAL_PROBE", "1");
        }
        Scenario::SurvivalChestPersistence => {
            cmd.env("MC_COMPAT_SURVIVAL_CHEST_PROBE", "1").env(
                "MC_COMPAT_SURVIVAL_CHEST_SESSION",
                (client_index + 1).to_string(),
            );
        }
        Scenario::EquipmentUpdateObservation | Scenario::EquipmentSlotItemMatrixExpansion => {
            let team = if client_index == 0 { "red" } else { "blue" };
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE_TEAM", team)
                .env("MC_COMPAT_EQUIPMENT_PROBE", "1");
        }
        Scenario::ProjectileHit | Scenario::ProjectileDamageAttribution => {
            let (team, role) = if client_index == 0 {
                ("red", "attacker")
            } else {
                ("blue", "victim")
            };
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE_TEAM", team)
                .env("MC_COMPAT_COMBAT_PROBE", "1")
                .env("MC_COMPAT_COMBAT_PROBE_ROLE", role)
                .env("MC_COMPAT_PROJECTILE_PROBE", "1");
            if role == "attacker" {
                cmd.env("MC_COMPAT_COMBAT_TARGET_USERNAME", "compatbotb");
            }
        }
        Scenario::CombatDamage
        | Scenario::CombatKnockback
        | Scenario::ArmorEquipmentMitigation
        | Scenario::ArmorLoadoutEnchantmentStatusMatrix
        | Scenario::FlagCarrierDeathReturn => {
            let (team, role) = if client_index == 0 {
                ("red", "attacker")
            } else {
                ("blue", "victim")
            };
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE_TEAM", team)
                .env("MC_COMPAT_COMBAT_PROBE", "1")
                .env("MC_COMPAT_COMBAT_PROBE_ROLE", role);
            if role == "attacker" {
                cmd.env("MC_COMPAT_COMBAT_TARGET_USERNAME", "compatbotb");
            }
            if uses_armor_mitigation_probe(scenario) {
                cmd.env("MC_COMPAT_ARMOR_MITIGATION_PROBE", "1");
                if role == "victim" {
                    cmd.env("MC_COMPAT_INVENTORY_PROBE", "1");
                }
            }
            if scenario == Scenario::FlagCarrierDeathReturn {
                cmd.env("MC_COMPAT_FLAG_CARRIER_DEATH_PROBE", "1")
                    .env("MC_COMPAT_RESPAWN_PROBE", "1");
                if client_index == 1 {
                    cmd.env("MC_COMPAT_FLAG_PROBE", "1")
                        .env("MC_COMPAT_FLAG_PROBE_TEAM", "blue")
                        .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", "1")
                        .env("MC_COMPAT_FLAG_PROBE_FIRST_TICK", "760")
                        .env("MC_COMPAT_FLAG_PROBE_REPEAT", "1");
                }
            }
        }
        Scenario::MultiClientLoadScore => {
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1");
            if client_index == 0 {
                cmd.env("MC_COMPAT_TEAM_PROBE", "1")
                    .env("MC_COMPAT_TEAM_PROBE_TEAM", "red")
                    .env("MC_COMPAT_FLAG_PROBE", "1")
                    .env("MC_COMPAT_FLAG_PROBE_REPEAT", "1");
            }
        }
        Scenario::NegativeCtfWrongScore => {
            cmd.env("MC_COMPAT_ACTIVE_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE", "1")
                .env("MC_COMPAT_TEAM_PROBE_TEAM", "red")
                .env("MC_COMPAT_FLAG_PROBE", "1")
                .env("MC_COMPAT_FLAG_PROBE_TEAM", "blue")
                .env("MC_COMPAT_FLAG_PROBE_PICKUP_ONLY", "1")
                .env("MC_COMPAT_NEGATIVE_PROBE", "ctf_wrong_score");
        }
    }
}

fn planned_client_usernames(cfg: &Config) -> Vec<String> {
    if matches!(
        cfg.scenario,
        Scenario::MultiClientLoadScore
            | Scenario::CombatDamage
            | Scenario::CombatKnockback
            | Scenario::ArmorEquipmentMitigation
            | Scenario::ArmorLoadoutEnchantmentStatusMatrix
            | Scenario::EquipmentUpdateObservation
            | Scenario::EquipmentSlotItemMatrixExpansion
            | Scenario::ProjectileHit
            | Scenario::ProjectileDamageAttribution
            | Scenario::FlagCarrierDeathReturn
    ) {
        vec![
            format!("{}a", cfg.client_username),
            format!("{}b", cfg.client_username),
        ]
    } else {
        vec![cfg.client_username.clone()]
    }
}

fn server_log_label(cfg: &Config) -> String {
    match cfg.server_backend {
        ServerBackend::Valence => cfg.valence_log.display().to_string(),
        ServerBackend::Paper => format!("docker logs {}", cfg.server_name),
    }
}

fn read_server_scenario_evidence(
    cfg: &Config,
    runs: &[SingleClientRun],
) -> Result<Option<ServerScenarioEvidence>, String> {
    let server_log = match cfg.server_backend {
        ServerBackend::Valence => read_valence_log(cfg)?,
        ServerBackend::Paper => read_paper_log(cfg)?,
    };
    let mut correlation_log = server_log;
    for run in runs {
        correlation_log.push('\n');
        correlation_log.push_str(&run.output);
    }
    let username = &cfg.client_username;
    Ok(Some(evaluate_server_scenario(
        cfg.scenario,
        &correlation_log,
        username,
    )))
}

fn read_valence_log(cfg: &Config) -> Result<String, String> {
    match fs::read_to_string(&cfg.valence_log) {
        Ok(text) => Ok(text),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(err) => Err(format!("read {}: {err}", cfg.valence_log.display())),
    }
}

fn read_paper_log(cfg: &Config) -> Result<String, String> {
    if cfg.mode == Mode::DryRun {
        return Ok(String::new());
    }
    let output = Command::new("docker")
        .arg("logs")
        .arg(&cfg.server_name)
        .output()
        .map_err(|e| format!("docker logs {}: {e}", cfg.server_name))?;
    let mut text = String::new();
    text.push_str(&String::from_utf8_lossy(&output.stdout));
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    Ok(text)
}

fn requires_server_correlation(cfg: &Config) -> bool {
    matches!(
        cfg.scenario,
        Scenario::FlagScoreRepeat
            | Scenario::ReconnectFlagScore
            | Scenario::MultiClientLoadScore
            | Scenario::InventoryInteraction
            | Scenario::SurvivalBreakPlacePickup
            | Scenario::SurvivalChestPersistence
            | Scenario::CombatDamage
            | Scenario::CombatKnockback
            | Scenario::ArmorEquipmentMitigation
            | Scenario::ArmorLoadoutEnchantmentStatusMatrix
            | Scenario::EquipmentUpdateObservation
            | Scenario::EquipmentSlotItemMatrixExpansion
            | Scenario::ProjectileHit
            | Scenario::ProjectileDamageAttribution
            | Scenario::FlagCarrierDeathReturn
    )
}

fn write_smoke_receipt(
    cfg: &Config,
    result: Result<&Option<ClientRunEvidence>, &String>,
) -> Result<(), String> {
    let Some(path) = &cfg.receipt_path else {
        return Ok(());
    };
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|e| format!("create receipt dir {}: {e}", parent.display()))?;
    }
    let client = match result {
        Ok(client) => client.as_ref(),
        Err(_) => None,
    };
    let typed_event_oracle = write_typed_event_oracle_artifact(cfg, client, path)?;
    let json = smoke_receipt_json_with_typed_event_oracle(
        cfg,
        result.map_err(|err| err.as_str()),
        typed_event_oracle.as_ref(),
    );
    fs::write(path, json).map_err(|e| format!("write receipt {}: {e}", path.display()))?;
    log(format_args!("wrote smoke receipt {}", path.display()));
    Ok(())
}

fn write_typed_event_oracle_artifact(
    cfg: &Config,
    client: Option<&ClientRunEvidence>,
    receipt_path: &Path,
) -> Result<Option<TypedEventOracleArtifact>, String> {
    if cfg.mode != Mode::Run {
        return Ok(None);
    }
    let Some(client) = client else {
        return Ok(None);
    };
    let events = typed_events_from_receipt_evidence(cfg, client)?;
    if events.is_empty() {
        return Ok(None);
    }
    let timeline = normalize_typed_event_timeline(&events);
    let timeline_blake3 = typed_event_timeline_blake3(&timeline);
    let event_log_path = typed_event_log_path_for_receipt(receipt_path);
    fs::write(&event_log_path, timeline)
        .map_err(|err| format!("write typed event log {}: {err}", event_log_path.display()))?;
    log(format_args!(
        "wrote typed event log {}",
        event_log_path.display()
    ));
    Ok(Some(TypedEventOracleArtifact {
        event_log_path,
        timeline_blake3,
        event_count: events.len(),
        contributes_to_pass_fail: typed_event_oracle_contributes_to_pass_fail(cfg.scenario),
    }))
}

fn typed_event_log_path_for_receipt(receipt_path: &Path) -> PathBuf {
    receipt_path.with_extension(TYPED_EVENT_LOG_EXTENSION)
}

const LATENCY_JITTER_ENABLED_ENV: &str = "MC_COMPAT_LATENCY_JITTER_ENABLED";
const LATENCY_JITTER_TARGET_RAIL_ENV: &str = "MC_COMPAT_LATENCY_JITTER_TARGET_RAIL";
const LATENCY_JITTER_MECHANISM_ENV: &str = "MC_COMPAT_LATENCY_JITTER_MECHANISM";
const LATENCY_MS_ENV: &str = "MC_COMPAT_LATENCY_MS";
const JITTER_MS_ENV: &str = "MC_COMPAT_JITTER_MS";
const LOSS_PERCENT_ENV: &str = "MC_COMPAT_LOSS_PERCENT";
const WAN_TARGET_OWNERSHIP_ENV: &str = "MC_COMPAT_WAN_TARGET_OWNERSHIP";
const WAN_AUTHORIZATION_ENV: &str = "MC_COMPAT_WAN_AUTHORIZATION";
const LATENCY_JITTER_ENV_ENABLED_VALUE: &str = "1";
const LATENCY_JITTER_DEFAULT_METRIC: &str = "0";
const LATENCY_JITTER_DEFAULT_MECHANISM: &str = "bounded-client-cadence";
const LATENCY_JITTER_ENABLED_HYGIENE_STATUS: &str = "bounded-local-fixture";
const LATENCY_JITTER_DISABLED_HYGIENE_STATUS: &str = "not-selected";
const WAN_TARGET_OWNERSHIP_OWNED_LOCAL: &str = "owned-local-loopback";
const WAN_AUTHORIZATION_OWNED_LOCAL: &str = "owned-local-fixture-approved";
const WAN_PASS_FAIL_CRITERIA: &str = "inventory_interaction_client_server_milestones";
const WAN_ABORT_REASON_NONE: &str = "none";
const NO_RECONNECT_SESSIONS: u32 = 0;
const SINGLE_RECONNECT_SESSION: u32 = 1;
const WAN_TELEMETRY_SAMPLES: &[&str] = &[
    "scenario_required_milestones",
    "scenario_observed_milestones",
    "server_required_milestones",
    "server_observed_milestones",
    "client_classification",
    "triage_boundary",
];

#[derive(Debug, Clone, PartialEq, Eq)]
struct LatencyJitterTelemetryReceipt {
    selected: bool,
    mechanism: String,
    target_rail: String,
    delay_ms: String,
    jitter_ms: String,
    loss_percent: String,
    timeout_secs: u64,
    duration_secs: u64,
    client_count: usize,
    reconnect_count: u32,
    target_ownership: String,
    authorization: String,
    hygiene_status: &'static str,
}

fn latency_jitter_receipt_json(cfg: &Config) -> String {
    let receipt = latency_jitter_receipt_from_config(cfg);
    render_latency_jitter_receipt_json(&receipt)
}

fn latency_jitter_receipt_from_config(cfg: &Config) -> LatencyJitterTelemetryReceipt {
    let selected = std::env::var(LATENCY_JITTER_ENABLED_ENV).unwrap_or_default()
        == LATENCY_JITTER_ENV_ENABLED_VALUE;
    let target_rail = std::env::var(LATENCY_JITTER_TARGET_RAIL_ENV)
        .unwrap_or_else(|_| scenario_name(cfg.scenario).to_string());
    let delay_ms =
        std::env::var(LATENCY_MS_ENV).unwrap_or_else(|_| LATENCY_JITTER_DEFAULT_METRIC.to_string());
    let jitter_ms =
        std::env::var(JITTER_MS_ENV).unwrap_or_else(|_| LATENCY_JITTER_DEFAULT_METRIC.to_string());
    let loss_percent = std::env::var(LOSS_PERCENT_ENV)
        .unwrap_or_else(|_| LATENCY_JITTER_DEFAULT_METRIC.to_string());
    let mechanism = std::env::var(LATENCY_JITTER_MECHANISM_ENV)
        .unwrap_or_else(|_| LATENCY_JITTER_DEFAULT_MECHANISM.to_string());
    let target_ownership = std::env::var(WAN_TARGET_OWNERSHIP_ENV)
        .unwrap_or_else(|_| WAN_TARGET_OWNERSHIP_OWNED_LOCAL.to_string());
    let authorization = std::env::var(WAN_AUTHORIZATION_ENV)
        .unwrap_or_else(|_| WAN_AUTHORIZATION_OWNED_LOCAL.to_string());
    let hygiene_status = if selected {
        LATENCY_JITTER_ENABLED_HYGIENE_STATUS
    } else {
        LATENCY_JITTER_DISABLED_HYGIENE_STATUS
    };
    LatencyJitterTelemetryReceipt {
        selected,
        mechanism,
        target_rail,
        delay_ms,
        jitter_ms,
        loss_percent,
        timeout_secs: cfg.client_timeout.as_secs(),
        duration_secs: cfg.client_timeout.as_secs(),
        client_count: planned_client_usernames(cfg).len(),
        reconnect_count: latency_jitter_reconnect_count(cfg.scenario),
        target_ownership,
        authorization,
        hygiene_status,
    }
}

fn latency_jitter_reconnect_count(scenario: Scenario) -> u32 {
    if matches!(
        scenario,
        Scenario::ReconnectFlagState
            | Scenario::ReconnectFlagScore
            | Scenario::NegativeReconnectRace
    ) {
        SINGLE_RECONNECT_SESSION
    } else {
        NO_RECONNECT_SESSIONS
    }
}

fn render_latency_jitter_receipt_json(receipt: &LatencyJitterTelemetryReceipt) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "mechanism": {mechanism},
    "target_rail": {target_rail},
    "delay_ms": {delay_ms},
    "jitter_ms": {jitter_ms},
    "loss_percent": {loss_percent},
    "timeout_secs": {timeout_secs},
    "duration_secs": {duration_secs},
    "client_count": {client_count},
    "reconnect_count": {reconnect_count},
    "target_ownership": {target_ownership},
    "authorization": {authorization},
    "telemetry_samples": {telemetry_samples},
    "pass_fail_criteria": {pass_fail_criteria},
    "abort_reason": {abort_reason},
    "hygiene_status": {hygiene_status},
    "privileged_network_mutation_required": false,
    "fail_closed_when_unavailable": true,
    "claims_wan_safety": false,
    "claims_packet_loss_tolerance": false,
    "claims_internet_path_safety": false,
    "claims_adversarial_network_safety": false,
    "claims_public_server_safety": false,
    "claims_production_readiness": false
  }}"#,
        selected = if receipt.selected { "true" } else { "false" },
        mechanism = json_string(&receipt.mechanism),
        target_rail = json_string(&receipt.target_rail),
        delay_ms = json_string(&receipt.delay_ms),
        jitter_ms = json_string(&receipt.jitter_ms),
        loss_percent = json_string(&receipt.loss_percent),
        timeout_secs = receipt.timeout_secs,
        duration_secs = receipt.duration_secs,
        client_count = receipt.client_count,
        reconnect_count = receipt.reconnect_count,
        target_ownership = json_string(&receipt.target_ownership),
        authorization = json_string(&receipt.authorization),
        telemetry_samples = json_string_array(WAN_TELEMETRY_SAMPLES),
        pass_fail_criteria = json_string(WAN_PASS_FAIL_CRITERIA),
        abort_reason = json_string(WAN_ABORT_REASON_NONE),
        hygiene_status = json_string(receipt.hygiene_status),
    )
}

fn render_negative_live_rail_json(evidence: &NegativeLiveRailEvidence) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "rail": {rail},
    "invalid_action": {invalid_action},
    "expected_outcome": {expected_outcome},
    "observed_outcome": {observed_outcome},
    "observed_outcome_source": {observed_outcome_source},
    "postcondition_milestone": {postcondition_milestone},
    "telemetry_present": {telemetry_present},
    "target_scope": {target_scope},
    "owned_local_target": {owned_local_target},
    "explicit_authorization": {explicit_authorization},
    "public_target": {public_target},
    "planned_clients": {planned_clients},
    "max_clients": {max_clients},
    "timeout_secs": {timeout_secs},
    "evidence_fields": {evidence_fields},
    "missing_fields": {missing_fields},
    "bound_violations": {bound_violations},
    "preflight_passed": {preflight_passed},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        rail = json_optional_string(evidence.rail),
        invalid_action = json_optional_string(evidence.invalid_action),
        expected_outcome = json_optional_string(evidence.expected_outcome),
        observed_outcome = json_optional_string(evidence.observed_outcome),
        observed_outcome_source = json_optional_string(evidence.observed_outcome_source.as_deref()),
        postcondition_milestone = json_optional_string(evidence.postcondition_milestone),
        telemetry_present = evidence.telemetry_present,
        target_scope = json_string(evidence.target_scope),
        owned_local_target = evidence.owned_local_target,
        explicit_authorization = evidence.explicit_authorization,
        public_target = evidence.public_target,
        planned_clients = evidence.planned_clients,
        max_clients = evidence.max_clients,
        timeout_secs = evidence.timeout_secs,
        evidence_fields = json_string_array(NEGATIVE_LIVE_RAIL_EVIDENCE_FIELDS),
        missing_fields = json_string_array(&evidence.missing_fields),
        bound_violations = json_string_array(&evidence.bound_violations),
        preflight_passed = evidence.preflight_passed,
        non_claims = json_string_array(NEGATIVE_LIVE_RAIL_NON_CLAIMS),
    )
}

fn render_armor_loadout_enchantment_status_matrix_json(
    evidence: &ArmorLoadoutEnchantmentStatusMatrixEvidence,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "row_id": {row_id},
    "loadout_id": {loadout_id},
    "equipment_slots": {equipment_slots},
    "enchantments": {enchantments},
    "status_effects": {status_effects},
    "attack_type": {attack_type},
    "reference_required": {reference_required},
    "reference_receipt": {reference_receipt},
    "live_receipt": {live_receipt},
    "promotion_ready": {promotion_ready},
    "required_client_milestones": {required_client_milestones},
    "observed_client_milestones": {observed_client_milestones},
    "required_server_milestones": {required_server_milestones},
    "observed_server_milestones": {observed_server_milestones},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        row_id = json_string(evidence.row_id),
        loadout_id = json_string(evidence.loadout_id),
        equipment_slots = json_string_array(&evidence.equipment_slots),
        enchantments = json_string_array(&evidence.enchantments),
        status_effects = json_string_array(&evidence.status_effects),
        attack_type = json_string(evidence.attack_type),
        reference_required = evidence.reference_required,
        reference_receipt = json_string(evidence.reference_receipt),
        live_receipt = evidence.live_receipt,
        promotion_ready = evidence.promotion_ready,
        required_client_milestones = json_string_array(&evidence.required_client_milestones),
        observed_client_milestones = json_string_array(&evidence.observed_client_milestones),
        required_server_milestones = json_string_array(&evidence.required_server_milestones),
        observed_server_milestones = json_string_array(&evidence.observed_server_milestones),
        non_claims = json_string_array(&evidence.non_claims),
    )
}

fn render_equipment_slot_item_matrix_expansion_json(
    evidence: &EquipmentSlotItemMatrixExpansionEvidence,
) -> String {
    format!(
        r#"{{
    "selected": {selected},
    "row_id": {row_id},
    "actor_username": {actor_username},
    "observer_username": {observer_username},
    "remote_entity_id": {remote_entity_id},
    "semantic_slot": {semantic_slot},
    "wire_slot": {wire_slot},
    "item_id": {item_id},
    "item_count": {item_count},
    "transition_kind": {transition_kind},
    "update_order": {update_order},
    "reference_required": {reference_required},
    "reference_receipt": {reference_receipt},
    "live_receipt": {live_receipt},
    "promotion_ready": {promotion_ready},
    "required_client_milestones": {required_client_milestones},
    "observed_client_milestones": {observed_client_milestones},
    "required_server_milestones": {required_server_milestones},
    "observed_server_milestones": {observed_server_milestones},
    "non_claims": {non_claims}
  }}"#,
        selected = evidence.selected,
        row_id = json_string(evidence.row_id),
        actor_username = json_string(evidence.actor_username),
        observer_username = json_string(evidence.observer_username),
        remote_entity_id = json_string(evidence.remote_entity_id),
        semantic_slot = json_string(evidence.semantic_slot),
        wire_slot = json_string(evidence.wire_slot),
        item_id = json_string(evidence.item_id),
        item_count = json_string(evidence.item_count),
        transition_kind = json_string(evidence.transition_kind),
        update_order = json_string(evidence.update_order),
        reference_required = evidence.reference_required,
        reference_receipt = json_string(evidence.reference_receipt),
        live_receipt = evidence.live_receipt,
        promotion_ready = evidence.promotion_ready,
        required_client_milestones = json_string_array(&evidence.required_client_milestones),
        observed_client_milestones = json_string_array(&evidence.observed_client_milestones),
        required_server_milestones = json_string_array(&evidence.required_server_milestones),
        observed_server_milestones = json_string_array(&evidence.observed_server_milestones),
        non_claims = json_string_array(&evidence.non_claims),
    )
}

fn render_load_network_safety_json(evidence: &LoadNetworkSafetyEvidence) -> String {
    format!(
        r#"{{
    "target_scope": {target_scope},
    "owned_local_target": {owned_local_target},
    "explicit_authorization": {explicit_authorization},
    "public_target": {public_target},
    "authorized": {authorized},
    "planned_clients": {planned_clients},
    "max_clients": {max_clients},
    "duration_secs": {duration_secs},
    "max_duration_secs": {max_duration_secs},
    "reconnect_sessions": {reconnect_sessions},
    "latency_ms": {latency_ms},
    "jitter_ms": {jitter_ms},
    "loss_percent": {loss_percent},
    "telemetry_present": {telemetry_present},
    "live_receipt": {live_receipt},
    "missing_fields": {missing_fields},
    "bound_violations": {bound_violations},
    "preflight_passed": {preflight_passed},
    "promotion_ready": {promotion_ready},
    "claims_public_server_safety": false,
    "claims_production_readiness": false,
    "claims_unbounded_soak": false,
    "claims_unbounded_reconnect": false,
    "claims_wan_safety": false,
    "claims_adversarial_network_safety": false
  }}"#,
        target_scope = json_string(evidence.target_scope),
        owned_local_target = evidence.owned_local_target,
        explicit_authorization = evidence.explicit_authorization,
        public_target = evidence.public_target,
        authorized = evidence.authorized,
        planned_clients = evidence.planned_clients,
        max_clients = evidence.max_clients,
        duration_secs = evidence.duration_secs,
        max_duration_secs = evidence.max_duration_secs,
        reconnect_sessions = evidence.reconnect_sessions,
        latency_ms = json_string(&evidence.latency_ms),
        jitter_ms = json_string(&evidence.jitter_ms),
        loss_percent = json_string(&evidence.loss_percent),
        telemetry_present = evidence.telemetry_present,
        live_receipt = evidence.live_receipt,
        missing_fields = json_string_array(&evidence.missing_fields),
        bound_violations = json_string_array(&evidence.bound_violations),
        preflight_passed = evidence.preflight_passed,
        promotion_ready = evidence.promotion_ready,
    )
}

fn smoke_receipt_json(cfg: &Config, result: Result<&Option<ClientRunEvidence>, &str>) -> String {
    smoke_receipt_json_with_typed_event_oracle(cfg, result, None)
}

fn smoke_receipt_json_with_typed_event_oracle(
    cfg: &Config,
    result: Result<&Option<ClientRunEvidence>, &str>,
    typed_event_oracle: Option<&TypedEventOracleArtifact>,
) -> String {
    let status = if result.is_ok() { "pass" } else { "fail" };
    let error = result.err();
    let client = result.ok().and_then(|client| client.as_ref());
    let receipt_path = cfg
        .receipt_path
        .as_ref()
        .map(|path| path.display().to_string());
    let client_log_path = client
        .and_then(|evidence| evidence.log_path.as_ref())
        .map(|path| path.display().to_string());
    let client_log_paths = client
        .map(|evidence| {
            evidence
                .log_paths
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let client_usernames = client
        .map(|evidence| evidence.usernames.clone())
        .unwrap_or_else(|| planned_client_usernames(cfg));
    let matched_pattern = client.and_then(|evidence| evidence.matched_success_pattern.as_deref());
    let classification = client.map(|evidence| evidence.classification);
    let exit_code = client.and_then(|evidence| evidence.exit_code);
    let scenario_evidence = client.and_then(|evidence| evidence.scenario.as_ref());
    let fallback_scenario = evaluate_scenario_for_config(cfg, "");
    let scenario = scenario_evidence.unwrap_or(&fallback_scenario);
    let server_evidence = client.and_then(|evidence| evidence.server_scenario.as_ref());
    let fallback_server = evaluate_server_scenario(cfg.scenario, "", &cfg.client_username);
    let server_scenario = server_evidence.unwrap_or(&fallback_server);
    let projectile_damage_causality =
        client.and_then(|evidence| evidence.projectile_damage_causality.as_ref());
    let fallback_projectile_damage_causality =
        evaluate_projectile_damage_causality(&[], "", &cfg.client_username);
    let selected_projectile_damage_causality =
        if cfg.scenario == Scenario::ProjectileDamageAttribution {
            Some(projectile_damage_causality.unwrap_or(&fallback_projectile_damage_causality))
        } else {
            None
        };
    let projectile_damage_causality_passed = selected_projectile_damage_causality
        .map(|evidence| evidence.passed)
        .unwrap_or(true);
    let projectile_damage_causality_json = projectile_damage_causality_json(
        cfg.scenario == Scenario::ProjectileDamageAttribution,
        selected_projectile_damage_causality,
    );
    let scenario_required: Vec<&str> = scenario_required_milestones(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let scenario_forbidden: Vec<&str> = scenario_forbidden_patterns(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let server_required: Vec<&str> = server_required_milestones(cfg.scenario)
        .iter()
        .map(|(name, _)| *name)
        .collect();
    let compat_bot_probe_selected = cfg.scenario == Scenario::CompatBotProbe;
    let compat_bot_target_address = format!("127.0.0.1:{}", cfg.server_port);
    let compat_bot_planned_clients = planned_client_usernames(cfg).len();
    let first_missing_client = scenario.missing_milestones.first().copied();
    let first_missing_server = server_scenario.missing_milestones.first().copied();
    let (first_forbidden_source, first_forbidden_pattern) =
        first_forbidden_match(scenario, server_scenario);
    let suggested_boundary = suggested_triage_boundary(
        error.is_some(),
        client.is_some(),
        first_missing_client,
        first_missing_server,
        first_forbidden_pattern,
        requires_server_correlation(cfg),
    );
    let enriched_triage = build_enriched_triage(
        scenario,
        server_scenario,
        scenario_name(cfg.scenario),
        &client_usernames,
        error,
        first_missing_client,
        first_missing_server,
        first_forbidden_source,
        first_forbidden_pattern,
        suggested_boundary,
    );
    let enriched_triage_json = enriched_triage_json(&enriched_triage);
    let status_sample_json = json_string_vec(&cfg.expected_status_sample);
    let status_resource_configured = cfg.expected_status_description.is_some()
        || cfg.expected_status_version_name.is_some()
        || !cfg.expected_status_sample.is_empty();
    let packet_capture_selected = cfg.packet_capture_summary;
    let packet_capture_expected_packets: Vec<&str> = match cfg.scenario {
        Scenario::Smoke => vec!["status_response", "login_or_timeout"],
        Scenario::CompatBotProbe => vec!["status_response", "login_success", "play_join_game"],
        Scenario::FlagScoreRepeat | Scenario::BlueFlagScore => {
            vec!["login_success", "play_join_game", "chat_scoreboard"]
        }
        Scenario::InventoryInteraction => vec![
            "login_success",
            "play_join_game",
            "inventory_set_slot",
            "player_action_drop_item",
            "open_container",
            "player_window_click",
            "player_block_placement",
        ],
        Scenario::SurvivalBreakPlacePickup => vec![
            "login_success",
            "play_join_game",
            "player_action_break_block",
            "block_update",
            "inventory_pickup",
            "player_block_placement",
        ],
        Scenario::SurvivalChestPersistence => vec![
            "login_success",
            "play_join_game",
            "open_container",
            "player_window_click",
            "close_window",
            "disconnect_reconnect",
        ],
        Scenario::CombatDamage => vec!["two_client_login", "play_join_game", "use_entity_attack"],
        Scenario::CombatKnockback => vec![
            "two_client_login",
            "play_join_game",
            "use_entity_attack",
            "entity_velocity",
        ],
        Scenario::ArmorEquipmentMitigation | Scenario::ArmorLoadoutEnchantmentStatusMatrix => vec![
            "two_client_login",
            "play_join_game",
            "inventory_set_slot",
            "use_entity_attack",
            "armor_mitigation",
        ],
        Scenario::EquipmentUpdateObservation | Scenario::EquipmentSlotItemMatrixExpansion => vec![
            "two_client_login",
            "play_join_game",
            "entity_equipment_update",
        ],
        Scenario::ProjectileHit => vec![
            "two_client_login",
            "play_join_game",
            "projectile_use_item",
            "projectile_hit_attribution",
        ],
        Scenario::ProjectileDamageAttribution => vec![
            "two_client_login",
            "play_join_game",
            "projectile_use_item",
            "projectile_hit_attribution",
            "health_update",
        ],
        Scenario::FlagCarrierDeathReturn => vec![
            "two_client_login",
            "play_join_game",
            "flag_pickup",
            "use_entity_attack",
            "health_death",
            "respawn_request",
        ],
        Scenario::ReconnectFlagState => vec![
            "login_success",
            "play_join_game",
            "flag_pickup",
            "disconnect_reconnect",
            "flag_state_reset",
        ],
        Scenario::ReconnectFlagScore => vec![
            "login_success",
            "play_join_game",
            "disconnect_reconnect",
            "chat_scoreboard",
        ],
        Scenario::MultiClientLoadScore => {
            vec!["two_client_login", "play_join_game", "chat_scoreboard"]
        }
        Scenario::NegativeInventoryStaleState => vec![
            "login_success",
            "play_join_game",
            "inventory_click_stale_state",
        ],
        Scenario::NegativeInventoryInvalidClick => vec![
            "login_success",
            "play_join_game",
            "inventory_click_invalid_slot",
        ],
        Scenario::NegativeCustomPayload => vec![
            "login_success",
            "play_join_game",
            "custom_payload_malformed",
        ],
        Scenario::NegativeReconnectRace => vec![
            "login_success",
            "play_join_game",
            "disconnect_reconnect",
            "flag_state_race",
        ],
        Scenario::NegativeCtfWrongScore => {
            vec!["login_success", "play_join_game", "wrong_score_path"]
        }
    };
    let typed_event_oracle_json = typed_event_oracle_receipt_json(typed_event_oracle);
    let latency_jitter_json = latency_jitter_receipt_json(cfg);
    let load_network_safety = evaluate_load_network_safety(load_network_safety_inputs(
        cfg,
        client.is_some() && server_scenario.passed,
        matches!(cfg.mode, Mode::Run),
    ));
    let load_network_safety_json = render_load_network_safety_json(&load_network_safety);
    let negative_live_rail =
        evaluate_negative_live_rail_safety_from_inputs(negative_live_rail_inputs_from_config(
            cfg,
            Some(scenario),
            matches!(cfg.mode, Mode::Run) && is_negative_live_rail(cfg.scenario),
        ));
    let negative_live_rail_json = render_negative_live_rail_json(&negative_live_rail);
    let armor_loadout_enchantment_status_matrix =
        evaluate_armor_loadout_enchantment_status_matrix(cfg, scenario, server_scenario);
    let armor_loadout_enchantment_status_matrix_json =
        render_armor_loadout_enchantment_status_matrix_json(
            &armor_loadout_enchantment_status_matrix,
        );
    let equipment_slot_item_matrix_expansion =
        evaluate_equipment_slot_item_matrix_expansion(cfg, scenario, server_scenario);
    let equipment_slot_item_matrix_expansion_json =
        render_equipment_slot_item_matrix_expansion_json(&equipment_slot_item_matrix_expansion);
    let proxy_route = cfg.proxy_route.as_deref().unwrap_or("direct");
    let proxy_forwarding_mode = cfg.proxy_forwarding_mode.as_deref().unwrap_or("none");
    let proxy_selected = cfg.proxy_route.is_some();
    let gameplay_oracle_milestones: Vec<&str> = vec![
        "protocol_detected",
        "join_game",
        "render_tick",
        "team_red",
        "team_blue",
        "flag_pickup",
        "flag_capture",
        "score_red_1",
        "score_red_2",
        "score_blue_1",
        "inventory_slot_update",
        "inventory_sword_slot",
        "inventory_wool_slot",
        "inventory_drop_sent",
        "inventory_pickup_seen",
        "inventory_click_sent",
        "inventory_open_container_seen",
        "inventory_container_click_sent",
        "inventory_block_place_sent",
        "survival_break_sent",
        "survival_break_update",
        "survival_pickup_seen",
        "survival_place_sent",
        "survival_place_update",
        "server_survival_join",
        "server_survival_break",
        "server_survival_pickup",
        "server_survival_place",
        "survival_chest_open_seen",
        "survival_chest_store_sent",
        "survival_chest_close_sent",
        "survival_chest_reconnect_sent",
        "survival_chest_reopen_seen",
        "survival_chest_persisted_seen",
        "server_survival_chest_open",
        "server_survival_chest_store",
        "server_survival_chest_close",
        "server_survival_chest_reopen",
        "server_survival_chest_persisted",
        "server_inventory_hotbar_select",
        "server_inventory_drop",
        "server_inventory_pickup",
        "server_inventory_click",
        "server_inventory_open_container",
        "server_inventory_container_click",
        "server_block_place",
        "reconnect_session",
        "multi_client_count",
        "remote_player_spawn",
        "combat_attack_sent",
        "combat_health_update",
        "combat_velocity_update",
        "armor_inventory_slot",
        "entity_equipment_update",
        "server_equipment_state",
        "server_equipment_update_state",
        "server_armor_mitigation",
        "server_combat_damage",
        "server_combat_knockback",
        "projectile_use_sent",
        "projectile_swing_sent",
        "projectile_damage_update",
        "server_projectile_use",
        "server_projectile_hit",
        "flag_carrier_death",
        "flag_return",
        "flag_disconnect_return",
        "reconnect_state_coherent",
    ];
    let gameplay_non_claims: Vec<&str> = vec![
        "full_ctf_correctness",
        "full_survival_compatibility",
        "vanilla_parity",
        "broad_minecraft_compatibility",
        "unbounded_soak",
        "production_load",
        "full_projectile_physics",
        "all_projectile_weapons",
        "enchantments_or_status_effects",
    ];
    let child_revisions = child_revision_evidence_for_receipt(cfg);
    let client_git_rev_json = json_optional_string(child_revisions.client.resolved_rev.as_deref());
    let client_git_status_json = json_string(child_revisions.client.status);
    let client_git_diagnostics_json = json_string_vec(&child_revisions.client.diagnostics);
    let valence_git_rev_requested_json =
        json_optional_string(child_revisions.valence.requested_rev.as_deref());
    let valence_git_rev_resolved_json =
        json_optional_string(child_revisions.valence.resolved_rev.as_deref());
    let valence_git_status_json = json_string(child_revisions.valence.status);
    let valence_git_diagnostics_json = json_string_vec(&child_revisions.valence.diagnostics);
    let error_json = error
        .map(|err| json_string(err))
        .unwrap_or_else(|| "null".to_string());
    let receipt_path_json = json_optional_string(receipt_path.as_deref());
    let client_log_json = json_optional_string(client_log_path.as_deref());
    let client_logs_json = json_string_vec(&client_log_paths);
    let client_usernames_json = json_string_vec(&client_usernames);
    let server_log_json = json_string(&server_log_label(cfg));
    let matched_pattern_json = json_optional_string(matched_pattern);
    let classification_json = json_optional_string(classification);
    let exit_code_json = exit_code
        .map(|code| code.to_string())
        .unwrap_or_else(|| "null".to_string());

    format!(
        r#"{{
  "schema": "mc.compat.scenario.receipt.v2",
  "legacy_schema": "mc.compat.smoke.receipt.v1",
  "status": {status_json},
  "mode": {mode_json},
  "dry_run": {dry_run},
  "contract": {{
    "cairn_contract": "mc.compat.scenario.receipt.v2",
    "legacy_cairn_contract": "mc.compat.smoke.receipt.v1",
    "octet_producer_surface": "tools/mc-compat-runner/src/main.rs",
    "claims_correctness": false,
    "claims_semantic_equivalence": false
  }},
  "scenario": {{
    "name": {scenario_name_json},
    "required_milestones": {scenario_required_json},
    "observed_milestones": {scenario_observed_json},
    "missing_milestones": {scenario_missing_json},
    "forbidden_patterns": {scenario_forbidden_json},
    "forbidden_matches": {scenario_forbidden_matches_json},
    "passed": {scenario_passed}
  }},
  "compat_bot_probe": {{
    "selected": {compat_bot_probe_selected},
    "safe_bounded_probe": true,
    "target_address": {compat_bot_target_address_json},
    "owned_local_target_required": true,
    "external_server_load_authorized": false,
    "public_stress_tool": false,
    "planned_clients": {compat_bot_planned_clients},
    "max_clients": 1,
    "duration_secs": {timeout_secs},
    "derived_from": "hyperion/tools/rust-mc-bot pattern"
  }},
  "status_response_resource": {{
    "resource_owned": true,
    "configured": {status_resource_configured},
    "expected_description": {expected_status_description_json},
    "expected_version_name": {expected_status_version_name_json},
    "expected_player_sample": {status_sample_json},
    "defaults_stable": true,
    "asserted_by_status_probe": {status_resource_configured}
  }},
  "packet_capture_oracle": {{
    "selected": {packet_capture_selected},
    "headless_cli": true,
    "redacted_receipt": true,
    "raw_payloads_recorded": false,
    "normalized_fields": ["direction", "state", "packet_id", "decode_status"],
    "expected_summary_packets": {packet_capture_expected_packets_json},
    "triage_correlation": true
  }},
  "typed_event_oracle": {typed_event_oracle_json},
  "latency_jitter_tolerance": {latency_jitter_json},
  "load_network_safety": {load_network_safety_json},
  "negative_live_rail": {negative_live_rail_json},
  "proxy_compat_seam": {{
    "selected": {proxy_selected},
    "route": {proxy_route_json},
    "forwarding_mode": {proxy_forwarding_mode_json},
    "direct_and_proxied_claims_separated": true,
    "mtls_ported": false,
    "credentials_recorded": false,
    "owned_local_proxy_required": true
  }},
  "gameplay_oracles": {{
    "derived_from": "hyperion gameplay milestone vocabulary",
    "selected_scenario": {scenario_name_json},
    "catalog": {gameplay_oracle_milestones_json},
    "requires_client_and_server_evidence_for_semantic_claims": true,
    "non_claims": {gameplay_non_claims_json}
  }},
  "armor_loadout_enchantment_status_matrix": {armor_loadout_enchantment_status_matrix_json},
  "equipment_slot_item_matrix_expansion": {equipment_slot_item_matrix_expansion_json},
  "server": {{
    "backend": {backend_json},
    "version": {version_json},
    "protocol": {protocol},
    "port": {port},
    "required_milestones": {server_required_json},
    "observed_milestones": {server_observed_json},
    "missing_milestones": {server_missing_json},
    "forbidden_matches": {server_forbidden_matches_json},
    "passed": {server_passed},
    "client_server_correlation": {{
      "scenario": {scenario_name_json},
      "usernames": {client_usernames_json},
      "passed": {correlation_passed}
    }}
  }},
  "projectile_damage_causality": {projectile_damage_causality_json},
  "client": {{
    "dir": {client_dir_json},
    "git_rev": {client_git_rev_json},
    "git_status": {client_git_status_json},
    "git_dirty": {client_git_dirty},
    "git_diagnostics": {client_git_diagnostics_json},
    "target_dir": {target_dir_json},
    "username": {username_json},
    "usernames": {client_usernames_json},
    "timeout_secs": {timeout_secs},
    "headless_isolation": {{
      "xvfb": true,
      "x11_backend": true,
      "software_gl": true,
      "wayland_socket_inherited": false
    }},
    "log_path": {client_log_json},
    "log_paths": {client_logs_json},
    "exit_code": {exit_code_json},
    "classification": {classification_json},
    "matched_success_pattern": {matched_pattern_json}
  }},
  "valence": {{
    "repo": {valence_repo_json},
    "rev": {valence_rev_json},
    "git_rev_requested": {valence_git_rev_requested_json},
    "git_rev_resolved": {valence_git_rev_resolved_json},
    "git_status": {valence_git_status_json},
    "git_dirty": {valence_git_dirty},
    "git_diagnostics": {valence_git_diagnostics_json},
    "worktree": {valence_worktree_json},
    "example": {valence_example_json},
    "log_path": {valence_log_json}
  }},
  "triage": {{
    "first_missing_client_milestone": {first_missing_client_json},
    "first_missing_server_milestone": {first_missing_server_json},
    "first_forbidden_pattern": {first_forbidden_pattern_json},
    "first_forbidden_source": {first_forbidden_source_json},
    "client_log_paths": {client_logs_json},
    "server_log_path": {server_log_json},
    "suggested_boundary": {suggested_boundary_json},
    "enriched": {enriched_triage_json}
  }},
  "receipt_path": {receipt_path_json},
  "error": {error_json}
}}
"#,
        status_json = json_string(status),
        mode_json = json_string(mode_name(cfg.mode)),
        dry_run = cfg.mode == Mode::DryRun,
        scenario_name_json = json_string(scenario_name(cfg.scenario)),
        scenario_required_json = json_string_array(&scenario_required),
        scenario_observed_json = json_string_array(&scenario.observed_milestones),
        scenario_missing_json = json_string_array(&scenario.missing_milestones),
        scenario_forbidden_json = json_string_array(&scenario_forbidden),
        scenario_forbidden_matches_json = json_string_array(&scenario.forbidden_matches),
        scenario_passed = scenario.passed,
        compat_bot_probe_selected = compat_bot_probe_selected,
        compat_bot_target_address_json = json_string(&compat_bot_target_address),
        compat_bot_planned_clients = compat_bot_planned_clients,
        status_resource_configured = status_resource_configured,
        expected_status_description_json =
            json_optional_string(cfg.expected_status_description.as_deref()),
        expected_status_version_name_json =
            json_optional_string(cfg.expected_status_version_name.as_deref()),
        status_sample_json = status_sample_json,
        packet_capture_selected = packet_capture_selected,
        packet_capture_expected_packets_json = json_string_array(&packet_capture_expected_packets),
        typed_event_oracle_json = typed_event_oracle_json,
        load_network_safety_json = load_network_safety_json,
        negative_live_rail_json = negative_live_rail_json,
        armor_loadout_enchantment_status_matrix_json = armor_loadout_enchantment_status_matrix_json,
        equipment_slot_item_matrix_expansion_json = equipment_slot_item_matrix_expansion_json,
        proxy_selected = proxy_selected,
        proxy_route_json = json_string(proxy_route),
        proxy_forwarding_mode_json = json_string(proxy_forwarding_mode),
        gameplay_oracle_milestones_json = json_string_array(&gameplay_oracle_milestones),
        gameplay_non_claims_json = json_string_array(&gameplay_non_claims),
        server_required_json = json_string_array(&server_required),
        server_observed_json = json_string_array(&server_scenario.observed_milestones),
        server_missing_json = json_string_array(&server_scenario.missing_milestones),
        server_forbidden_matches_json = json_string_array(&server_scenario.forbidden_matches),
        server_passed = server_scenario.passed,
        correlation_passed =
            scenario.passed && server_scenario.passed && projectile_damage_causality_passed,
        projectile_damage_causality_json = projectile_damage_causality_json,
        backend_json = json_string(backend_name(cfg.server_backend)),
        version_json = json_string(&cfg.server_version),
        protocol = cfg.server_protocol,
        port = cfg.server_port,
        client_dir_json = json_string(&cfg.client_dir.display().to_string()),
        client_git_rev_json = client_git_rev_json,
        client_git_status_json = client_git_status_json,
        client_git_dirty = child_revisions.client.dirty,
        client_git_diagnostics_json = client_git_diagnostics_json,
        target_dir_json = json_string(&cfg.target_dir.display().to_string()),
        username_json = json_string(&cfg.client_username),
        client_usernames_json = client_usernames_json,
        client_logs_json = client_logs_json,
        client_log_json = client_log_json,
        matched_pattern_json = matched_pattern_json,
        classification_json = classification_json,
        exit_code_json = exit_code_json,
        timeout_secs = cfg.client_timeout.as_secs(),
        valence_repo_json = json_string(&cfg.valence_repo.display().to_string()),
        valence_rev_json = json_string(&cfg.valence_rev),
        valence_git_rev_requested_json = valence_git_rev_requested_json,
        valence_git_rev_resolved_json = valence_git_rev_resolved_json,
        valence_git_status_json = valence_git_status_json,
        valence_git_dirty = child_revisions.valence.dirty,
        valence_git_diagnostics_json = valence_git_diagnostics_json,
        valence_worktree_json = json_string(&cfg.valence_worktree.display().to_string()),
        valence_example_json = json_string(&cfg.valence_example),
        valence_log_json = json_string(&cfg.valence_log.display().to_string()),
        server_log_json = server_log_json,
        receipt_path_json = receipt_path_json,
        error_json = error_json,
        first_missing_client_json = json_optional_string(first_missing_client),
        first_missing_server_json = json_optional_string(first_missing_server),
        first_forbidden_pattern_json = json_optional_string(first_forbidden_pattern),
        first_forbidden_source_json = json_optional_string(first_forbidden_source),
        suggested_boundary_json = json_string(suggested_boundary),
        enriched_triage_json = enriched_triage_json,
    )
}

fn projectile_damage_causality_json(
    selected: bool,
    evidence: Option<&ProjectileDamageCausalityEvidence>,
) -> String {
    let Some(evidence) = evidence else {
        return format!(
            r#"{{
    "selected": {selected},
    "attacker": null,
    "victim": null,
    "required_steps": [],
    "observed_steps": [],
    "missing_steps": [],
    "order_violations": [],
    "proof_basis": "not-selected",
    "passed": {passed}
  }}"#,
            selected = selected,
            passed = !selected,
        );
    };
    format!(
        r#"{{
    "selected": {selected},
    "attacker": {attacker_json},
    "victim": {victim_json},
    "required_steps": {required_steps_json},
    "observed_steps": {observed_steps_json},
    "missing_steps": {missing_steps_json},
    "order_violations": {order_violations_json},
    "proof_basis": "attacker_client_packet_send_plus_valence_attacker_victim_health_delta_plus_victim_client_health_update",
    "passed": {passed}
  }}"#,
        selected = selected,
        attacker_json = json_string(&evidence.attacker_username),
        victim_json = json_string(&evidence.victim_username),
        required_steps_json = json_string_array(&evidence.required_steps),
        observed_steps_json = json_string_array(&evidence.observed_steps),
        missing_steps_json = json_string_array(&evidence.missing_steps),
        order_violations_json = json_string_array(&evidence.order_violations),
        passed = evidence.passed,
    )
}

fn first_forbidden_match<'a>(
    scenario: &'a ScenarioEvidence,
    server_scenario: &'a ServerScenarioEvidence,
) -> (Option<&'static str>, Option<&'a str>) {
    if let Some(pattern) = scenario.forbidden_matches.first() {
        (Some("client"), Some(*pattern))
    } else if let Some(pattern) = server_scenario.forbidden_matches.first() {
        (Some("server"), Some(*pattern))
    } else {
        (None, None)
    }
}

fn suggested_triage_boundary(
    has_error: bool,
    has_client_evidence: bool,
    first_missing_client: Option<&str>,
    first_missing_server: Option<&str>,
    first_forbidden_pattern: Option<&str>,
    requires_server_correlation: bool,
) -> &'static str {
    if has_error && !has_client_evidence {
        "preflight-or-server-startup"
    } else if first_forbidden_pattern.is_some() {
        "protocol-runtime"
    } else if first_missing_client.is_some() {
        "client-probe"
    } else if requires_server_correlation && first_missing_server.is_some() {
        "server-correlation"
    } else if has_error {
        "runner-error"
    } else {
        "none"
    }
}

fn build_enriched_triage(
    scenario: &ScenarioEvidence,
    server_scenario: &ServerScenarioEvidence,
    scenario_name: &str,
    usernames: &[String],
    error: Option<&str>,
    first_missing_client: Option<&str>,
    first_missing_server: Option<&str>,
    first_forbidden_source: Option<&str>,
    first_forbidden_pattern: Option<&str>,
    suggested_boundary: &str,
) -> EnrichedTriage {
    let last_client_event = scenario
        .observed_milestones
        .last()
        .map(|name| (*name).to_string());
    let last_server_event = server_scenario
        .observed_milestones
        .last()
        .map(|name| (*name).to_string());
    let mut correlation_ids = vec![format!("scenario={scenario_name}")];
    correlation_ids.extend(usernames.iter().map(|username| format!("user={username}")));

    let mut timeline_excerpt = Vec::new();
    push_triage_excerpt(
        &mut timeline_excerpt,
        format!("boundary={suggested_boundary}"),
    );
    if let Some(error) = error {
        push_triage_excerpt(&mut timeline_excerpt, format!("error={error}"));
    }
    if let Some(milestone) = first_missing_client {
        push_triage_excerpt(&mut timeline_excerpt, format!("missing_client={milestone}"));
    }
    if let Some(milestone) = first_missing_server {
        push_triage_excerpt(&mut timeline_excerpt, format!("missing_server={milestone}"));
    }
    if let Some(pattern) = first_forbidden_pattern {
        let source = first_forbidden_source.unwrap_or("unknown");
        push_triage_excerpt(
            &mut timeline_excerpt,
            format!("forbidden_{source}={pattern}"),
        );
    }
    if timeline_excerpt.is_empty() {
        push_triage_excerpt(&mut timeline_excerpt, "boundary=none".to_string());
    }

    EnrichedTriage {
        last_client_event,
        last_server_event,
        correlation_ids,
        timeline_excerpt,
        boundary_confidence: triage_boundary_confidence(suggested_boundary),
    }
}

fn push_triage_excerpt(lines: &mut Vec<String>, line: String) {
    if lines.len() >= TRIAGE_MAX_TIMELINE_EVENTS {
        return;
    }
    lines.push(bound_redacted_excerpt(&line));
}

fn triage_boundary_confidence(boundary: &str) -> &'static str {
    match boundary {
        "none" => TRIAGE_CONFIDENCE_NONE,
        "client-probe" | "runner-error" => TRIAGE_CONFIDENCE_MEDIUM,
        _ => TRIAGE_CONFIDENCE_HIGH,
    }
}

fn bound_redacted_excerpt(line: &str) -> String {
    let redacted = redact_sensitive_excerpt(line);
    if redacted.chars().count() <= TRIAGE_MAX_EXCERPT_CHARS {
        return redacted;
    }
    redacted.chars().take(TRIAGE_MAX_EXCERPT_CHARS).collect()
}

fn redact_sensitive_excerpt(line: &str) -> String {
    line.split_whitespace()
        .map(redact_sensitive_token)
        .collect::<Vec<_>>()
        .join(" ")
}

fn redact_sensitive_token(token: &str) -> &str {
    let lower = token.to_ascii_lowercase();
    if lower.contains("token=")
        || lower.contains("secret=")
        || lower.contains("password=")
        || token.starts_with('/')
    {
        TRIAGE_REDACTED
    } else {
        token
    }
}

fn enriched_triage_json(triage: &EnrichedTriage) -> String {
    format!(
        r#"{{
    "last_client_event": {last_client_event_json},
    "last_server_event": {last_server_event_json},
    "correlation_ids": {correlation_ids_json},
    "timeline_excerpt": {timeline_excerpt_json},
    "boundary_confidence": {boundary_confidence_json}
  }}"#,
        last_client_event_json = json_optional_string(triage.last_client_event.as_deref()),
        last_server_event_json = json_optional_string(triage.last_server_event.as_deref()),
        correlation_ids_json = json_string_vec(&triage.correlation_ids),
        timeline_excerpt_json = json_string_vec(&triage.timeline_excerpt),
        boundary_confidence_json = json_string(triage.boundary_confidence),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ReceiptSummary {
    path: PathBuf,
    schema: String,
    status: String,
    dry_run: bool,
    backend: String,
    protocol: u32,
    port: u16,
    classification: String,
    matched_success_pattern: Option<String>,
    xvfb: bool,
    x11_backend: bool,
    software_gl: bool,
    wayland_socket_inherited: bool,
}

fn run_matrix(cfg: &Config) -> Result<(), String> {
    let receipt_dir = cfg
        .receipt_dir
        .clone()
        .unwrap_or_else(|| cfg.root.join("target/mc-compat-matrix"));
    fs::create_dir_all(&receipt_dir)
        .map_err(|e| format!("create receipt dir {}: {e}", receipt_dir.display()))?;

    let paper_receipt = receipt_dir.join("paper.json");
    let valence_receipt = receipt_dir.join("valence.json");
    let matrix_mode = if cfg.matrix_dry_run { "dry-run" } else { "run" };
    log(format_args!(
        "starting {matrix_mode} matrix: paper receipt={} valence receipt={}",
        paper_receipt.display(),
        valence_receipt.display()
    ));

    let paper_cfg = matrix_backend_config(cfg, ServerBackend::Paper, paper_receipt.clone());
    run_matrix_backend(&paper_cfg)?;

    let valence_cfg = matrix_backend_config(cfg, ServerBackend::Valence, valence_receipt.clone());
    run_matrix_backend(&valence_cfg)?;

    let paper = read_receipt_summary(&paper_receipt)?;
    let valence = read_receipt_summary(&valence_receipt)?;
    validate_receipt_pair(&paper, &valence, cfg.server_protocol)?;
    println!(
        "[mc-compat] matrix passed: paper={} valence={} protocol={} mode={matrix_mode}",
        paper_receipt.display(),
        valence_receipt.display(),
        paper.protocol
    );
    Ok(())
}

fn matrix_backend_config(cfg: &Config, backend: ServerBackend, receipt_path: PathBuf) -> Config {
    let mut backend_cfg = cfg.clone();
    backend_cfg.mode = if cfg.matrix_dry_run {
        Mode::DryRun
    } else {
        Mode::Run
    };
    backend_cfg.server_backend = backend;
    backend_cfg.server_port = default_port(backend);
    backend_cfg.receipt_path = Some(receipt_path);
    backend_cfg.receipt_dir = None;
    backend_cfg.compare_receipts = None;
    backend_cfg.keep_server = false;
    backend_cfg
}

fn run_matrix_backend(cfg: &Config) -> Result<(), String> {
    log(format_args!(
        "matrix backend {} -> {}",
        backend_name(cfg.server_backend),
        cfg.receipt_path
            .as_ref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "<missing-receipt>".to_string())
    ));
    let result = execute(cfg);
    if let Err(receipt_err) = write_smoke_receipt(cfg, result.as_ref()) {
        return match result {
            Ok(_) => Err(receipt_err),
            Err(err) => Err(format!(
                "{err}; additionally failed to write receipt: {receipt_err}"
            )),
        };
    }
    result.map(|_| ())
}

fn compare_receipts(cfg: &Config) -> Result<(), String> {
    let (left, right) = cfg
        .compare_receipts
        .as_ref()
        .ok_or_else(|| "compare-receipts mode requires two receipt paths".to_string())?;
    let left = read_receipt_summary(left)?;
    let right = read_receipt_summary(right)?;
    validate_receipt_pair(&left, &right, cfg.server_protocol)?;
    let paper = if left.backend == "paper" {
        &left
    } else {
        &right
    };
    let valence = if left.backend == "valence" {
        &left
    } else {
        &right
    };
    println!(
        "[mc-compat] receipt comparison passed: paper={} valence={} protocol={} headless=xvfb/x11/software-gl/no-wayland",
        paper.path.display(),
        valence.path.display(),
        paper.protocol
    );
    Ok(())
}

fn read_receipt_summary(path: &Path) -> Result<ReceiptSummary, String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("read receipt {}: {e}", path.display()))?;
    read_receipt_summary_from_text(path.to_path_buf(), &text)
}

fn read_receipt_summary_from_text(path: PathBuf, text: &str) -> Result<ReceiptSummary, String> {
    Ok(ReceiptSummary {
        path,
        schema: json_string_field(text, "schema")?,
        status: json_string_field(text, "status")?,
        dry_run: json_bool_field(text, "dry_run")?,
        backend: json_object_string_field(text, "server", "backend")?,
        protocol: json_object_u32_field(text, "server", "protocol")?,
        port: json_object_u32_field(text, "server", "port")? as u16,
        classification: json_object_string_field(text, "client", "classification")?,
        matched_success_pattern: json_object_optional_string_field(
            text,
            "client",
            "matched_success_pattern",
        )?,
        xvfb: json_object_bool_field(text, "headless_isolation", "xvfb")?,
        x11_backend: json_object_bool_field(text, "headless_isolation", "x11_backend")?,
        software_gl: json_object_bool_field(text, "headless_isolation", "software_gl")?,
        wayland_socket_inherited: json_object_bool_field(
            text,
            "headless_isolation",
            "wayland_socket_inherited",
        )?,
    })
}

fn validate_receipt_pair(
    left: &ReceiptSummary,
    right: &ReceiptSummary,
    expected_protocol: u32,
) -> Result<(), String> {
    validate_receipt_summary(left)?;
    validate_receipt_summary(right)?;
    let backends = [left.backend.as_str(), right.backend.as_str()];
    if !(backends.contains(&"paper") && backends.contains(&"valence")) {
        return Err(format!(
            "expected one paper receipt and one valence receipt, got {} and {}",
            left.backend, right.backend
        ));
    }
    if left.protocol != right.protocol {
        return Err(format!(
            "receipt protocol mismatch: {} has {}, {} has {}",
            left.path.display(),
            left.protocol,
            right.path.display(),
            right.protocol
        ));
    }
    if left.protocol != expected_protocol {
        return Err(format!(
            "expected protocol {}, got {}",
            expected_protocol, left.protocol
        ));
    }
    for receipt in [left, right] {
        match receipt.backend.as_str() {
            "paper" if receipt.port != 25566 => {
                return Err(format!(
                    "paper receipt port must be 25566, got {}",
                    receipt.port
                ));
            }
            "valence" if receipt.port != 25565 => {
                return Err(format!(
                    "valence receipt port must be 25565, got {}",
                    receipt.port
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_receipt_summary(receipt: &ReceiptSummary) -> Result<(), String> {
    if !matches!(
        receipt.schema.as_str(),
        "mc.compat.smoke.receipt.v1" | "mc.compat.scenario.receipt.v2"
    ) {
        return Err(format!(
            "{} has unexpected schema {}",
            receipt.path.display(),
            receipt.schema
        ));
    }
    if receipt.status != "pass" {
        return Err(format!(
            "{} did not pass; status={}",
            receipt.path.display(),
            receipt.status
        ));
    }
    let classification_supported = matches!(
        receipt.classification.as_str(),
        "timeout-success-evidence" | "client-exited-success"
    ) || (receipt.dry_run && receipt.classification == "dry-run");
    if !classification_supported {
        return Err(format!(
            "{} has unsupported client classification {}",
            receipt.path.display(),
            receipt.classification
        ));
    }
    if receipt.matched_success_pattern.is_none() && !receipt.dry_run {
        return Err(format!(
            "{} is missing matched client success pattern",
            receipt.path.display()
        ));
    }
    if !(receipt.xvfb && receipt.x11_backend && receipt.software_gl)
        || receipt.wayland_socket_inherited
    {
        return Err(format!(
            "{} does not prove niri-safe headless isolation",
            receipt.path.display()
        ));
    }
    Ok(())
}

fn json_object_string_field(text: &str, object: &str, key: &str) -> Result<String, String> {
    json_string_field(json_object_slice(text, object)?, key)
}

fn json_object_optional_string_field(
    text: &str,
    object: &str,
    key: &str,
) -> Result<Option<String>, String> {
    json_optional_string_field(json_object_slice(text, object)?, key)
}

fn json_object_u32_field(text: &str, object: &str, key: &str) -> Result<u32, String> {
    json_u32_field(json_object_slice(text, object)?, key)
}

fn json_object_bool_field(text: &str, object: &str, key: &str) -> Result<bool, String> {
    json_bool_field(json_object_slice(text, object)?, key)
}

fn json_object_slice<'a>(text: &'a str, object: &str) -> Result<&'a str, String> {
    let key = format!("\"{object}\"");
    let start = text
        .find(&key)
        .ok_or_else(|| format!("missing object {object}"))?;
    let after_key = &text[start + key.len()..];
    let brace_offset = after_key
        .find('{')
        .ok_or_else(|| format!("missing object body for {object}"))?;
    let body_start = start + key.len() + brace_offset;
    let mut depth = 0usize;
    for (offset, ch) in text[body_start..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(&text[body_start..=body_start + offset]);
                }
            }
            _ => {}
        }
    }
    Err(format!("unterminated object {object}"))
}

fn json_string_field(text: &str, key: &str) -> Result<String, String> {
    let after_colon = json_field_value(text, key)?;
    parse_json_string(after_colon).map(|(value, _)| value)
}

fn json_optional_string_field(text: &str, key: &str) -> Result<Option<String>, String> {
    let Some(after_colon) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    if after_colon.trim_start().starts_with("null") {
        Ok(None)
    } else {
        parse_json_string(after_colon).map(|(value, _)| Some(value))
    }
}

fn json_optional_bool_field(text: &str, key: &str) -> Result<Option<bool>, String> {
    let Some(after_colon) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    if after_colon.starts_with("true") {
        Ok(Some(true))
    } else if after_colon.starts_with("false") {
        Ok(Some(false))
    } else {
        Err(format!("field {key} must be a boolean"))
    }
}

fn json_optional_u32_field(text: &str, key: &str) -> Result<Option<u32>, String> {
    let Some(value) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    parse_json_u32_value(key, value).map(Some)
}

fn json_u32_field(text: &str, key: &str) -> Result<u32, String> {
    parse_json_u32_value(key, json_field_value(text, key)?)
}

fn parse_json_u32_value(key: &str, value: &str) -> Result<u32, String> {
    let value = value.trim_start();
    let digits: String = value.chars().take_while(|ch| ch.is_ascii_digit()).collect();
    if digits.is_empty() {
        return Err(format!("field {key} is not an unsigned integer"));
    }
    digits
        .parse()
        .map_err(|e| format!("parse field {key}: {e}"))
}

fn json_optional_string_array_field(text: &str, key: &str) -> Result<Option<Vec<String>>, String> {
    let Some(value) = json_field_value_opt(text, key)? else {
        return Ok(None);
    };
    parse_json_string_array(value).map(Some)
}

fn json_bool_field(text: &str, key: &str) -> Result<bool, String> {
    let value = json_field_value(text, key)?.trim_start();
    if value.starts_with("true") {
        Ok(true)
    } else if value.starts_with("false") {
        Ok(false)
    } else {
        Err(format!("field {key} is not a bool"))
    }
}

fn json_field_value<'a>(text: &'a str, key: &str) -> Result<&'a str, String> {
    json_field_value_opt(text, key)?.ok_or_else(|| format!("missing field {key}"))
}

fn json_field_value_opt<'a>(text: &'a str, key: &str) -> Result<Option<&'a str>, String> {
    let needle = format!("\"{key}\"");
    let Some(start) = text.find(&needle) else {
        return Ok(None);
    };
    let after_key = &text[start + needle.len()..];
    let colon = after_key
        .find(':')
        .ok_or_else(|| format!("missing colon for field {key}"))?;
    Ok(Some(&after_key[colon + 1..]))
}

fn parse_json_string(text: &str) -> Result<(String, &str), String> {
    let text = text.trim_start();
    let mut chars = text.char_indices();
    match chars.next() {
        Some((_, '"')) => {}
        _ => return Err("expected JSON string".to_string()),
    }
    let mut out = String::new();
    let mut escape = false;
    for (idx, ch) in chars {
        if escape {
            match ch {
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                '/' => out.push('/'),
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                other => out.push(other),
            }
            escape = false;
        } else if ch == '\\' {
            escape = true;
        } else if ch == '"' {
            return Ok((out, &text[idx + 1..]));
        } else {
            out.push(ch);
        }
    }
    Err("unterminated JSON string".to_string())
}

fn parse_json_string_array(text: &str) -> Result<Vec<String>, String> {
    let mut rest = text.trim_start();
    if !rest.starts_with('[') {
        return Err("expected JSON string array".to_string());
    }
    rest = &rest[1..];
    let mut out = Vec::new();
    loop {
        rest = rest.trim_start();
        if let Some(after) = rest.strip_prefix(']') {
            let _ = after;
            return Ok(out);
        }
        let (value, after_string) = parse_json_string(rest)?;
        out.push(value);
        rest = after_string.trim_start();
        if let Some(after) = rest.strip_prefix(',') {
            rest = after;
        } else if rest.starts_with(']') {
            continue;
        } else {
            return Err("expected comma or closing bracket in JSON string array".to_string());
        }
    }
}

fn mode_name(mode: Mode) -> &'static str {
    match mode {
        Mode::DryRun => "dry-run",
        Mode::Run => "run",
        Mode::RunMatrix => "run-matrix",
        Mode::BuildClient => "build-client",
        Mode::StatusOnly => "status-only",
        Mode::HarnessStatus => "status",
        Mode::Cleanup => "cleanup",
        Mode::Stop => "stop",
        Mode::CompareReceipts => "compare-receipts",
    }
}

fn backend_name(backend: ServerBackend) -> &'static str {
    match backend {
        ServerBackend::Valence => "valence",
        ServerBackend::Paper => "paper",
    }
}

fn json_optional_string(value: Option<&str>) -> String {
    value.map(json_string).unwrap_or_else(|| "null".to_string())
}

fn json_string_array(values: &[&str]) -> String {
    json_string_iter(values.iter().copied())
}

fn json_string_vec(values: &[String]) -> String {
    json_string_iter(values.iter().map(String::as_str))
}

fn json_string_iter<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    let mut out = String::from("[");
    for (idx, value) in values.into_iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str(&json_string(value));
    }
    out.push(']');
    out
}

fn json_string(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_control() => out.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn apply_build_env(cmd: &mut Command, target_dir: &Path) {
    cmd.env("RUSTC_WRAPPER", "")
        .env("CARGO_TARGET_DIR", target_dir)
        .env("CMAKE_POLICY_VERSION_MINIMUM", "3.5");
}

fn apply_headless_env(cmd: &mut Command) {
    cmd.env_remove("WAYLAND_DISPLAY")
        .env_remove("WAYLAND_SOCKET")
        .env_remove("XDG_CURRENT_DESKTOP")
        .env("XDG_SESSION_TYPE", "x11")
        .env("WINIT_UNIX_BACKEND", "x11")
        .env("GDK_BACKEND", "x11")
        .env("SDL_VIDEODRIVER", "x11")
        .env("LIBGL_ALWAYS_SOFTWARE", "1")
        .env("MESA_LOADER_DRIVER_OVERRIDE", "llvmpipe");
}

fn run_cmd(cfg: &Config, cmd: &mut Command) -> Result<(), String> {
    if cfg.mode == Mode::DryRun {
        println!("+ {cmd:?}");
        return Ok(());
    }
    let status = cmd.status().map_err(|e| format!("spawn {cmd:?}: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command {cmd:?} failed with {status}"))
    }
}

fn write_packet(id: u32, payload: &[u8], out: &mut TcpStream) -> Result<(), String> {
    let mut body = Vec::new();
    write_varint(id, &mut body);
    body.extend_from_slice(payload);
    let mut packet = Vec::new();
    write_varint(body.len() as u32, &mut packet);
    packet.extend_from_slice(&body);
    out.write_all(&packet).map_err(|e| e.to_string())
}

fn write_string(s: &str, out: &mut Vec<u8>) {
    write_varint(s.len() as u32, out);
    out.extend_from_slice(s.as_bytes());
}

fn write_varint(mut value: u32, out: &mut Vec<u8>) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            break;
        }
    }
}

fn read_varint(input: &mut TcpStream) -> Result<u32, String> {
    let mut value = 0u32;
    for shift in (0..35).step_by(7) {
        let mut byte = [0u8; 1];
        input.read_exact(&mut byte).map_err(|e| e.to_string())?;
        value |= u32::from(byte[0] & 0x7f) << shift;
        if byte[0] & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err("varint too long".to_string())
}

fn parse_backend(value: &str) -> Result<ServerBackend, String> {
    match value {
        "valence" => Ok(ServerBackend::Valence),
        "paper" => Ok(ServerBackend::Paper),
        other => Err(format!("unknown server backend: {other}")),
    }
}

fn env_string(name: &str) -> Option<String> {
    env::var(name).ok().filter(|s| !s.is_empty())
}

fn env_path(name: &str) -> Option<PathBuf> {
    env_string(name).map(PathBuf::from)
}

fn temp_client_log_for(label: &str) -> PathBuf {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    let safe_label: String = label
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' {
                ch
            } else {
                '-'
            }
        })
        .collect();
    PathBuf::from(format!("/tmp/mc-compat-client.{safe_label}.{millis}.log"))
}

fn log(args: std::fmt::Arguments<'_>) {
    println!("[mc-compat] {args}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn test_config(args: &[&str], env: &[(&str, &str)]) -> Result<Config, String> {
        let env: BTreeMap<String, String> = env
            .iter()
            .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
            .collect();
        Config::from_sources(
            PathBuf::from("/workspace/mc"),
            |name| env.get(name).cloned(),
            args.iter().map(|arg| (*arg).to_string()),
        )
    }

    fn fake_stevenarella_checkout(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "mc-compat-stevenarella-{label}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create fake Stevenarella checkout");
        fs::write(
            dir.join("Cargo.toml"),
            "[package]\nname = \"stevenarella\"\nversion = \"0.0.0\"\nedition = \"2021\"\n",
        )
        .expect("write fake Stevenarella manifest");
        dir
    }

    const ALL_TEST_SCENARIOS: &[Scenario] = &[
        Scenario::Smoke,
        Scenario::CompatBotProbe,
        Scenario::FlagScoreRepeat,
        Scenario::BlueFlagScore,
        Scenario::InventoryInteraction,
        Scenario::SurvivalBreakPlacePickup,
        Scenario::SurvivalChestPersistence,
        Scenario::CombatDamage,
        Scenario::CombatKnockback,
        Scenario::ArmorEquipmentMitigation,
        Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        Scenario::EquipmentUpdateObservation,
        Scenario::EquipmentSlotItemMatrixExpansion,
        Scenario::ProjectileHit,
        Scenario::ProjectileDamageAttribution,
        Scenario::FlagCarrierDeathReturn,
        Scenario::ReconnectFlagState,
        Scenario::ReconnectFlagScore,
        Scenario::MultiClientLoadScore,
        Scenario::NegativeInventoryStaleState,
        Scenario::NegativeInventoryInvalidClick,
        Scenario::NegativeCustomPayload,
        Scenario::NegativeReconnectRace,
        Scenario::NegativeCtfWrongScore,
    ];

    fn passing_client_lines(scenario: Scenario) -> Vec<(&'static str, String)> {
        scenario_required_milestones(scenario)
            .iter()
            .map(|(name, needle)| (*name, (*needle).to_string()))
            .collect()
    }

    fn passing_client_output(scenario: Scenario) -> String {
        output_from_lines(&passing_client_lines(scenario))
    }

    fn passing_server_lines(scenario: Scenario) -> Vec<(&'static str, String)> {
        server_required_milestones(scenario)
            .iter()
            .map(|(name, needle)| (*name, server_fixture_line_for(name, needle)))
            .collect()
    }

    fn output_from_lines(lines: &[(&'static str, String)]) -> String {
        lines
            .iter()
            .map(|(_, line)| line.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn output_without_line(lines: &[(&'static str, String)], omitted: &'static str) -> String {
        lines
            .iter()
            .filter(|(name, _)| *name != omitted)
            .map(|(_, line)| line.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn server_fixture_line_for(name: &str, needle: &str) -> String {
        match name {
            "server_username_seen" => "compatbot".to_string(),
            "server_client_a_seen" => "compatbota".to_string(),
            "server_client_b_seen" => "compatbotb".to_string(),
            "server_flag_or_score" => "flag".to_string(),
            _ => needle.to_string(),
        }
    }

    #[test]
    fn git_revision_evidence_core_reports_clean_dirty_and_unavailable() {
        let clean_evidence =
            build_git_revision_evidence(Some("HEAD"), Ok("abc123".to_string()), Ok(false));
        assert_eq!(clean_evidence.status, GIT_STATUS_CLEAN);
        assert!(!clean_evidence.dirty);
        assert_eq!(clean_evidence.requested_rev.as_deref(), Some("HEAD"));
        assert_eq!(clean_evidence.resolved_rev.as_deref(), Some("abc123"));
        assert!(clean_evidence.diagnostics.is_empty(), "{clean_evidence:?}");

        let dirty_evidence =
            build_git_revision_evidence(Some("HEAD"), Ok("abc123".to_string()), Ok(true));
        assert_eq!(dirty_evidence.status, GIT_STATUS_DIRTY);
        assert!(dirty_evidence.dirty);
        assert_eq!(dirty_evidence.resolved_rev.as_deref(), Some("abc123"));

        let unavailable_evidence = build_git_revision_evidence(
            None,
            Err("missing rev".to_string()),
            Err("missing status".to_string()),
        );
        assert_eq!(unavailable_evidence.status, GIT_STATUS_UNAVAILABLE);
        assert!(unavailable_evidence.dirty);
        assert!(unavailable_evidence.resolved_rev.is_none());
        let expected_diagnostic_count = 2;
        assert_eq!(
            unavailable_evidence.diagnostics.len(),
            expected_diagnostic_count
        );
    }

    #[test]
    fn dry_run_receipt_records_deterministic_child_revision_placeholders() {
        let cfg = test_config(&["--scenario=survival-break-place-pickup"], &[])
            .expect("dry-run config parses");
        let json = smoke_receipt_json(&cfg, Ok(&None));

        assert!(json.contains("\"git_rev\": \"dry-run\""), "{json}");
        assert!(json.contains("\"git_status\": \"dry-run\""), "{json}");
        assert!(json.contains("\"git_dirty\": false"), "{json}");
        assert!(
            json.contains("\"git_rev_requested\": \"8ad9c85\""),
            "{json}"
        );
        assert!(json.contains("\"git_rev_resolved\": \"dry-run\""), "{json}");
    }

    #[test]
    fn defaults_to_valence_protocol_and_port() {
        let cfg = test_config(&[], &[]).expect("default config parses");

        assert_eq!(cfg.root, PathBuf::from("/workspace/mc"));
        assert_eq!(cfg.client_dir, PathBuf::from("/workspace/mc/stevenarella"));
        assert_eq!(cfg.valence_repo, PathBuf::from("/workspace/mc/valence"));
        assert_eq!(cfg.server_backend, ServerBackend::Valence);
        assert_eq!(cfg.server_protocol, DEFAULT_SERVER_PROTOCOL);
        assert_eq!(cfg.server_port, 25565);
        assert_eq!(cfg.valence_rev, DEFAULT_VALENCE_REV);
        assert_eq!(cfg.mode, Mode::DryRun);
    }

    #[test]
    fn cli_overrides_backend_client_dir_valence_repo_and_revision() {
        let cfg = test_config(
            &[
                "--run",
                "--server-backend",
                "paper",
                "--client-dir",
                "/tmp/editable-stevenarella",
                "--receipt=/tmp/mc-smoke.json",
                "--valence-repo",
                "/tmp/editable-valence",
                "--valence-rev=local-debug-rev",
            ],
            &[],
        )
        .expect("cli override config parses");

        assert_eq!(cfg.mode, Mode::Run);
        assert_eq!(cfg.server_backend, ServerBackend::Paper);
        assert_eq!(cfg.server_port, 25566);
        assert_eq!(cfg.client_dir, PathBuf::from("/tmp/editable-stevenarella"));
        assert_eq!(cfg.receipt_path, Some(PathBuf::from("/tmp/mc-smoke.json")));
        assert_eq!(cfg.valence_repo, PathBuf::from("/tmp/editable-valence"));
        assert_eq!(cfg.valence_rev, "local-debug-rev");
    }

    #[test]
    fn run_matrix_config_sets_receipt_dir_and_backend_defaults() {
        let cfg = test_config(
            &[
                "--run-matrix",
                "--receipt-dir",
                "/tmp/matrix-receipts",
                "--dry-run",
                "--client-dir",
                "/tmp/stevenarella",
            ],
            &[],
        )
        .expect("matrix config parses");

        assert_eq!(cfg.mode, Mode::RunMatrix);
        assert!(cfg.matrix_dry_run);
        assert_eq!(cfg.receipt_dir, Some(PathBuf::from("/tmp/matrix-receipts")));

        let paper = matrix_backend_config(&cfg, ServerBackend::Paper, PathBuf::from("paper.json"));
        let valence =
            matrix_backend_config(&cfg, ServerBackend::Valence, PathBuf::from("valence.json"));
        assert_eq!(paper.mode, Mode::DryRun);
        assert_eq!(paper.server_port, 25566);
        assert_eq!(paper.receipt_path, Some(PathBuf::from("paper.json")));
        assert_eq!(valence.mode, Mode::DryRun);
        assert_eq!(valence.server_port, 25565);
        assert_eq!(valence.receipt_path, Some(PathBuf::from("valence.json")));
    }

    #[test]
    fn run_matrix_rejects_single_receipt_path() {
        let err = test_config(&["--run-matrix", "--receipt", "/tmp/one.json"], &[]).unwrap_err();
        assert!(
            err.contains("--run-matrix writes backend receipts"),
            "{err}"
        );
    }

    #[test]
    fn status_and_cleanup_modes_parse_without_server_probe_mode() {
        let status = test_config(&["--status"], &[]).expect("status config parses");
        assert_eq!(status.mode, Mode::HarnessStatus);
        assert!(!status.cleanup_apply);

        let cleanup_dry =
            test_config(&["--cleanup", "--dry-run"], &[]).expect("cleanup dry-run config parses");
        assert_eq!(cleanup_dry.mode, Mode::Cleanup);
        assert!(!cleanup_dry.cleanup_apply);

        let cleanup_apply =
            test_config(&["--cleanup", "--apply"], &[]).expect("cleanup apply config parses");
        assert_eq!(cleanup_apply.mode, Mode::Cleanup);
        assert!(cleanup_apply.cleanup_apply);
    }

    #[test]
    fn cleanup_client_log_match_is_narrow() {
        assert!(is_mc_compat_client_log("mc-compat-client.123.log"));
        assert!(!is_mc_compat_client_log("mc-compat-client.123.txt"));
        assert!(!is_mc_compat_client_log("other-mc-compat-client.123.log"));
    }

    #[test]
    fn cleanup_path_dry_run_preserves_existing_files() {
        let dir =
            std::env::temp_dir().join(format!("mc-compat-cleanup-dry-run-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create cleanup dry-run fixture");
        let file = dir.join("artifact.log");
        fs::write(&file, "keep me").expect("write cleanup fixture");

        cleanup_path("test artifact", &file, false).expect("dry-run cleanup succeeds");
        assert!(file.exists(), "dry-run cleanup must not remove files");

        cleanup_path("test artifact", &file, true).expect("apply cleanup removes file");
        assert!(!file.exists(), "apply cleanup removes files");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn nickel_exported_json_config_sets_defaults_and_allows_env_cli_precedence() {
        let config_json = r#"{
          "client_dir": "/config/stevenarella",
          "valence_repo": "/config/valence",
          "valence_rev": "config-rev",
          "server_backend": "paper",
          "server_protocol": 758,
          "server_port": 25566,
          "client_timeout_secs": 9,
          "client_success_patterns": ["Detected server protocol version", "Dimension type:"],
          "receipt_path": "/config/receipt.json"
        }"#;
        let mut cfg = Config::defaults(PathBuf::from("/workspace/mc"));

        let server_port_was_set = apply_config_json(&mut cfg, config_json).expect("config applies");

        assert!(server_port_was_set);
        assert_eq!(cfg.client_dir, PathBuf::from("/config/stevenarella"));
        assert_eq!(cfg.valence_repo, PathBuf::from("/config/valence"));
        assert_eq!(cfg.valence_rev, "config-rev");
        assert_eq!(cfg.server_backend, ServerBackend::Paper);
        assert_eq!(cfg.server_port, 25566);
        assert_eq!(cfg.client_timeout, Duration::from_secs(9));
        assert_eq!(
            cfg.receipt_path,
            Some(PathBuf::from("/config/receipt.json"))
        );
        assert_eq!(
            cfg.client_success_needles,
            vec![
                "Detected server protocol version".to_string(),
                "Dimension type:".to_string()
            ]
        );

        let cfg = test_config(
            &[
                "--config",
                "/tmp/mc-compat-config.json",
                "--server-backend",
                "valence",
            ],
            &[("MC_COMPAT_CONFIG", "/tmp/mc-compat-config.json")],
        );
        assert!(
            cfg.unwrap_err()
                .contains("read config /tmp/mc-compat-config.json"),
            "missing config path should produce actionable read error"
        );
    }

    #[test]
    fn restricted_steel_config_sets_runtime_defaults_and_allows_env_cli_precedence() {
        let path =
            std::env::temp_dir().join(format!("mc-compat-steel-config-{}.scm", std::process::id()));
        fs::write(
            &path,
            r#"
(define config-version 1)
(define sandbox-profile "mc-compat/pure-v1")
(define server-backend "paper")
(define server-version "1.20.1")
(define server-protocol 763)
(define server-port 25566)
(define valence-rev "main")
(define valence-example "ctf")
(define valence-worktree "/tmp/valence-compat-763")
(define valence-target-dir "/tmp/valence-compat-763-target")
(define valence-log "/tmp/mc-compat-valence.log")
(define valence-pid-file "/tmp/mc-compat-valence.pid")
(define client-username "compatbot")
(define client-timeout-secs 77)
(define client-success-patterns (list "Detected server protocol version" "Dimension type:"))
(define receipt-dir "target/mc-compat-steel")
(define scenario "projectile-damage-attribution")
(define arrow-base-damage 3.0)
(define arrow-velocity-multiplier 1.0)
(define arrow-max-damage 10.0)
(define (arrow-damage ctx)
  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))
"#,
        )
        .expect("write Steel config fixture");

        let cfg = test_config(
            &[
                "--steel-config",
                path.to_str().expect("utf8 path"),
                "--server-backend",
                "valence",
            ],
            &[],
        )
        .expect("Steel config parses");

        assert_eq!(cfg.steel_config_path, Some(path.clone()));
        assert_eq!(cfg.server_backend, ServerBackend::Valence);
        assert_eq!(cfg.server_version, "1.20.1");
        assert_eq!(cfg.server_port, 25566);
        assert_eq!(cfg.server_protocol, 763);
        assert_eq!(cfg.valence_rev, "main");
        assert_eq!(cfg.valence_example, "ctf");
        assert_eq!(
            cfg.valence_worktree,
            PathBuf::from("/tmp/valence-compat-763")
        );
        assert_eq!(
            cfg.valence_target_dir,
            PathBuf::from("/tmp/valence-compat-763-target")
        );
        assert_eq!(cfg.valence_log, PathBuf::from("/tmp/mc-compat-valence.log"));
        assert_eq!(
            cfg.valence_pid_file,
            PathBuf::from("/tmp/mc-compat-valence.pid")
        );
        assert_eq!(cfg.client_username, "compatbot");
        assert_eq!(cfg.client_timeout, Duration::from_secs(77));
        assert_eq!(
            cfg.receipt_dir,
            Some(PathBuf::from("target/mc-compat-steel"))
        );
        assert_eq!(cfg.scenario, Scenario::ProjectileDamageAttribution);
        assert_eq!(
            cfg.client_success_needles,
            vec![
                "Detected server protocol version".to_string(),
                "Dimension type:".to_string()
            ]
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn restricted_steel_config_rejects_forbidden_capabilities() {
        let path = std::env::temp_dir().join(format!(
            "mc-compat-bad-steel-config-{}.scm",
            std::process::id()
        ));
        fs::write(
            &path,
            r#"
(define config-version 1)
(define sandbox-profile "mc-compat/pure-v1")
(define server-backend "valence")
(define server-protocol 763)
(define server-port 25565)
(define client-timeout-secs 20)
(define client-success-patterns (list "Detected server protocol version"))
(define scenario "smoke")
(define arrow-base-damage 3.0)
(define arrow-velocity-multiplier 1.0)
(define arrow-max-damage 10.0)
(define (arrow-damage ctx)
  (damage-linear ctx arrow-base-damage arrow-velocity-multiplier arrow-max-damage))
(open-input-file "/etc/passwd")
"#,
        )
        .expect("write bad Steel config fixture");

        let err =
            test_config(&["--steel-config", path.to_str().expect("utf8 path")], &[]).unwrap_err();
        assert!(err.contains("forbidden Steel capability"), "{err}");
        let _ = fs::remove_file(path);
    }

    #[test]
    fn env_overrides_are_parsed_without_global_environment_mutation() {
        let cfg = test_config(
            &["--server-backend=paper"],
            &[
                ("MC_COMPAT_ROOT", "/repo/mc"),
                ("CLIENT_TIMEOUT", "8"),
                (
                    "CLIENT_SUCCESS_PATTERN",
                    "Detected server protocol version|Dimension type:",
                ),
                ("SERVER_PORT", "24444"),
                ("SMOKE_RECEIPT", "/repo/receipts/smoke.json"),
                ("CLIENT_DIR", "/repo/stevenarella-edit"),
                ("VALENCE_REPO", "/repo/valence-edit"),
                ("VALENCE_REV", "debug-rev"),
                ("PAPER_PLUGIN_JAR", "/repo/fixtures/paper-survival.jar"),
            ],
        )
        .expect("env override config parses");

        assert_eq!(cfg.root, PathBuf::from("/repo/mc"));
        assert_eq!(cfg.client_dir, PathBuf::from("/repo/stevenarella-edit"));
        assert_eq!(cfg.server_backend, ServerBackend::Paper);
        assert_eq!(cfg.server_port, 24444);
        assert_eq!(
            cfg.receipt_path,
            Some(PathBuf::from("/repo/receipts/smoke.json"))
        );
        assert_eq!(cfg.client_timeout, Duration::from_secs(8));
        assert_eq!(cfg.valence_repo, PathBuf::from("/repo/valence-edit"));
        assert_eq!(cfg.valence_rev, "debug-rev");
        assert_eq!(
            cfg.paper_plugin_jar,
            Some(PathBuf::from("/repo/fixtures/paper-survival.jar"))
        );
        assert_eq!(
            cfg.client_success_needles,
            vec![
                "Detected server protocol version".to_string(),
                "Dimension type:".to_string()
            ]
        );
    }

    #[test]
    fn invalid_backend_is_rejected() {
        let err = test_config(&["--server-backend", "spigot"], &[]).unwrap_err();
        assert!(err.contains("unknown server backend: spigot"), "{err}");
    }

    #[test]
    fn scenario_cli_and_env_parse() {
        let cli =
            test_config(&["--scenario", "flag-score-repeat"], &[]).expect("scenario CLI parses");
        assert_eq!(cli.scenario, Scenario::FlagScoreRepeat);

        let env = test_config(&[], &[("MC_COMPAT_SCENARIO", "flag-score-repeat")])
            .expect("scenario env parses");
        assert_eq!(env.scenario, Scenario::FlagScoreRepeat);

        let compat = test_config(&["--scenario", "valence-compat-bot-probe"], &[])
            .expect("compat-bot scenario parses");
        assert_eq!(compat.scenario, Scenario::CompatBotProbe);

        let compat_alias =
            test_config(&["--scenario", "compat-bot-probe"], &[]).expect("compat-bot alias parses");
        assert_eq!(compat_alias.scenario, Scenario::CompatBotProbe);

        let reconnect = test_config(&["--scenario", "reconnect-flag-score"], &[])
            .expect("reconnect scenario parses");
        assert_eq!(reconnect.scenario, Scenario::ReconnectFlagScore);

        let reconnect_state = test_config(&["--scenario", "reconnect-flag-state"], &[])
            .expect("reconnect flag-state scenario parses");
        assert_eq!(reconnect_state.scenario, Scenario::ReconnectFlagState);

        let blue =
            test_config(&["--scenario", "blue-flag-score"], &[]).expect("blue scenario parses");
        assert_eq!(blue.scenario, Scenario::BlueFlagScore);

        let multi = test_config(&["--scenario", "multi-client-load-score"], &[])
            .expect("multi-client scenario parses");
        assert_eq!(multi.scenario, Scenario::MultiClientLoadScore);

        let inventory = test_config(&["--scenario", "inventory-interaction"], &[])
            .expect("inventory scenario parses");
        assert_eq!(inventory.scenario, Scenario::InventoryInteraction);

        let survival = test_config(&["--scenario", "survival-break-place-pickup"], &[])
            .expect("survival scenario parses");
        assert_eq!(survival.scenario, Scenario::SurvivalBreakPlacePickup);

        let chest = test_config(&["--scenario", "survival-chest-persistence"], &[])
            .expect("survival chest scenario parses");
        assert_eq!(chest.scenario, Scenario::SurvivalChestPersistence);

        let knockback = test_config(&["--scenario", "combat-knockback"], &[])
            .expect("combat-knockback scenario parses");
        assert_eq!(knockback.scenario, Scenario::CombatKnockback);

        let armor_matrix = test_config(
            &["--scenario", "armor-loadout-enchantment-status-matrix"],
            &[],
        )
        .expect("armor matrix scenario parses");
        assert_eq!(
            armor_matrix.scenario,
            Scenario::ArmorLoadoutEnchantmentStatusMatrix
        );

        let equipment_matrix =
            test_config(&["--scenario", "equipment-slot-item-matrix-expansion"], &[])
                .expect("equipment matrix scenario parses");
        assert_eq!(
            equipment_matrix.scenario,
            Scenario::EquipmentSlotItemMatrixExpansion
        );

        let projectile_damage = test_config(&["--scenario", "projectile-damage-attribution"], &[])
            .expect("projectile damage scenario parses");
        assert_eq!(
            projectile_damage.scenario,
            Scenario::ProjectileDamageAttribution
        );

        let negative = test_config(&["--scenario", "negative-inventory-stale-state"], &[])
            .expect("negative scenario parses");
        assert_eq!(negative.scenario, Scenario::NegativeInventoryStaleState);
    }

    #[test]
    fn supported_scenario_usage_lists_all_supported_scenarios() {
        for row in scenario_manifest_generated::SCENARIO_MANIFEST_ROWS {
            assert!(
                SUPPORTED_SCENARIO_USAGE.contains(row.name),
                "usage omits supported scenario {}",
                row.name
            );
        }
    }

    #[test]
    fn generated_scenario_manifest_matches_runner_parser() {
        for row in scenario_manifest_generated::SCENARIO_MANIFEST_ROWS {
            let canonical = parse_scenario(row.name).expect("canonical scenario parses");
            assert_eq!(scenario_name(canonical), row.name);
            for alias in row.aliases {
                let parsed = parse_scenario(alias).expect("alias scenario parses");
                assert_eq!(
                    parsed, canonical,
                    "alias {alias} parsed away from {}",
                    row.name
                );
            }
            assert_eq!(
                scenario_required_milestones(canonical).len(),
                row.client_milestones.len()
            );
            for milestone in row.client_milestones {
                assert!(
                    scenario_required_milestones(canonical)
                        .iter()
                        .any(|(name, _)| name == milestone),
                    "generated manifest has client milestone {milestone} absent from runner"
                );
            }
            assert_eq!(
                server_required_milestones(canonical).len(),
                row.server_milestones.len()
            );
            for milestone in row.server_milestones {
                assert!(
                    server_required_milestones(canonical)
                        .iter()
                        .any(|(name, _)| name == milestone),
                    "generated manifest has server milestone {milestone} absent from runner"
                );
            }
            for forbidden in row.forbidden_patterns {
                assert!(
                    scenario_forbidden_patterns(canonical)
                        .iter()
                        .any(|(name, _)| name == forbidden),
                    "generated manifest has forbidden pattern {forbidden} absent from runner"
                );
            }
        }
    }

    #[test]
    fn scenario_oracle_property_all_required_client_milestones_matter() {
        for scenario in ALL_TEST_SCENARIOS {
            let lines = passing_client_lines(*scenario);
            let full = evaluate_scenario(*scenario, &output_from_lines(&lines));
            assert!(
                full.passed,
                "{scenario:?} complete fixture failed: {full:?}"
            );

            for (milestone, _) in &lines {
                let mutated = evaluate_scenario(*scenario, &output_without_line(&lines, milestone));
                assert!(
                    !mutated.passed,
                    "{scenario:?} passed after removing client milestone {milestone}"
                );
                assert!(
                    mutated.missing_milestones.contains(milestone),
                    "{scenario:?} missing diagnostic for removed client milestone {milestone}: {mutated:?}"
                );
            }
        }
    }

    #[test]
    fn scenario_oracle_property_all_required_server_milestones_matter() {
        for scenario in ALL_TEST_SCENARIOS {
            let lines = passing_server_lines(*scenario);
            let full = evaluate_server_scenario(*scenario, &output_from_lines(&lines), "compatbot");
            assert!(
                full.passed,
                "{scenario:?} complete server fixture failed: {full:?}"
            );

            for (milestone, _) in &lines {
                let mutated_output = if *milestone == "server_username_seen" {
                    output_from_lines(&lines).replace("compatbot", "otherbot")
                } else {
                    output_without_line(&lines, milestone)
                };
                let mutated = evaluate_server_scenario(*scenario, &mutated_output, "compatbot");
                assert!(
                    !mutated.passed,
                    "{scenario:?} passed after removing server milestone {milestone}"
                );
                assert!(
                    mutated.missing_milestones.contains(milestone),
                    "{scenario:?} missing diagnostic for removed server milestone {milestone}: {mutated:?}"
                );
            }
        }
    }

    #[test]
    fn scenario_oracle_property_forbidden_markers_fail() {
        for scenario in ALL_TEST_SCENARIOS {
            let base = passing_client_output(*scenario);
            for (forbidden_name, forbidden_needle) in scenario_forbidden_patterns(*scenario) {
                let mutated = format!("{base}\n{forbidden_needle}\n");
                let evidence = evaluate_scenario(*scenario, &mutated);
                assert!(
                    !evidence.passed,
                    "{scenario:?} passed after forbidden marker {forbidden_name}"
                );
                assert!(
                    evidence.forbidden_matches.contains(forbidden_name),
                    "{scenario:?} missing forbidden diagnostic {forbidden_name}: {evidence:?}"
                );
            }
        }
    }

    #[test]
    fn enriched_triage_core_bounds_and_redacts_context() {
        let scenario = evaluate_scenario(
            Scenario::FlagScoreRepeat,
            "Detected server protocol version 763",
        );
        let server = evaluate_server_scenario(Scenario::FlagScoreRepeat, "compatbot", "compatbot");
        let usernames = vec!["compatbot".to_string()];
        let triage = build_enriched_triage(
            &scenario,
            &server,
            "flag-score-repeat",
            &usernames,
            Some("token=secret /tmp/private/server.log"),
            scenario.missing_milestones.first().copied(),
            server.missing_milestones.first().copied(),
            None,
            None,
            "client-probe",
        );

        assert_eq!(triage.boundary_confidence, TRIAGE_CONFIDENCE_MEDIUM);
        assert!(triage
            .correlation_ids
            .contains(&"scenario=flag-score-repeat".to_string()));
        assert!(triage
            .correlation_ids
            .contains(&"user=compatbot".to_string()));
        assert!(triage
            .timeline_excerpt
            .iter()
            .any(|line| line.contains(TRIAGE_REDACTED)));
        assert!(triage
            .timeline_excerpt
            .iter()
            .all(|line| line.chars().count() <= TRIAGE_MAX_EXCERPT_CHARS));
    }

    #[test]
    fn enriched_triage_receipt_preserves_existing_fields_and_adds_context() {
        let cfg = test_config(
            &[
                "--server-backend=valence",
                "--scenario=flag-score-repeat",
                "--receipt=/tmp/receipt.json",
                "--client-dir=/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        let err = "server status probe failed with token=secret /tmp/private".to_string();

        let json = smoke_receipt_json(&cfg, Err(&err));

        assert!(json.contains("\"suggested_boundary\""), "{json}");
        assert!(json.contains("\"enriched\""), "{json}");
        assert!(json.contains("\"boundary_confidence\""), "{json}");
        assert!(json.contains(TRIAGE_REDACTED), "{json}");
    }

    fn typed_event_fixture_lines() -> Vec<&'static str> {
        vec![
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=1 event=protocol_detected",
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=2 event=join_game",
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=3 event=render_tick",
        ]
    }

    fn typed_event_fixture() -> Vec<TypedEvent> {
        typed_event_fixture_lines()
            .into_iter()
            .map(|line| parse_typed_event_line(line).expect("typed event parses"))
            .collect()
    }

    const TEST_SESSION_ID: &str = "s1";
    const TEST_USERNAME: &str = "compatbot";
    const TEST_ATTACKER_USERNAME: &str = "compatbota";
    const TEST_VICTIM_USERNAME: &str = "compatbotb";

    type TypedEventFixtureStep = (&'static str, Option<&'static str>, &'static str);

    fn typed_event_fixture_from_steps(
        scenario: Scenario,
        steps: &[TypedEventFixtureStep],
    ) -> Vec<TypedEvent> {
        let scenario_label = scenario_name(scenario);
        steps
            .iter()
            .enumerate()
            .map(|(index, step)| {
                let (source, username, kind) = *step;
                let sequence_index = index + TYPED_EVENT_SEQUENCE_INDEX_OFFSET;
                let sequence =
                    u32::try_from(sequence_index).expect("fixture sequence fits in u32");
                let username_field = username
                    .map(|name| format!(" username={name}"))
                    .unwrap_or_default();
                let line = format!(
                    "{TYPED_EVENT_PREFIX} schema={TYPED_EVENT_SCHEMA_VERSION} source={source} scenario={scenario_label} session={TEST_SESSION_ID}{username_field} seq={sequence} event={kind}"
                );
                parse_typed_event_line(&line).expect("typed event fixture parses")
            })
            .collect()
    }

    fn typed_event_fixture_required_events(steps: &[TypedEventFixtureStep]) -> Vec<&'static str> {
        steps.iter().map(|(_, _, kind)| *kind).collect()
    }

    fn assert_typed_event_fixture_passes(
        scenario: Scenario,
        username: Option<&str>,
        steps: &[TypedEventFixtureStep],
        ordered_edges: &[(&str, &str)],
    ) {
        let events = typed_event_fixture_from_steps(scenario, steps);
        let required_events = typed_event_fixture_required_events(steps);
        let result = evaluate_typed_event_graph(
            &events,
            scenario_name(scenario),
            TEST_SESSION_ID,
            username,
            &required_events,
            &["panic"],
            ordered_edges,
        );

        assert_eq!(events.len(), steps.len());
        assert!(events
            .iter()
            .all(|event| event.schema_version == TYPED_EVENT_SCHEMA_VERSION));
        assert!(events
            .iter()
            .all(|event| event.scenario == scenario_name(scenario)));
        assert!(events.iter().all(|event| event.session == TEST_SESSION_ID));
        assert!(events
            .iter()
            .all(|event| event.source == "client" || event.source == "server"));
        assert!(events
            .windows(2)
            .all(|pair| pair[0].sequence < pair[1].sequence));
        assert!(result.passed, "{result:?}");
        assert_eq!(result.observed_events.len(), required_events.len());
        assert!(result.missing_events.is_empty(), "{result:?}");
        assert!(result.forbidden_events.is_empty(), "{result:?}");
        assert!(result.order_violations.is_empty(), "{result:?}");
    }

    #[test]
    fn typed_event_parser_accepts_versioned_event_lines() {
        let event = parse_typed_event_line(typed_event_fixture_lines()[0]).expect("event parses");

        assert_eq!(event.schema_version, TYPED_EVENT_SCHEMA_VERSION);
        assert_eq!(event.source, "client");
        assert_eq!(event.scenario, "smoke");
        assert_eq!(event.session, "s1");
        assert_eq!(event.username.as_deref(), Some("compatbot"));
        assert_eq!(event.sequence, 1);
        assert_eq!(event.kind, "protocol_detected");

        let wrong_schema = parse_typed_event_line(
            "MC-COMPAT-EVENT schema=2 source=client scenario=smoke session=s1 username=compatbot seq=1 event=protocol_detected",
        )
        .unwrap_err();
        assert!(
            wrong_schema.contains("unsupported typed event schema"),
            "{wrong_schema}"
        );
    }

    #[test]
    fn typed_event_graph_checks_required_forbidden_and_ordered_events() {
        let events = typed_event_fixture();
        let pass = evaluate_typed_event_graph(
            &events,
            "smoke",
            "s1",
            Some("compatbot"),
            &["protocol_detected", "join_game", "render_tick"],
            &["panic"],
            &[("protocol_detected", "render_tick")],
        );
        assert!(pass.passed, "{pass:?}");

        let missing_required = evaluate_typed_event_graph(
            &events,
            "smoke",
            "s1",
            Some("compatbot"),
            &["protocol_detected", "missing_event"],
            &[],
            &[],
        );
        assert!(!missing_required.passed, "{missing_required:?}");
        assert!(missing_required
            .missing_events
            .contains(&"missing_event".to_string()));

        let wrong_username = evaluate_typed_event_graph(
            &events,
            "smoke",
            "s1",
            Some("otherbot"),
            &["protocol_detected"],
            &[],
            &[],
        );
        assert!(!wrong_username.passed, "{wrong_username:?}");
        assert!(wrong_username
            .missing_events
            .contains(&"protocol_detected".to_string()));

        let wrong_session = evaluate_typed_event_graph(
            &events,
            "smoke",
            "s2",
            Some("compatbot"),
            &["protocol_detected"],
            &[],
            &[],
        );
        assert!(!wrong_session.passed, "{wrong_session:?}");
        assert!(wrong_session
            .missing_events
            .contains(&"protocol_detected".to_string()));

        let mut forbidden_events = events.clone();
        forbidden_events.push(parse_typed_event_line(
            "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=4 event=panic",
        )
        .expect("forbidden event parses"));
        let forbidden = evaluate_typed_event_graph(
            &forbidden_events,
            "smoke",
            "s1",
            Some("compatbot"),
            &["protocol_detected"],
            &["panic"],
            &[],
        );
        assert!(!forbidden.passed, "{forbidden:?}");
        assert!(forbidden.forbidden_events.contains(&"panic".to_string()));

        let out_of_order = vec![
            parse_typed_event_line(
                "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=2 event=protocol_detected",
            )
            .expect("late event parses"),
            parse_typed_event_line(
                "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=1 event=render_tick",
            )
            .expect("early event parses"),
        ];
        let ordered = evaluate_typed_event_graph(
            &out_of_order,
            "smoke",
            "s1",
            Some("compatbot"),
            &["protocol_detected", "render_tick"],
            &[],
            &[("protocol_detected", "render_tick")],
        );
        assert!(!ordered.passed, "{ordered:?}");
        assert!(ordered
            .order_violations
            .contains(&"protocol_detected_before_render_tick".to_string()));

        let stale_sequence = vec![
            parse_typed_event_line(
                "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=7 event=protocol_detected",
            )
            .expect("first duplicate sequence event parses"),
            parse_typed_event_line(
                "MC-COMPAT-EVENT schema=1 source=client scenario=smoke session=s1 username=compatbot seq=7 event=render_tick",
            )
            .expect("stale sequence event parses"),
        ];
        let stale = evaluate_typed_event_graph(
            &stale_sequence,
            "smoke",
            "s1",
            Some("compatbot"),
            &["protocol_detected", "render_tick"],
            &[],
            &[("protocol_detected", "render_tick")],
        );
        assert!(!stale.passed, "{stale:?}");
        assert!(stale
            .order_violations
            .contains(&"protocol_detected_before_render_tick".to_string()));
    }

    #[test]
    fn typed_event_graph_accepts_representative_scenario_fixtures() {
        assert_typed_event_fixture_passes(
            Scenario::Smoke,
            Some(TEST_USERNAME),
            &[
                ("client", Some(TEST_USERNAME), "protocol_detected"),
                ("client", Some(TEST_USERNAME), "join_game"),
                ("client", Some(TEST_USERNAME), "render_tick"),
            ],
            &[("protocol_detected", "render_tick")],
        );
        assert_typed_event_fixture_passes(
            Scenario::InventoryInteraction,
            Some(TEST_USERNAME),
            &[
                ("client", Some(TEST_USERNAME), "protocol_detected"),
                ("client", Some(TEST_USERNAME), "join_game"),
                ("client", Some(TEST_USERNAME), "render_tick"),
                ("client", Some(TEST_USERNAME), "team_red"),
                ("client", Some(TEST_USERNAME), "inventory_slot_update"),
                ("client", Some(TEST_USERNAME), "inventory_drop_sent"),
                ("client", Some(TEST_USERNAME), "inventory_pickup_seen"),
                ("client", Some(TEST_USERNAME), "inventory_click_sent"),
                ("client", Some(TEST_USERNAME), "inventory_block_place_sent"),
                (
                    "server",
                    Some(TEST_USERNAME),
                    "server_inventory_hotbar_select",
                ),
                ("server", Some(TEST_USERNAME), "server_inventory_drop"),
                ("server", Some(TEST_USERNAME), "server_inventory_pickup"),
                ("server", Some(TEST_USERNAME), "server_inventory_click"),
                (
                    "server",
                    Some(TEST_USERNAME),
                    "server_inventory_container_click",
                ),
                ("server", Some(TEST_USERNAME), "server_block_place"),
            ],
            &[
                ("protocol_detected", "inventory_drop_sent"),
                ("inventory_drop_sent", "inventory_pickup_seen"),
                ("server_inventory_drop", "server_inventory_pickup"),
                ("server_inventory_container_click", "server_block_place"),
            ],
        );
        assert_typed_event_fixture_passes(
            Scenario::SurvivalBreakPlacePickup,
            Some(TEST_USERNAME),
            &[
                ("client", Some(TEST_USERNAME), "protocol_detected"),
                ("client", Some(TEST_USERNAME), "join_game"),
                ("client", Some(TEST_USERNAME), "render_tick"),
                ("client", Some(TEST_USERNAME), "survival_break_sent"),
                ("client", Some(TEST_USERNAME), "survival_break_update"),
                ("client", Some(TEST_USERNAME), "survival_pickup_seen"),
                ("client", Some(TEST_USERNAME), "survival_place_sent"),
                ("client", Some(TEST_USERNAME), "survival_place_update"),
                ("server", Some(TEST_USERNAME), "server_survival_join"),
                ("server", Some(TEST_USERNAME), "server_survival_break"),
                ("server", Some(TEST_USERNAME), "server_survival_pickup"),
                ("server", Some(TEST_USERNAME), "server_survival_place"),
            ],
            &[
                ("survival_break_sent", "survival_pickup_seen"),
                ("survival_pickup_seen", "survival_place_sent"),
                ("server_survival_break", "server_survival_place"),
            ],
        );
        assert_typed_event_fixture_passes(
            Scenario::ReconnectFlagState,
            Some(TEST_USERNAME),
            &[
                ("client", Some(TEST_USERNAME), "protocol_detected"),
                ("client", Some(TEST_USERNAME), "join_game"),
                ("client", Some(TEST_USERNAME), "render_tick"),
                ("client", Some(TEST_USERNAME), "team_red"),
                ("client", Some(TEST_USERNAME), "flag_pickup"),
                ("client", Some(TEST_USERNAME), "reconnect_session"),
                ("server", Some(TEST_USERNAME), "server_flag_pickup"),
                (
                    "server",
                    Some(TEST_USERNAME),
                    "server_flag_disconnect_return",
                ),
                (
                    "server",
                    Some(TEST_USERNAME),
                    "server_reconnect_state_coherent",
                ),
            ],
            &[
                ("flag_pickup", "reconnect_session"),
                (
                    "server_flag_disconnect_return",
                    "server_reconnect_state_coherent",
                ),
            ],
        );
        assert_typed_event_fixture_passes(
            Scenario::CombatDamage,
            None,
            &[
                ("client", Some(TEST_ATTACKER_USERNAME), "protocol_detected"),
                ("client", Some(TEST_ATTACKER_USERNAME), "team_red"),
                ("client", Some(TEST_VICTIM_USERNAME), "team_blue"),
                (
                    "client",
                    Some(TEST_ATTACKER_USERNAME),
                    "remote_player_spawn",
                ),
                ("client", Some(TEST_ATTACKER_USERNAME), "combat_attack_sent"),
                ("client", Some(TEST_VICTIM_USERNAME), "combat_health_update"),
                (
                    "server",
                    Some(TEST_ATTACKER_USERNAME),
                    "server_client_a_seen",
                ),
                ("server", Some(TEST_VICTIM_USERNAME), "server_client_b_seen"),
                ("server", None, "server_combat_damage"),
            ],
            &[
                ("remote_player_spawn", "combat_attack_sent"),
                ("combat_attack_sent", "combat_health_update"),
                ("server_client_a_seen", "server_combat_damage"),
            ],
        );
        assert_typed_event_fixture_passes(
            Scenario::ProjectileDamageAttribution,
            None,
            &[
                (
                    "client",
                    Some(TEST_ATTACKER_USERNAME),
                    "attacker_client_projectile_use_sent",
                ),
                (
                    "client",
                    Some(TEST_ATTACKER_USERNAME),
                    "attacker_client_projectile_swing_sent",
                ),
                ("server", None, "server_projectile_use_attacker_victim"),
                (
                    "server",
                    None,
                    "server_projectile_hit_attacker_victim_health_delta",
                ),
                (
                    "client",
                    Some(TEST_VICTIM_USERNAME),
                    "victim_client_damage_update",
                ),
            ],
            &[
                (
                    "attacker_client_projectile_use_sent",
                    "server_projectile_use_attacker_victim",
                ),
                (
                    "server_projectile_use_attacker_victim",
                    "server_projectile_hit_attacker_victim_health_delta",
                ),
                (
                    "server_projectile_hit_attacker_victim_health_delta",
                    "victim_client_damage_update",
                ),
            ],
        );
    }

    #[test]
    fn typed_event_receipt_artifact_records_reviewable_timeline_hash() {
        let events = typed_event_fixture();
        let timeline = normalize_typed_event_timeline(&events);
        let timeline_blake3 = typed_event_timeline_blake3(&timeline);
        let artifact = TypedEventOracleArtifact {
            event_log_path: PathBuf::from("/tmp/mc-compat.typed-events.log"),
            timeline_blake3: timeline_blake3.clone(),
            event_count: events.len(),
            contributes_to_pass_fail: false,
        };

        let json = typed_event_oracle_receipt_json(Some(&artifact));

        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(json.contains("derived-from-milestones"), "{json}");
        assert!(json.contains("/tmp/mc-compat.typed-events.log"), "{json}");
        let event_count_json = format!("\"event_count\": {}", events.len());
        assert!(json.contains(&timeline_blake3), "{json}");
        assert!(json.contains(&event_count_json), "{json}");
        assert!(
            json.contains("\"contributes_to_pass_fail\": false"),
            "{json}"
        );
        assert!(json.contains("\"raw_payloads_recorded\": false"), "{json}");
    }

    #[test]
    fn typed_events_from_receipt_evidence_include_client_and_server_sources() {
        let cfg = test_config(
            &[
                "--scenario",
                "inventory-interaction",
                "--receipt",
                "/tmp/inventory.receipt.json",
            ],
            &[],
        )
        .expect("inventory config parses");
        let client = ClientRunEvidence {
            log_path: None,
            log_paths: Vec::new(),
            usernames: vec![TEST_USERNAME.to_string()],
            exit_code: Some(0),
            classification: "client-exited-success",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(ScenarioEvidence {
                observed_milestones: vec!["protocol_detected", "inventory_drop_sent"],
                missing_milestones: Vec::new(),
                forbidden_matches: Vec::new(),
                passed: true,
            }),
            server_scenario: Some(ServerScenarioEvidence {
                observed_milestones: vec!["server_inventory_drop"],
                missing_milestones: Vec::new(),
                forbidden_matches: Vec::new(),
                passed: true,
            }),
            projectile_damage_causality: None,
        };

        let events =
            typed_events_from_receipt_evidence(&cfg, &client).expect("typed event evidence builds");
        let timeline = normalize_typed_event_timeline(&events);

        let expected_event_count = client
            .scenario
            .as_ref()
            .map(|scenario| scenario.observed_milestones.len())
            .unwrap_or_default()
            + client
                .server_scenario
                .as_ref()
                .map(|server| server.observed_milestones.len())
                .unwrap_or_default();
        assert_eq!(events.len(), expected_event_count);
        assert!(events.iter().any(|event| event.source == "client"));
        assert!(events.iter().any(|event| event.source == "server"));
        assert!(events
            .iter()
            .all(|event| event.username.as_deref() == Some(TEST_USERNAME)));
        assert!(timeline.contains("session=inventory.receipt"), "{timeline}");
        assert!(timeline.contains("event=inventory_drop_sent"), "{timeline}");
        assert!(
            timeline.contains("event=server_inventory_drop"),
            "{timeline}"
        );
    }

    #[test]
    fn typed_event_oracle_validates_migrated_inventory_graph() {
        let cfg = test_config(
            &[
                "--scenario",
                "inventory-interaction",
                "--receipt",
                "/tmp/inventory.receipt.json",
            ],
            &[],
        )
        .expect("inventory config parses");
        let client_observed = scenario_required_milestones(Scenario::InventoryInteraction)
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>();
        let server_observed = server_required_milestones(Scenario::InventoryInteraction)
            .iter()
            .map(|(name, _)| *name)
            .collect::<Vec<_>>();
        let passing = ClientRunEvidence {
            log_path: None,
            log_paths: Vec::new(),
            usernames: vec![TEST_USERNAME.to_string()],
            exit_code: Some(0),
            classification: "client-exited-success",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(ScenarioEvidence {
                observed_milestones: client_observed,
                missing_milestones: Vec::new(),
                forbidden_matches: Vec::new(),
                passed: true,
            }),
            server_scenario: Some(ServerScenarioEvidence {
                observed_milestones: server_observed.clone(),
                missing_milestones: Vec::new(),
                forbidden_matches: Vec::new(),
                passed: true,
            }),
            projectile_damage_causality: None,
        };
        validate_typed_event_oracle_for_migrated_scenario(&cfg, &passing)
            .expect("complete typed inventory graph passes");

        let mut missing_server = passing.clone();
        missing_server
            .server_scenario
            .as_mut()
            .expect("server evidence")
            .observed_milestones
            .retain(|name| *name != "server_block_place");
        let err = validate_typed_event_oracle_for_migrated_scenario(&cfg, &missing_server)
            .expect_err("missing typed server event fails");
        assert!(err.contains("server_block_place"), "{err}");
    }

    #[test]
    fn typed_event_oracle_receipt_records_migration_fallback() {
        let cfg = test_config(
            &[
                "--server-backend=paper",
                "--receipt",
                "/tmp/receipt.json",
                "--client-dir",
                "/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        let json = smoke_receipt_json(&cfg, Err(&"preflight".to_string()));

        assert!(json.contains("\"typed_event_oracle\""), "{json}");
        assert!(
            json.contains("\"migration_status\": \"substring-fallback\""),
            "{json}"
        );
        assert!(json.contains("\"raw_payloads_recorded\": false"), "{json}");
    }

    #[test]
    fn status_packet_proxy_and_gameplay_receipt_blocks_are_recorded() {
        let mut cfg = test_config(
            &[
                "--server-backend=valence",
                "--scenario=reconnect-flag-score",
                "--expect-status-description=compat fixture",
                "--expect-status-version=compat-version",
                "--expect-status-sample=compatbot,observer",
                "--packet-capture-summary",
                "--proxy-route=velocity-local",
                "--proxy-forwarding-mode=modern",
                "--client-dir=/tmp/stevenarella",
            ],
            &[],
        )
        .expect("extended receipt config parses");
        cfg.server_port = 25565;
        let scenario = evaluate_scenario(
            Scenario::ReconnectFlagScore,
            "Detected server protocol version 763
join_game
render_tick_with_player
You are on team RED!
You have the flag!
You captured the flag!
RED: 1
mc_compat_reconnect_session=2
",
        );
        assert!(scenario.passed, "{scenario:?}");
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(scenario),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::ReconnectFlagScore,
                "compatbot joined
red flag captured
",
                "compatbot",
            )),
            projectile_damage_causality: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));

        assert!(
            json.contains("\"name\": \"reconnect-flag-score\""),
            "{json}"
        );
        assert!(json.contains("\"status_response_resource\""), "{json}");
        assert!(json.contains("\"configured\": true"), "{json}");
        assert!(json.contains("compat fixture"), "{json}");
        assert!(json.contains("compat-version"), "{json}");
        assert!(json.contains("compatbot"), "{json}");
        assert!(json.contains("\"packet_capture_oracle\""), "{json}");
        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(json.contains("\"raw_payloads_recorded\": false"), "{json}");
        assert!(json.contains("\"proxy_compat_seam\""), "{json}");
        assert!(json.contains("\"route\": \"velocity-local\""), "{json}");
        assert!(json.contains("\"forwarding_mode\": \"modern\""), "{json}");
        assert!(json.contains("\"mtls_ported\": false"), "{json}");
        assert!(json.contains("\"gameplay_oracles\""), "{json}");
        assert!(json.contains("reconnect_session"), "{json}");
        assert!(json.contains("full_ctf_correctness"), "{json}");
    }

    #[test]
    fn compat_bot_probe_scenario_is_bounded_and_receipted() {
        let pass = evaluate_scenario(
            Scenario::CompatBotProbe,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\n",
        );
        assert!(pass.passed, "{pass:?}");
        assert_eq!(pass.missing_milestones, Vec::<&str>::new());

        let fail = evaluate_scenario(
            Scenario::CompatBotProbe,
            "Detected server protocol version 763\n",
        );
        assert!(!fail.passed, "{fail:?}");
        assert!(fail.missing_milestones.contains(&"join_game"));
        assert!(fail.missing_milestones.contains(&"render_tick"));

        let mut cfg = test_config(
            &[
                "--server-backend=valence",
                "--scenario=valence-compat-bot-probe",
                "--receipt=/tmp/receipt.json",
                "--client-dir=/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        cfg.server_port = 25565;
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(pass),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::CompatBotProbe,
                "compatbot joined\n",
                "compatbot",
            )),
            projectile_damage_causality: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));

        assert!(
            json.contains("\"name\": \"valence-compat-bot-probe\""),
            "{json}"
        );
        assert!(json.contains("\"compat_bot_probe\""), "{json}");
        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(json.contains("\"safe_bounded_probe\": true"), "{json}");
        assert!(
            json.contains("\"external_server_load_authorized\": false"),
            "{json}"
        );
        assert!(json.contains("\"public_stress_tool\": false"), "{json}");
        assert!(json.contains("\"planned_clients\": 1"), "{json}");
        assert!(json.contains("\"max_clients\": 1"), "{json}");
        assert!(
            json.contains("\"target_address\": \"127.0.0.1:25565\""),
            "{json}"
        );
        assert!(json.contains("\"load_network_safety\""), "{json}");
        assert!(
            json.contains("\"target_scope\": \"owned-local-loopback\""),
            "{json}"
        );
        assert!(
            json.contains("\"claims_public_server_safety\": false"),
            "{json}"
        );
        assert!(json.contains("\"claims_unbounded_soak\": false"), "{json}");
    }

    #[test]
    fn latency_jitter_receipt_renders_bounded_wan_telemetry_fields() {
        const TEST_TIMEOUT_SECS: u64 = 180;
        const TEST_CLIENT_COUNT: usize = 1;
        let receipt = LatencyJitterTelemetryReceipt {
            selected: true,
            mechanism: LATENCY_JITTER_DEFAULT_MECHANISM.to_string(),
            target_rail: "inventory-interaction".to_string(),
            delay_ms: "80".to_string(),
            jitter_ms: "30".to_string(),
            loss_percent: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
            timeout_secs: TEST_TIMEOUT_SECS,
            duration_secs: TEST_TIMEOUT_SECS,
            client_count: TEST_CLIENT_COUNT,
            reconnect_count: NO_RECONNECT_SESSIONS,
            target_ownership: WAN_TARGET_OWNERSHIP_OWNED_LOCAL.to_string(),
            authorization: WAN_AUTHORIZATION_OWNED_LOCAL.to_string(),
            hygiene_status: LATENCY_JITTER_ENABLED_HYGIENE_STATUS,
        };

        let json = render_latency_jitter_receipt_json(&receipt);

        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(
            json.contains("\"target_ownership\": \"owned-local-loopback\""),
            "{json}"
        );
        assert!(
            json.contains("\"authorization\": \"owned-local-fixture-approved\""),
            "{json}"
        );
        assert!(json.contains("\"duration_secs\": 180"), "{json}");
        assert!(json.contains("\"client_count\": 1"), "{json}");
        assert!(json.contains("\"reconnect_count\": 0"), "{json}");
        assert!(json.contains("\"telemetry_samples\""), "{json}");
        assert!(json.contains("\"scenario_observed_milestones\""), "{json}");
        assert!(
            json.contains(
                "\"pass_fail_criteria\": \"inventory_interaction_client_server_milestones\""
            ),
            "{json}"
        );
        assert!(json.contains("\"claims_wan_safety\": false"), "{json}");
        assert!(
            json.contains("\"claims_packet_loss_tolerance\": false"),
            "{json}"
        );
        assert!(
            json.contains("\"claims_public_server_safety\": false"),
            "{json}"
        );
        assert!(
            json.contains("\"claims_production_readiness\": false"),
            "{json}"
        );
    }

    #[test]
    fn latency_jitter_receipt_disabled_path_stays_non_claim() {
        const TEST_TIMEOUT_SECS: u64 = 180;
        const TEST_CLIENT_COUNT: usize = 1;
        let receipt = LatencyJitterTelemetryReceipt {
            selected: false,
            mechanism: LATENCY_JITTER_DEFAULT_MECHANISM.to_string(),
            target_rail: "smoke".to_string(),
            delay_ms: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
            jitter_ms: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
            loss_percent: LATENCY_JITTER_DEFAULT_METRIC.to_string(),
            timeout_secs: TEST_TIMEOUT_SECS,
            duration_secs: TEST_TIMEOUT_SECS,
            client_count: TEST_CLIENT_COUNT,
            reconnect_count: NO_RECONNECT_SESSIONS,
            target_ownership: WAN_TARGET_OWNERSHIP_OWNED_LOCAL.to_string(),
            authorization: WAN_AUTHORIZATION_OWNED_LOCAL.to_string(),
            hygiene_status: LATENCY_JITTER_DISABLED_HYGIENE_STATUS,
        };

        let json = render_latency_jitter_receipt_json(&receipt);

        assert!(json.contains("\"selected\": false"), "{json}");
        assert!(
            json.contains("\"hygiene_status\": \"not-selected\""),
            "{json}"
        );
        assert!(
            json.contains("\"fail_closed_when_unavailable\": true"),
            "{json}"
        );
        assert!(json.contains("\"claims_wan_safety\": false"), "{json}");
        assert!(
            json.contains("\"claims_internet_path_safety\": false"),
            "{json}"
        );
    }

    #[test]
    fn latency_jitter_reconnect_count_is_explicit() {
        assert_eq!(
            latency_jitter_reconnect_count(Scenario::InventoryInteraction),
            NO_RECONNECT_SESSIONS
        );
        assert_eq!(
            latency_jitter_reconnect_count(Scenario::ReconnectFlagState),
            SINGLE_RECONNECT_SESSION
        );
        assert_eq!(
            latency_jitter_reconnect_count(Scenario::NegativeReconnectRace),
            SINGLE_RECONNECT_SESSION
        );
    }

    fn baseline_negative_live_rail_inputs() -> NegativeLiveRailInputs {
        NegativeLiveRailInputs {
            selected: true,
            rail: Some("negative-custom-payload"),
            invalid_action: Some("malformed_custom_payload"),
            expected_outcome: Some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME),
            observed_outcome: Some(NEGATIVE_LIVE_RAIL_OBSERVED_OUTCOME_CONTAINMENT),
            observed_outcome_source: Some(
                "client_milestone:negative_custom_payload_contained".to_string(),
            ),
            postcondition_milestone: Some("negative_custom_payload_contained"),
            telemetry_required: true,
            telemetry_present: true,
            target_scope: SAFETY_OWNED_LOCAL_SCOPE,
            explicit_authorization: false,
            public_target: false,
            planned_clients: 1,
            max_clients: NEGATIVE_LIVE_RAIL_MAX_CLIENTS,
            timeout_secs: 20,
        }
    }

    #[test]
    fn negative_live_rail_checker_rejects_unbounded_public_unauthenticated_inputs() {
        let mut inputs = baseline_negative_live_rail_inputs();
        inputs.public_target = true;
        inputs.planned_clients = NEGATIVE_LIVE_RAIL_MAX_CLIENTS + 1;
        let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
        assert!(!evidence.preflight_passed, "{evidence:?}");
        assert!(evidence
            .bound_violations
            .contains(&"public_target_without_authorization"));
        assert!(evidence
            .bound_violations
            .contains(&"planned_clients_exceed_negative_max"));
    }

    #[test]
    fn negative_live_rail_checker_rejects_missing_telemetry() {
        let mut inputs = baseline_negative_live_rail_inputs();
        inputs.telemetry_present = false;
        inputs.observed_outcome = None;
        inputs.observed_outcome_source = None;
        let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
        assert!(!evidence.preflight_passed, "{evidence:?}");
        assert!(evidence.missing_fields.contains(&"telemetry"));
    }

    #[test]
    fn negative_live_rail_checker_rejects_missing_expected_outcome() {
        let mut inputs = baseline_negative_live_rail_inputs();
        inputs.expected_outcome = None;
        let evidence = evaluate_negative_live_rail_safety_from_inputs(inputs);
        assert!(!evidence.preflight_passed, "{evidence:?}");
        assert!(evidence.missing_fields.contains(&"expected_outcome"));
    }

    #[test]
    fn negative_live_rail_receipt_records_observed_containment_outcome() {
        let cfg = test_config(&["--run", "--scenario", "negative-custom-payload"], &[])
            .expect("negative live rail config parses");
        let scenario = evaluate_scenario(
            Scenario::NegativeCustomPayload,
            "Detected server protocol version 763
join_game
render_tick_with_player
negative_custom_payload_sent
negative_custom_payload_contained
",
        );
        assert!(scenario.passed, "{scenario:?}");
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(scenario),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::NegativeCustomPayload,
                "compatbot joined\n",
                "compatbot",
            )),
            projectile_damage_causality: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));
        assert!(
            json.contains("\"observed_outcome\": \"containment_observed\""),
            "{json}"
        );
        assert!(
            json.contains("client_milestone:negative_custom_payload_contained"),
            "{json}"
        );
        assert!(json.contains("\"telemetry_present\": true"), "{json}");
        assert!(json.contains("\"preflight_passed\": true"), "{json}");
    }

    #[test]
    fn negative_live_rail_envelope_records_expected_outcome_and_non_claims() {
        let cfg = test_config(
            &["--dry-run", "--scenario", "negative-inventory-stale-state"],
            &[],
        )
        .expect("negative rail config parses");
        let evidence = evaluate_negative_live_rail_safety(&cfg);
        assert!(evidence.selected, "{evidence:?}");
        assert_eq!(evidence.rail, Some("negative-inventory-stale-state"));
        assert_eq!(evidence.invalid_action, Some("stale_inventory_state_id"));
        assert_eq!(
            evidence.expected_outcome,
            Some(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME)
        );
        assert!(evidence.owned_local_target, "{evidence:?}");
        assert!(evidence.preflight_passed, "{evidence:?}");

        let json = smoke_receipt_json(&cfg, Ok(&None));
        assert!(json.contains("\"negative_live_rail\""), "{json}");
        assert!(json.contains("\"selected\": true"), "{json}");
        assert!(json.contains("stale_inventory_state_id"), "{json}");
        assert!(json.contains(NEGATIVE_LIVE_RAIL_EXPECTED_OUTCOME), "{json}");
        assert!(json.contains("broad_invalid_input_coverage"), "{json}");
        assert!(json.contains("\"raw_payloads_recorded\": false"), "{json}");
    }

    #[test]
    fn negative_live_rail_preflight_rejects_public_unowned_targets() {
        let cfg = test_config(
            &["--dry-run", "--scenario", "negative-custom-payload"],
            &[("MC_COMPAT_PUBLIC_TARGET", "1")],
        )
        .expect("negative rail config parses");
        let err = validate_negative_live_rail_preflight(&cfg)
            .expect_err("public negative rail without authorization fails");
        assert!(err.contains("public_target_without_authorization"), "{err}");
    }

    #[test]
    fn load_network_safety_envelope_fails_closed_for_unsafe_inputs() {
        let safe = evaluate_load_network_safety(LoadNetworkSafetyInputs {
            target_scope: SAFETY_OWNED_LOCAL_SCOPE,
            owned_local_target: true,
            explicit_authorization: false,
            public_target: false,
            planned_clients: SAFETY_MAX_LOCAL_CLIENTS,
            max_clients: SAFETY_MAX_LOCAL_CLIENTS,
            duration_secs: SAFETY_MAX_DURATION_SECS,
            max_duration_secs: SAFETY_MAX_DURATION_SECS,
            reconnect_sessions: SAFETY_SINGLE_SESSION_COUNT,
            latency_ms: SAFETY_ZERO_VALUE.to_string(),
            jitter_ms: SAFETY_ZERO_VALUE.to_string(),
            loss_percent: SAFETY_ZERO_VALUE.to_string(),
            telemetry_present: true,
            live_receipt: true,
        });
        assert!(safe.preflight_passed, "{safe:?}");
        assert!(safe.promotion_ready, "{safe:?}");

        let unsafe_public = evaluate_load_network_safety(LoadNetworkSafetyInputs {
            target_scope: "public",
            owned_local_target: false,
            explicit_authorization: false,
            public_target: true,
            planned_clients: SAFETY_MAX_LOCAL_CLIENTS + 1,
            max_clients: SAFETY_MAX_LOCAL_CLIENTS,
            duration_secs: SAFETY_MAX_DURATION_SECS + 1,
            max_duration_secs: SAFETY_MAX_DURATION_SECS,
            reconnect_sessions: SAFETY_SINGLE_SESSION_COUNT,
            latency_ms: String::new(),
            jitter_ms: SAFETY_ZERO_VALUE.to_string(),
            loss_percent: SAFETY_ZERO_VALUE.to_string(),
            telemetry_present: false,
            live_receipt: false,
        });
        assert!(!unsafe_public.preflight_passed, "{unsafe_public:?}");
        assert!(!unsafe_public.promotion_ready, "{unsafe_public:?}");
        assert!(unsafe_public.missing_fields.contains(&"latency_ms"));
        assert!(unsafe_public
            .bound_violations
            .contains(&"public_target_without_authorization"));
        assert!(unsafe_public
            .bound_violations
            .contains(&"planned_clients_exceed_max"));
        assert!(unsafe_public
            .bound_violations
            .contains(&"duration_exceeds_max"));
    }

    #[test]
    fn multi_client_scenario_tracks_client_and_server_evidence() {
        let cfg = test_config(
            &["--scenario", "multi-client-load-score"],
            &[("CLIENT_TIMEOUT", "150")],
        )
        .expect("multi-client config parses");
        assert_eq!(
            planned_client_usernames(&cfg),
            vec!["compatbota", "compatbotb"]
        );
        assert_eq!(client_timeout_secs(&cfg, 0), 150);
        assert_eq!(
            client_timeout_secs(&cfg, 1),
            MULTI_CLIENT_LOAD_PEER_TIMEOUT_SECS
        );

        let client = evaluate_scenario(
            Scenario::MultiClientLoadScore,
            "mc_compat_multi_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 1\n",
        );
        assert!(client.passed, "{client:?}");

        let server = evaluate_server_scenario(
            Scenario::MultiClientLoadScore,
            "compatbota joined\ncompatbotb joined\nred flag captured\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");

        let missing_peer = evaluate_server_scenario(
            Scenario::MultiClientLoadScore,
            "compatbota joined\nred flag captured\n",
            "compatbot",
        );
        assert!(!missing_peer.passed, "{missing_peer:?}");
        assert!(missing_peer
            .missing_milestones
            .contains(&"server_client_b_seen"));
    }

    #[test]
    fn combat_damage_scenario_tracks_client_and_server_evidence() {
        let cfg = test_config(
            &["--scenario", "combat-damage"],
            &[("CLIENT_TIMEOUT", "150")],
        )
        .expect("combat config parses");
        assert_eq!(
            planned_client_usernames(&cfg),
            vec!["compatbota", "compatbotb"]
        );

        let client = evaluate_scenario(
            Scenario::CombatDamage,
            "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ncombat_probe_attack_sent\nupdate_health health=16.0\n",
        );
        assert!(client.passed, "{client:?}");

        let server = evaluate_server_scenario(
            Scenario::CombatDamage,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=4.0 victim_health_before=20.0 victim_health_after=16.0 attacker_item=WoodenSword\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");

        let missing_damage = evaluate_server_scenario(
            Scenario::CombatDamage,
            "compatbota joined\ncompatbotb joined\n",
            "compatbot",
        );
        assert!(!missing_damage.passed, "{missing_damage:?}");
        assert!(missing_damage
            .missing_milestones
            .contains(&"server_combat_damage"));
    }

    #[test]
    fn armor_loadout_enchantment_status_matrix_tracks_isolated_row_evidence() {
        let cfg = test_config(
            &["--scenario", "armor-loadout-enchantment-status-matrix"],
            &[("CLIENT_TIMEOUT", "150")],
        )
        .expect("armor matrix config parses");
        assert_eq!(
            planned_client_usernames(&cfg),
            vec!["compatbota", "compatbotb"]
        );

        let client = evaluate_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ninventory_probe_set_slot\ncombat_probe_attack_sent\nupdate_health health=18.0\n",
        );
        assert!(client.passed, "{client:?}");
        assert!(client.missing_milestones.is_empty());

        let server = evaluate_server_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate source=team_inventory_setup\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");
        assert!(server.missing_milestones.is_empty());

        let missing_equipment = evaluate_server_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
            "compatbot",
        );
        assert!(!missing_equipment.passed, "{missing_equipment:?}");
        assert!(missing_equipment
            .missing_milestones
            .contains(&"server_equipment_state"));
    }

    #[test]
    fn armor_loadout_enchantment_status_matrix_receipt_keeps_nonclaims() {
        let cfg = test_config(
            &["--scenario", "armor-loadout-enchantment-status-matrix"],
            &[],
        )
        .expect("armor matrix config parses");
        let scenario = evaluate_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "mc_compat_combat_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\ninventory_probe_set_slot\ncombat_probe_attack_sent\nupdate_health health=18.0\n",
        );
        let server = evaluate_server_scenario(
            Scenario::ArmorLoadoutEnchantmentStatusMatrix,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE armor_equipment_state username=compatbotb slot=chest item=DiamondChestplate source=team_inventory_setup\nMC-COMPAT-MILESTONE combat_damage attacker=compatbota victim=compatbotb damage=2.0 victim_health_before=20.0 victim_health_after=18.0 attacker_item=WoodenSword\nMC-COMPAT-MILESTONE combat_armor_mitigation attacker=compatbota victim=compatbotb base_damage=4.0 mitigation=2.0 final_damage=2.0 chest_item=DiamondChestplate victim_health_before=20.0 victim_health_after=18.0\n",
            "compatbot",
        );
        let matrix = evaluate_armor_loadout_enchantment_status_matrix(&cfg, &scenario, &server);
        assert!(matrix.selected, "{matrix:?}");
        assert!(!matrix.live_receipt, "{matrix:?}");
        assert!(!matrix.promotion_ready, "{matrix:?}");
        assert_eq!(matrix.row_id, ARMOR_MATRIX_ROW_ID);
        let json = render_armor_loadout_enchantment_status_matrix_json(&matrix);
        assert!(
            json.contains("\"row_id\": \"chest_diamond_none_none_melee\""),
            "{json}"
        );
        assert!(
            json.contains("\"loadout_id\": \"armor_loadout_chest_only\""),
            "{json}"
        );
        assert!(json.contains("\"reference_required\": false"), "{json}");
        assert!(json.contains("\"promotion_ready\": false"), "{json}");
        assert!(json.contains("\"all_enchantments\""), "{json}");
        assert!(json.contains("\"full_combat_correctness\""), "{json}");
    }

    #[test]
    fn projectile_damage_attribution_scenario_tracks_client_and_server_evidence() {
        let cfg = test_config(
            &["--scenario", "projectile-damage-attribution"],
            &[("CLIENT_TIMEOUT", "150")],
        )
        .expect("projectile damage config parses");
        assert_eq!(
            planned_client_usernames(&cfg),
            vec!["compatbota", "compatbotb"]
        );

        let client = evaluate_scenario(
            Scenario::ProjectileDamageAttribution,
            "mc_compat_projectile_damage_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\nprojectile_probe_use_item_sent\nprojectile_probe_swing_sent\nupdate_health health=17.0\n",
        );
        assert!(client.passed, "{client:?}");

        let server = evaluate_server_scenario(
            Scenario::ProjectileDamageAttribution,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE projectile_loadout username=compatbota slot=0 item=Bow arrows=16\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\nMC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");

        let missing_hit = evaluate_server_scenario(
            Scenario::ProjectileDamageAttribution,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE projectile_loadout username=compatbota slot=0 item=Bow arrows=16\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\n",
            "compatbot",
        );
        assert!(!missing_hit.passed, "{missing_hit:?}");
        assert!(missing_hit
            .missing_milestones
            .contains(&"server_projectile_hit"));

        let attacker_log = "projectile_probe_use_item_sent hand=main sequence=303\nprojectile_probe_swing_sent hand=main\n";
        let victim_log = "update_health health=17.0 food=20 saturation=0.0\n";
        let server_log = "MC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\nMC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\n";
        let client_logs = [
            ClientLogSlice {
                username: "compatbota",
                output: attacker_log,
            },
            ClientLogSlice {
                username: "compatbotb",
                output: victim_log,
            },
        ];
        let causal = evaluate_projectile_damage_causality(&client_logs, server_log, "compatbot");
        assert!(causal.passed, "{causal:?}");
        assert!(causal.missing_steps.is_empty(), "{causal:?}");
        assert!(causal.order_violations.is_empty(), "{causal:?}");

        let out_of_order_server = "MC-COMPAT-MILESTONE projectile_hit attacker=compatbota victim=compatbotb victim_health_before=20.0 victim_health_after=17.0\nMC-COMPAT-MILESTONE projectile_use attacker=compatbota victim=compatbotb hand=Main sequence=303 damage=3.0\n";
        let causal_order_fail =
            evaluate_projectile_damage_causality(&client_logs, out_of_order_server, "compatbot");
        assert!(!causal_order_fail.passed, "{causal_order_fail:?}");
        assert!(causal_order_fail
            .order_violations
            .contains(&"server_projectile_use_before_hit"));

        let missing_victim_health = evaluate_projectile_damage_causality(
            &[ClientLogSlice {
                username: "compatbota",
                output: attacker_log,
            }],
            server_log,
            "compatbot",
        );
        assert!(!missing_victim_health.passed, "{missing_victim_health:?}");
        assert!(missing_victim_health
            .missing_steps
            .contains(&"victim_client_damage_update"));
    }

    #[test]
    fn projectile_damage_dry_run_uses_steel_arrow_policy() {
        let mut cfg = test_config(
            &[
                "--scenario",
                "projectile-damage-attribution",
                "--valence-rev",
                PINNED_PROJECTILE_DAMAGE_VALENCE_REV,
            ],
            &[],
        )
        .expect("projectile damage config parses");
        cfg.arrow_damage_policy = runtime_config::ArrowDamagePolicy {
            base_damage: 4.0,
            velocity_multiplier: DEFAULT_ARROW_VELOCITY_MULTIPLIER,
            max_damage: DEFAULT_ARROW_MAX_DAMAGE,
        };

        let evidence = projectile_damage_dry_run_evidence(&cfg);
        assert!(
            evidence
                .scenario
                .as_ref()
                .expect("scenario evidence")
                .passed,
            "{evidence:?}"
        );
        let causality = evidence
            .projectile_damage_causality
            .as_ref()
            .expect("causality evidence");
        assert!(causality.passed, "{causality:?}");
        assert!(causality
            .observed_steps
            .contains(&"server_projectile_hit_attacker_victim_health_delta"));
    }

    #[test]
    fn projectile_damage_attribution_requires_pinned_valence_revision() {
        let cfg = test_config(
            &[
                "--dry-run",
                "--scenario",
                "projectile-damage-attribution",
                "--valence-rev",
                "HEAD",
            ],
            &[],
        )
        .expect("config parses before execution validation");
        let err = validate_projectile_damage_dependency(&cfg).unwrap_err();
        assert!(err.contains(PINNED_PROJECTILE_DAMAGE_VALENCE_REV), "{err}");

        let pinned = test_config(
            &[
                "--dry-run",
                "--scenario",
                "projectile-damage-attribution",
                "--valence-rev",
                PINNED_PROJECTILE_DAMAGE_VALENCE_REV,
            ],
            &[],
        )
        .expect("pinned config parses");
        validate_projectile_damage_dependency(&pinned).expect("pinned revision accepted");
    }

    #[test]
    fn equipment_update_scenario_tracks_current_client_equipment_marker() {
        let client = evaluate_scenario(
            Scenario::EquipmentUpdateObservation,
            "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\nequipment_probe_entity_equipment entity_id=4 entries=1 slots=slot4:id=829:count=1\n",
        );
        assert!(client.passed, "{client:?}");
        assert!(client.missing_milestones.is_empty());

        let missing_equipment = evaluate_scenario(
            Scenario::EquipmentUpdateObservation,
            "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn\n",
        );
        assert!(!missing_equipment.passed, "{missing_equipment:?}");
        assert!(missing_equipment
            .missing_milestones
            .contains(&"entity_equipment_update"));
    }

    #[test]
    fn equipment_slot_item_matrix_expansion_tracks_isolated_row_evidence() {
        let cfg = test_config(
            &["--scenario", "equipment-slot-item-matrix-expansion"],
            &[("CLIENT_TIMEOUT", "150")],
        )
        .expect("equipment matrix config parses");
        assert_eq!(
            planned_client_usernames(&cfg),
            vec!["compatbota", "compatbotb"]
        );

        let client = evaluate_scenario(
            Scenario::EquipmentSlotItemMatrixExpansion,
            "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn entity_id=4\nequipment_probe_entity_equipment entity_id=4 entries=1 slots=slot4:id=829:count=1\n",
        );
        assert!(client.passed, "{client:?}");
        assert!(client.missing_milestones.is_empty());

        let server = evaluate_server_scenario(
            Scenario::EquipmentSlotItemMatrixExpansion,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE equipment_update_state username=compatbotb slot=main_hand item_id=829 count=1\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");
        assert!(server.missing_milestones.is_empty());

        let missing_update = evaluate_scenario(
            Scenario::EquipmentSlotItemMatrixExpansion,
            "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn entity_id=4\n",
        );
        assert!(!missing_update.passed, "{missing_update:?}");
        assert!(missing_update
            .missing_milestones
            .contains(&"entity_equipment_update"));
    }

    #[test]
    fn equipment_slot_item_matrix_expansion_receipt_keeps_nonclaims() {
        let cfg = test_config(&["--scenario", "equipment-slot-item-matrix-expansion"], &[])
            .expect("equipment matrix config parses");
        let scenario = evaluate_scenario(
            Scenario::EquipmentSlotItemMatrixExpansion,
            "mc_compat_equipment_update_client_count=2\nDetected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou are on team BLUE!\nremote_player_spawn entity_id=4\nequipment_probe_entity_equipment entity_id=4 entries=1 slots=slot4:id=829:count=1\n",
        );
        let server = evaluate_server_scenario(
            Scenario::EquipmentSlotItemMatrixExpansion,
            "compatbota joined\ncompatbotb joined\nMC-COMPAT-MILESTONE equipment_update_state username=compatbotb slot=main_hand item_id=829 count=1\n",
            "compatbot",
        );
        let matrix = evaluate_equipment_slot_item_matrix_expansion(&cfg, &scenario, &server);
        assert!(matrix.selected, "{matrix:?}");
        assert!(!matrix.live_receipt, "{matrix:?}");
        assert!(!matrix.promotion_ready, "{matrix:?}");
        assert_eq!(matrix.row_id, EQUIPMENT_MATRIX_ROW_ID);
        let json = render_equipment_slot_item_matrix_expansion_json(&matrix);
        assert!(
            json.contains("\"row_id\": \"remote_main_hand_slot4_item829_count1_non_empty\""),
            "{json}"
        );
        assert!(json.contains("\"wire_slot\": \"slot4\""), "{json}");
        assert!(json.contains("\"item_id\": \"829\""), "{json}");
        assert!(json.contains("\"promotion_ready\": false"), "{json}");
        assert!(json.contains("\"all_equipment_slots\""), "{json}");
        assert!(json.contains("\"full_equipment_semantics\""), "{json}");
    }

    #[test]
    fn blue_flag_score_scenario_tracks_mirrored_team_evidence() {
        let pass = evaluate_scenario(
            Scenario::BlueFlagScore,
            "Detected server protocol version 763
join_game
render_tick_with_player
You are on team BLUE!
You have the flag!
You captured the flag!
BLUE: 1
",
        );
        assert!(pass.passed, "{pass:?}");
        assert!(pass.missing_milestones.is_empty());

        let fail = evaluate_scenario(
            Scenario::BlueFlagScore,
            "Detected server protocol version 763
join_game
render_tick_with_player
You are on team RED!
You have the flag!
You captured the flag!
RED: 1
",
        );
        assert!(!fail.passed);
        assert!(fail.missing_milestones.contains(&"team_blue"));
        assert!(fail.missing_milestones.contains(&"score_blue_1"));
    }

    #[test]
    fn inventory_interaction_scenario_tracks_client_and_server_evidence() {
        let client = evaluate_scenario(
            Scenario::InventoryInteraction,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\ninventory_probe_set_slot\ninventory_probe_slot36_nonempty\ninventory_probe_slot37_stack\ninventory_probe_drop_item_sent\ninventory_probe_collect_item\ninventory_probe_click_slot_sent\ninventory_probe_open_container\ninventory_probe_container_click_sent\ninventory_probe_place_block_sent\n",
        );
        assert!(client.passed, "{client:?}");

        let missing_drop = evaluate_scenario(
            Scenario::InventoryInteraction,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\ninventory_probe_set_slot\ninventory_probe_slot36_nonempty\ninventory_probe_slot37_stack\n",
        );
        assert!(!missing_drop.passed);
        assert!(missing_drop
            .missing_milestones
            .contains(&"inventory_drop_sent"));

        let server = evaluate_server_scenario(
            Scenario::InventoryInteraction,
            "compatbot joined\nMC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0\nMC-COMPAT-MILESTONE inventory_drop_item username=compatbot from_slot=36 item=WoodenSword count=1\nMC-COMPAT-MILESTONE inventory_pickup_item username=compatbot from_slot=36 item=WoodenSword count=1 collected_entity_id=7630036 collector_entity_id=1\nMC-COMPAT-MILESTONE inventory_click_slot username=compatbot window=0 slot=37 button=0 mode=click carried_item=RedWool count=63 slot_after=empty\nMC-COMPAT-MILESTONE inventory_open_container username=compatbot kind=Generic3x3 trigger=inventory_click_slot\nMC-COMPAT-MILESTONE inventory_container_click username=compatbot window=1 slot=0 button=0 mode=click carried_item=Air count=0 slot_changes=1\nMC-COMPAT-MILESTONE block_place_item username=compatbot item=RedWool from_slot=37 block=RedWool at=-40,65,0\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");

        let missing_drop = evaluate_server_scenario(
            Scenario::InventoryInteraction,
            "compatbot joined\nMC-COMPAT-MILESTONE inventory_hotbar_select username=compatbot slot=0\n",
            "compatbot",
        );
        assert!(!missing_drop.passed, "{missing_drop:?}");
        assert!(missing_drop
            .missing_milestones
            .contains(&"server_inventory_drop"));
    }

    #[test]
    fn survival_break_place_pickup_scenario_tracks_client_and_server_evidence() {
        let client = evaluate_scenario(
            Scenario::SurvivalBreakPlacePickup,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_probe_break_block_sent\nsurvival_probe_block_update\nsurvival_probe_pickup_seen\nsurvival_probe_place_block_sent\nsurvival_probe_place_update\n",
        );
        assert!(client.passed, "{client:?}");
        assert!(client.missing_milestones.is_empty());

        let missing_pickup = evaluate_scenario(
            Scenario::SurvivalBreakPlacePickup,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_probe_break_block_sent\nsurvival_probe_block_update\n",
        );
        assert!(!missing_pickup.passed, "{missing_pickup:?}");
        assert!(missing_pickup
            .missing_milestones
            .contains(&"survival_pickup_seen"));

        let server = evaluate_server_scenario(
            Scenario::SurvivalBreakPlacePickup,
            "compatbot joined\nMC-COMPAT-MILESTONE survival_join username=compatbot gamemode=Survival\nMC-COMPAT-MILESTONE survival_block_break username=compatbot item=Dirt at=0,64,1\nMC-COMPAT-MILESTONE survival_pickup_item username=compatbot slot=36 item=Dirt count=1\nMC-COMPAT-MILESTONE survival_block_place username=compatbot item=Dirt from_slot=36 at=0,65,1\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");

        let missing_place = evaluate_server_scenario(
            Scenario::SurvivalBreakPlacePickup,
            "compatbot joined\nMC-COMPAT-MILESTONE survival_join username=compatbot gamemode=Survival\nMC-COMPAT-MILESTONE survival_block_break username=compatbot item=Dirt at=0,64,1\n",
            "compatbot",
        );
        assert!(!missing_place.passed, "{missing_place:?}");
        assert!(missing_place
            .missing_milestones
            .contains(&"server_survival_place"));
    }

    #[test]
    fn survival_chest_persistence_scenario_tracks_client_and_server_evidence() {
        let client = evaluate_scenario(
            Scenario::SurvivalChestPersistence,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_chest_open_seen window=1 position=8,64,0\nsurvival_chest_store_sent window=1 slot=0 item=Dirt count=1\nsurvival_chest_close_sent window=1\nsurvival_chest_reconnect_sent session=1\nsurvival_chest_reopen_seen window=1 position=8,64,0\nsurvival_chest_persisted_seen window=1 slot=0 item=Dirt count=1\n",
        );
        assert!(client.passed, "{client:?}");
        assert!(client.missing_milestones.is_empty());

        let missing_reopen = evaluate_scenario(
            Scenario::SurvivalChestPersistence,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_chest_open_seen window=1 position=8,64,0\nsurvival_chest_store_sent window=1 slot=0 item=Dirt count=1\nsurvival_chest_close_sent window=1\nsurvival_chest_reconnect_sent session=1\n",
        );
        assert!(!missing_reopen.passed, "{missing_reopen:?}");
        assert!(missing_reopen
            .missing_milestones
            .contains(&"survival_chest_reopen_seen"));

        let wrong_client_values = evaluate_scenario(
            Scenario::SurvivalChestPersistence,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_chest_open_seen window=1 position=9,64,0\nsurvival_chest_store_sent window=1 slot=1 item=Stone count=2\nsurvival_chest_close_sent window=1\nsurvival_chest_reconnect_sent session=2\nsurvival_chest_reopen_seen window=1 position=9,64,0\nsurvival_chest_persisted_seen window=1 slot=1 item=Stone count=2\n",
        );
        assert!(!wrong_client_values.passed, "{wrong_client_values:?}");
        assert!(wrong_client_values
            .missing_milestones
            .contains(&"survival_chest_open_seen"));
        assert!(wrong_client_values
            .missing_milestones
            .contains(&"survival_chest_store_sent"));
        assert!(wrong_client_values
            .missing_milestones
            .contains(&"survival_chest_reconnect_sent"));
        assert!(wrong_client_values
            .missing_milestones
            .contains(&"survival_chest_reopen_seen"));
        assert!(wrong_client_values
            .missing_milestones
            .contains(&"survival_chest_persisted_seen"));

        let wrong_reopen_window = evaluate_scenario(
            Scenario::SurvivalChestPersistence,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nsurvival_chest_open_seen window=1 position=8,64,0\nsurvival_chest_store_sent window=1 slot=0 item=Dirt count=1\nsurvival_chest_close_sent window=1\nsurvival_chest_reconnect_sent session=1\nsurvival_chest_reopen_seen window=3 position=8,64,0\nsurvival_chest_persisted_seen window=3 slot=0 item=Dirt count=1\n",
        );
        assert!(!wrong_reopen_window.passed, "{wrong_reopen_window:?}");
        assert!(wrong_reopen_window
            .missing_milestones
            .contains(&"survival_chest_reopen_seen"));
        assert!(wrong_reopen_window
            .missing_milestones
            .contains(&"survival_chest_persisted_seen"));

        let server = evaluate_server_scenario(
            Scenario::SurvivalChestPersistence,
            "compatbot joined\nMC-COMPAT-MILESTONE survival_chest_open username=compatbot position=8,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_store username=compatbot window=1 slot=0 item=Dirt count=1\nMC-COMPAT-MILESTONE survival_chest_close username=compatbot window=1\nMC-COMPAT-MILESTONE survival_chest_reopen username=compatbot position=8,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_persisted username=compatbot slot=0 item=Dirt count=1\n",
            "compatbot",
        );
        assert!(server.passed, "{server:?}");

        let missing_store = evaluate_server_scenario(
            Scenario::SurvivalChestPersistence,
            "compatbot joined\nMC-COMPAT-MILESTONE survival_chest_open username=compatbot position=8,64,0 window=1\n",
            "compatbot",
        );
        assert!(!missing_store.passed, "{missing_store:?}");
        assert!(missing_store
            .missing_milestones
            .contains(&"server_survival_chest_store"));

        let wrong_server_values = evaluate_server_scenario(
            Scenario::SurvivalChestPersistence,
            "compatbot joined\nMC-COMPAT-MILESTONE survival_chest_open username=compatbot position=9,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_store username=compatbot window=1 slot=1 item=Stone count=2\nMC-COMPAT-MILESTONE survival_chest_close username=compatbot window=1\nMC-COMPAT-MILESTONE survival_chest_reopen username=compatbot position=9,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_persisted username=compatbot slot=1 item=Stone count=2\n",
            "compatbot",
        );
        assert!(!wrong_server_values.passed, "{wrong_server_values:?}");
        assert!(wrong_server_values
            .missing_milestones
            .contains(&"server_survival_chest_open"));
        assert!(wrong_server_values
            .missing_milestones
            .contains(&"server_survival_chest_store"));
        assert!(wrong_server_values
            .missing_milestones
            .contains(&"server_survival_chest_reopen"));
        assert!(wrong_server_values
            .missing_milestones
            .contains(&"server_survival_chest_persisted"));

        let wrong_server_reopen_window = evaluate_server_scenario(
            Scenario::SurvivalChestPersistence,
            "compatbot joined\nMC-COMPAT-MILESTONE survival_chest_open username=compatbot position=8,64,0 window=1\nMC-COMPAT-MILESTONE survival_chest_store username=compatbot window=1 slot=0 item=Dirt count=1\nMC-COMPAT-MILESTONE survival_chest_close username=compatbot window=1\nMC-COMPAT-MILESTONE survival_chest_reopen username=compatbot position=8,64,0 window=3\nMC-COMPAT-MILESTONE survival_chest_persisted username=compatbot slot=0 item=Dirt count=1\n",
            "compatbot",
        );
        assert!(
            !wrong_server_reopen_window.passed,
            "{wrong_server_reopen_window:?}"
        );
        assert!(wrong_server_reopen_window
            .missing_milestones
            .contains(&"server_survival_chest_reopen"));
    }

    #[test]
    fn flag_score_repeat_scenario_tracks_missing_and_forbidden_evidence() {
        let pass = evaluate_scenario(
            Scenario::FlagScoreRepeat,
            "Detected server protocol version 763\njoin_game\nrender_tick_with_player\nYou are on team RED!\nYou have the flag!\nYou captured the flag!\nRED: 1\nRED: 2\n",
        );
        assert!(pass.passed, "{pass:?}");
        assert_eq!(pass.missing_milestones, Vec::<&str>::new());
        assert_eq!(pass.forbidden_matches, Vec::<&str>::new());

        let fail = evaluate_scenario(
            Scenario::FlagScoreRepeat,
            "Detected server protocol version 763\njoin_game\nUnexpectedEof\n",
        );
        assert!(!fail.passed, "{fail:?}");
        assert!(fail.missing_milestones.contains(&"render_tick"));
        assert!(fail.missing_milestones.contains(&"score_red_2"));
        assert_eq!(fail.forbidden_matches, vec!["unexpected_eof"]);
    }

    #[test]
    fn missing_valence_checkout_has_actionable_diagnostic() {
        let missing =
            std::env::temp_dir().join(format!("mc-compat-missing-valence-{}", std::process::id()));
        let cfg = test_config(&["--valence-repo", missing.to_str().unwrap()], &[])
            .expect("config with missing Valence repo parses");

        let err = ensure_valence_repo_ready(&cfg).unwrap_err();

        assert!(err.contains("Valence checkout not found"), "{err}");
        assert!(
            err.contains("git clone https://github.com/valence-rs/valence"),
            "{err}"
        );
        assert!(err.contains("--valence-repo/VALENCE_REPO"), "{err}");
    }

    #[test]
    fn missing_client_checkout_has_actionable_diagnostic() {
        let missing = std::env::temp_dir().join(format!(
            "mc-compat-missing-stevenarella-{}",
            std::process::id()
        ));
        let cfg = test_config(&["--client-dir", missing.to_str().unwrap()], &[])
            .expect("config with missing Stevenarella checkout parses");

        let err = ensure_client_dir_ready(&cfg).unwrap_err();

        assert!(err.contains("Stevenarella checkout not found"), "{err}");
        assert!(
            err.contains("git clone https://github.com/iceiix/stevenarella"),
            "{err}"
        );
        assert!(err.contains("--client-dir/CLIENT_DIR"), "{err}");
    }

    #[test]
    fn client_checkout_must_point_at_manifest_root() {
        let dir =
            std::env::temp_dir().join(format!("mc-compat-bad-stevenarella-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create bad Stevenarella checkout");
        let cfg = test_config(&["--client-dir", dir.to_str().unwrap()], &[])
            .expect("config with bad Stevenarella checkout parses");

        let err = ensure_client_dir_ready(&cfg).unwrap_err();

        assert!(err.contains("missing Cargo.toml"), "{err}");
        assert!(err.contains("Stevenarella repository root"), "{err}");
    }

    #[test]
    fn valid_client_checkout_preflight_passes() {
        let dir = fake_stevenarella_checkout("valid");
        let cfg = test_config(&["--client-dir", dir.to_str().unwrap()], &[])
            .expect("config with fake Stevenarella checkout parses");

        ensure_client_dir_ready(&cfg).expect("fake checkout has a manifest");
    }

    #[test]
    fn smoke_receipt_records_cairn_contract_and_octet_surface() {
        let mut cfg = test_config(
            &[
                "--server-backend=paper",
                "--receipt",
                "/tmp/receipt.json",
                "--client-dir",
                "/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        cfg.server_port = 25566;
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "timeout-success-evidence",
            matched_success_pattern: Some("Detected server protocol version".to_string()),
            scenario: Some(evaluate_scenario(
                Scenario::Smoke,
                "Detected server protocol version",
            )),
            server_scenario: Some(evaluate_server_scenario(Scenario::Smoke, "", "compatbot")),
            projectile_damage_causality: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));

        assert!(
            json.contains("\"schema\": \"mc.compat.scenario.receipt.v2\""),
            "{json}"
        );
        assert!(
            json.contains("\"cairn_contract\": \"mc.compat.scenario.receipt.v2\""),
            "{json}"
        );
        assert!(
            json.contains("\"octet_producer_surface\": \"tools/mc-compat-runner/src/main.rs\""),
            "{json}"
        );
        assert!(
            json.contains("\"classification\": \"timeout-success-evidence\""),
            "{json}"
        );
        assert!(
            json.contains("\"matched_success_pattern\": \"Detected server protocol version\""),
            "{json}"
        );
        assert!(json.contains("\"name\": \"smoke\""), "{json}");
        assert!(
            json.contains("\"observed_milestones\": [\"protocol_detected\"]"),
            "{json}"
        );
        assert!(json.contains("\"passed\": true"), "{json}");
        assert!(json.contains("\"client_server_correlation\""), "{json}");
        assert!(json.contains("\"usernames\": [\"compatbot\"]"), "{json}");
        assert!(
            json.contains("\"log_paths\": [\"/tmp/client.log\"]"),
            "{json}"
        );
        assert!(json.contains("\"triage\""), "{json}");
        assert!(json.contains("\"suggested_boundary\": \"none\""), "{json}");
        assert!(
            json.contains("\"wayland_socket_inherited\": false"),
            "{json}"
        );
    }

    #[test]
    fn scenario_receipt_records_actionable_failure_triage() {
        let mut cfg = test_config(
            &[
                "--server-backend=valence",
                "--scenario=flag-score-repeat",
                "--receipt=/tmp/receipt.json",
                "--client-dir=/tmp/stevenarella",
            ],
            &[],
        )
        .expect("receipt config parses");
        cfg.valence_log = PathBuf::from("/tmp/valence.log");
        let client = Some(ClientRunEvidence {
            log_path: Some(PathBuf::from("/tmp/client.log")),
            log_paths: vec![PathBuf::from("/tmp/client.log")],
            usernames: vec!["compatbot".to_string()],
            exit_code: Some(124),
            classification: "failure-missing-scenario-evidence",
            matched_success_pattern: None,
            scenario: Some(evaluate_scenario(
                Scenario::FlagScoreRepeat,
                "Detected server protocol version 763\njoin_game\nUnexpectedEof\n",
            )),
            server_scenario: Some(evaluate_server_scenario(
                Scenario::FlagScoreRepeat,
                "compatbot joined\n",
                "compatbot",
            )),
            projectile_damage_causality: None,
        });

        let json = smoke_receipt_json(&cfg, Ok(&client));

        assert!(
            json.contains("\"first_missing_client_milestone\": \"render_tick\""),
            "{json}"
        );
        assert!(
            json.contains("\"first_missing_server_milestone\": \"server_flag_or_score\""),
            "{json}"
        );
        assert!(
            json.contains("\"first_forbidden_pattern\": \"unexpected_eof\""),
            "{json}"
        );
        assert!(
            json.contains("\"first_forbidden_source\": \"client\""),
            "{json}"
        );
        assert!(
            json.contains("\"suggested_boundary\": \"protocol-runtime\""),
            "{json}"
        );
        assert!(
            json.contains("\"client_log_paths\": [\"/tmp/client.log\"]"),
            "{json}"
        );
        assert!(
            json.contains("\"server_log_path\": \"/tmp/valence.log\""),
            "{json}"
        );
    }

    #[test]
    fn failed_preflight_receipt_triages_before_client_evidence() {
        let cfg =
            test_config(&["--receipt=/tmp/receipt.json"], &[]).expect("receipt config parses");
        let err = "server status probe failed".to_string();

        let json = smoke_receipt_json(&cfg, Err(&err));

        assert!(
            json.contains("\"first_missing_client_milestone\": \"protocol_detected\""),
            "{json}"
        );
        assert!(
            json.contains("\"suggested_boundary\": \"preflight-or-server-startup\""),
            "{json}"
        );
    }

    fn receipt_fixture(backend: &str, protocol: u32, port: u16) -> String {
        format!(
            "{{\n  \"schema\": \"mc.compat.smoke.receipt.v1\",\n  \"status\": \"pass\",\n  \"mode\": \"run\",\n  \"dry_run\": false,\n  \"contract\": {{\n    \"claims_correctness\": false,\n    \"claims_semantic_equivalence\": false\n  }},\n  \"server\": {{\n    \"backend\": \"{backend}\",\n    \"version\": \"1.18.2\",\n    \"protocol\": {protocol},\n    \"port\": {port}\n  }},\n  \"client\": {{\n    \"headless_isolation\": {{\n      \"xvfb\": true,\n      \"x11_backend\": true,\n      \"software_gl\": true,\n      \"wayland_socket_inherited\": false\n    }},\n    \"classification\": \"timeout-success-evidence\",\n    \"matched_success_pattern\": \"Detected server protocol version\"\n  }},\n  \"error\": null\n}}\n"
        )
    }

    #[test]
    fn compares_paper_and_valence_receipts() {
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", 758, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", 758, 25565),
        )
        .expect("valence fixture parses");

        validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL)
            .expect("matching receipts compare");
    }

    #[test]
    fn rejects_receipt_protocol_mismatch() {
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", 758, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", 759, 25565),
        )
        .expect("valence fixture parses");

        let err = validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL).unwrap_err();
        assert!(err.contains("receipt protocol mismatch"), "{err}");
    }

    #[test]
    fn compares_protocol_763_matrix_receipts_when_configured() {
        const PROTOCOL_763: u32 = 763;
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", PROTOCOL_763, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", PROTOCOL_763, 25565),
        )
        .expect("valence fixture parses");

        validate_receipt_pair(&paper, &valence, PROTOCOL_763)
            .expect("configured protocol receipts compare");
    }

    #[test]
    fn receipt_summary_mutations_fail_closed() {
        let missing_success = read_receipt_summary_from_text(
            PathBuf::from("missing-success.json"),
            &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566).replace(
                "\"matched_success_pattern\": \"Detected server protocol version\"",
                "\"matched_success_pattern\": null",
            ),
        )
        .expect("missing success fixture parses");
        let err = validate_receipt_summary(&missing_success).unwrap_err();
        assert!(
            err.contains("missing matched client success pattern"),
            "{err}"
        );

        let bad_headless = read_receipt_summary_from_text(
            PathBuf::from("bad-headless.json"),
            &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566).replace(
                "\"wayland_socket_inherited\": false",
                "\"wayland_socket_inherited\": true",
            ),
        )
        .expect("bad headless fixture parses");
        let err = validate_receipt_summary(&bad_headless).unwrap_err();
        assert!(err.contains("headless isolation"), "{err}");

        let failed_status = read_receipt_summary_from_text(
            PathBuf::from("failed-status.json"),
            &receipt_fixture("paper", DEFAULT_SERVER_PROTOCOL, 25566)
                .replace("\"status\": \"pass\"", "\"status\": \"fail\""),
        )
        .expect("failed status fixture parses");
        let err = validate_receipt_summary(&failed_status).unwrap_err();
        assert!(err.contains("did not pass"), "{err}");
    }

    #[test]
    fn rejects_receipts_that_do_not_match_configured_protocol() {
        const PROTOCOL_763: u32 = 763;
        let paper = read_receipt_summary_from_text(
            PathBuf::from("paper.json"),
            &receipt_fixture("paper", PROTOCOL_763, 25566),
        )
        .expect("paper fixture parses");
        let valence = read_receipt_summary_from_text(
            PathBuf::from("valence.json"),
            &receipt_fixture("valence", PROTOCOL_763, 25565),
        )
        .expect("valence fixture parses");

        let err = validate_receipt_pair(&paper, &valence, DEFAULT_SERVER_PROTOCOL).unwrap_err();
        assert!(
            err.contains(&format!(
                "expected protocol {DEFAULT_SERVER_PROTOCOL}, got {PROTOCOL_763}"
            )),
            "{err}"
        );
    }

    #[test]
    fn smoke_receipt_records_failures_without_success_claims() {
        let cfg =
            test_config(&["--receipt=/tmp/receipt.json"], &[]).expect("receipt config parses");
        let err = "server status probe failed".to_string();

        let json = smoke_receipt_json(&cfg, Err(&err));

        assert!(json.contains("\"status\": \"fail\""), "{json}");
        assert!(json.contains("\"classification\": null"), "{json}");
        assert!(
            json.contains("\"error\": \"server status probe failed\""),
            "{json}"
        );
        assert!(json.contains("\"claims_correctness\": false"), "{json}");
        assert!(
            json.contains("\"claims_semantic_equivalence\": false"),
            "{json}"
        );
    }
}
