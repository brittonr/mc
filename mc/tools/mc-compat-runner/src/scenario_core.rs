use crate::{
    scenario_manifest_generated, CTF_RACE_ACCEPTED_SERVER_NEEDLE, CTF_RACE_CLIENT_COUNT_NEEDLE,
    CTF_RACE_FINAL_SERVER_NEEDLE, CTF_RACE_REJECTED_SERVER_NEEDLE,
    CTF_SCORE_LIMIT_CLIENT_WIN_NEEDLE, CTF_SCORE_LIMIT_SERVER_FINAL_CAPTURE_NEEDLE,
    CTF_SCORE_LIMIT_SERVER_PRE_STATE_NEEDLE, CTF_SCORE_LIMIT_SERVER_WIN_NEEDLE,
    CTF_SPAWN_RESOURCE_RESET_NEEDLE, CTF_SPAWN_TEAM_BALANCE_NEEDLE,
    CTF_SPAWN_TEAM_BLUE_ASSIGNMENT_NEEDLE, CTF_SPAWN_TEAM_RED_ASSIGNMENT_NEEDLE,
    CTF_SPAWN_TEAM_RESET_CLIENT_COUNT_NEEDLE, INVENTORY_DRAG_CLIENT_END_NEEDLE,
    INVENTORY_DRAG_CLIENT_FINAL_NEEDLE, INVENTORY_DRAG_CLIENT_INITIAL_NEEDLE,
    INVENTORY_DRAG_CLIENT_PICKUP_NEEDLE, INVENTORY_DRAG_CLIENT_SOURCE_EMPTY_NEEDLE,
    INVENTORY_DRAG_CLIENT_START_NEEDLE, INVENTORY_DRAG_CLIENT_TARGET_A_NEEDLE,
    INVENTORY_DRAG_CLIENT_TARGET_B_NEEDLE, INVENTORY_DRAG_SERVER_END_NEEDLE,
    INVENTORY_DRAG_SERVER_PICKUP_NEEDLE, INVENTORY_DRAG_SERVER_START_NEEDLE,
    INVENTORY_DRAG_SERVER_TARGET_A_NEEDLE, INVENTORY_DRAG_SERVER_TARGET_B_NEEDLE,
    INVENTORY_STACK_CLIENT_DESTINATION_NEEDLE, INVENTORY_STACK_CLIENT_FINAL_NEEDLE,
    INVENTORY_STACK_CLIENT_INITIAL_NEEDLE, INVENTORY_STACK_CLIENT_MERGE_EMPTY_NEEDLE,
    INVENTORY_STACK_CLIENT_MERGE_PICKUP_NEEDLE, INVENTORY_STACK_CLIENT_MERGE_PLACE_NEEDLE,
    INVENTORY_STACK_CLIENT_SPLIT_PICKUP_NEEDLE, INVENTORY_STACK_CLIENT_SPLIT_PLACE_NEEDLE,
    INVENTORY_STACK_CLIENT_SPLIT_SOURCE_NEEDLE, INVENTORY_STACK_SERVER_MERGE_NEEDLE,
    INVENTORY_STACK_SERVER_MERGE_PICKUP_NEEDLE, INVENTORY_STACK_SERVER_SPLIT_NEEDLE,
    INVENTORY_STACK_SERVER_SPLIT_PICKUP_NEEDLE, MCP_CONTROLLED_SMOKE_SCENARIO,
    SURVIVAL_BIOME_DIMENSION_CLIENT_STATE_NEEDLE, SURVIVAL_BIOME_DIMENSION_SERVER_STATE_NEEDLE,
    SURVIVAL_BLOCK_ENTITY_CLIENT_POST_RESTART_NEEDLE,
    SURVIVAL_BLOCK_ENTITY_CLIENT_PRE_RESTART_NEEDLE, SURVIVAL_BLOCK_ENTITY_CLIENT_RECONNECT_NEEDLE,
    SURVIVAL_BLOCK_ENTITY_SERVER_CLEAN_NEEDLE, SURVIVAL_BLOCK_ENTITY_SERVER_MUTATION_NEEDLE,
    SURVIVAL_BLOCK_ENTITY_SERVER_POST_NEEDLE, SURVIVAL_BLOCK_ENTITY_SERVER_RESTART_NEEDLE,
    SURVIVAL_BLOCK_ENTITY_SERVER_STATE_NEEDLE, SURVIVAL_CHEST_CLIENT_CLOSE_NEEDLE,
    SURVIVAL_CHEST_CLIENT_OPEN_NEEDLE, SURVIVAL_CHEST_CLIENT_PERSISTED_NEEDLE,
    SURVIVAL_CHEST_CLIENT_RECONNECT_NEEDLE, SURVIVAL_CHEST_CLIENT_REOPEN_NEEDLE,
    SURVIVAL_CHEST_CLIENT_STORE_NEEDLE, SURVIVAL_CHEST_SERVER_CLOSE_NEEDLE,
    SURVIVAL_CHEST_SERVER_OPEN_NEEDLE, SURVIVAL_CHEST_SERVER_PERSISTED_NEEDLE,
    SURVIVAL_CHEST_SERVER_REOPEN_NEEDLE, SURVIVAL_CHEST_SERVER_STORE_NEEDLE,
    SURVIVAL_CRAFTING_CLIENT_COLLECT_NEEDLE, SURVIVAL_CRAFTING_CLIENT_INPUT_A_NEEDLE,
    SURVIVAL_CRAFTING_CLIENT_INPUT_B_NEEDLE, SURVIVAL_CRAFTING_CLIENT_INVENTORY_NEEDLE,
    SURVIVAL_CRAFTING_CLIENT_OPEN_NEEDLE, SURVIVAL_CRAFTING_CLIENT_RESULT_NEEDLE,
    SURVIVAL_CRAFTING_SERVER_COLLECT_NEEDLE, SURVIVAL_CRAFTING_SERVER_INPUT_A_NEEDLE,
    SURVIVAL_CRAFTING_SERVER_INPUT_B_NEEDLE, SURVIVAL_CRAFTING_SERVER_OPEN_NEEDLE,
    SURVIVAL_CRAFTING_SERVER_RESULT_NEEDLE, SURVIVAL_CRASH_RECOVERY_CLIENT_MUTATION_NEEDLE,
    SURVIVAL_CRASH_RECOVERY_CLIENT_POST_CRASH_NEEDLE,
    SURVIVAL_CRASH_RECOVERY_CLIENT_PRE_CRASH_NEEDLE,
    SURVIVAL_CRASH_RECOVERY_CLIENT_RECONNECT_NEEDLE,
    SURVIVAL_CRASH_RECOVERY_SERVER_FORCED_STOP_NEEDLE,
    SURVIVAL_CRASH_RECOVERY_SERVER_MUTATION_NEEDLE, SURVIVAL_CRASH_RECOVERY_SERVER_POST_NEEDLE,
    SURVIVAL_CRASH_RECOVERY_SERVER_RESTART_NEEDLE, SURVIVAL_CRASH_RECOVERY_SERVER_STATE_NEEDLE,
    SURVIVAL_FURNACE_CLIENT_BURN_NEEDLE, SURVIVAL_FURNACE_CLIENT_COLLECT_NEEDLE,
    SURVIVAL_FURNACE_CLIENT_FUEL_NEEDLE, SURVIVAL_FURNACE_CLIENT_INPUT_NEEDLE,
    SURVIVAL_FURNACE_CLIENT_INVENTORY_NEEDLE, SURVIVAL_FURNACE_CLIENT_OPEN_NEEDLE,
    SURVIVAL_FURNACE_CLIENT_OUTPUT_NEEDLE, SURVIVAL_FURNACE_CLIENT_RECONNECT_NEEDLE,
    SURVIVAL_FURNACE_CLIENT_REOPEN_NEEDLE, SURVIVAL_FURNACE_SERVER_BURN_NEEDLE,
    SURVIVAL_FURNACE_SERVER_COLLECT_NEEDLE, SURVIVAL_FURNACE_SERVER_FUEL_NEEDLE,
    SURVIVAL_FURNACE_SERVER_INPUT_NEEDLE, SURVIVAL_FURNACE_SERVER_OPEN_NEEDLE,
    SURVIVAL_FURNACE_SERVER_OUTPUT_NEEDLE, SURVIVAL_FURNACE_SERVER_REOPEN_NEEDLE,
    SURVIVAL_FURNACE_SERVER_STATE_NEEDLE, SURVIVAL_HUNGER_FOOD_CLIENT_INVENTORY_NEEDLE,
    SURVIVAL_HUNGER_FOOD_CLIENT_ITEM_NEEDLE, SURVIVAL_HUNGER_FOOD_CLIENT_POST_NEEDLE,
    SURVIVAL_HUNGER_FOOD_CLIENT_PRE_NEEDLE, SURVIVAL_HUNGER_FOOD_CLIENT_USE_NEEDLE,
    SURVIVAL_HUNGER_FOOD_SERVER_CONSUME_FINISH_NEEDLE,
    SURVIVAL_HUNGER_FOOD_SERVER_CONSUME_START_NEEDLE, SURVIVAL_HUNGER_FOOD_SERVER_INVENTORY_NEEDLE,
    SURVIVAL_HUNGER_FOOD_SERVER_PRE_NEEDLE, SURVIVAL_HUNGER_FOOD_SERVER_STATE_NEEDLE,
    SURVIVAL_MOB_DROP_CLIENT_ATTACK_NEEDLE, SURVIVAL_MOB_DROP_CLIENT_DEATH_NEEDLE,
    SURVIVAL_MOB_DROP_CLIENT_DROP_NEEDLE, SURVIVAL_MOB_DROP_CLIENT_INVENTORY_NEEDLE,
    SURVIVAL_MOB_DROP_CLIENT_MOB_NEEDLE, SURVIVAL_MOB_DROP_CLIENT_PICKUP_NEEDLE,
    SURVIVAL_MOB_DROP_SERVER_ATTACK_NEEDLE, SURVIVAL_MOB_DROP_SERVER_DEATH_NEEDLE,
    SURVIVAL_MOB_DROP_SERVER_DROP_NEEDLE, SURVIVAL_MOB_DROP_SERVER_INVENTORY_NEEDLE,
    SURVIVAL_MOB_DROP_SERVER_PICKUP_NEEDLE, SURVIVAL_MOB_DROP_SERVER_SPAWN_NEEDLE,
    SURVIVAL_MOB_DROP_SERVER_STATE_NEEDLE, SURVIVAL_REDSTONE_TOGGLE_CLIENT_INPUT_OFF_NEEDLE,
    SURVIVAL_REDSTONE_TOGGLE_CLIENT_INPUT_ON_NEEDLE,
    SURVIVAL_REDSTONE_TOGGLE_CLIENT_OUTPUT_OFF_NEEDLE,
    SURVIVAL_REDSTONE_TOGGLE_CLIENT_OUTPUT_ON_NEEDLE, SURVIVAL_REDSTONE_TOGGLE_SERVER_INPUT_NEEDLE,
    SURVIVAL_REDSTONE_TOGGLE_SERVER_OFF_NEEDLE, SURVIVAL_REDSTONE_TOGGLE_SERVER_ON_NEEDLE,
    SURVIVAL_REDSTONE_TOGGLE_SERVER_STATE_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_CLIENT_MUTATION_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_CLIENT_POST_RESTART_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_CLIENT_PRE_RESTART_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_CLIENT_RECONNECT_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_SERVER_CLEAN_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_SERVER_MUTATION_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_SERVER_POST_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_SERVER_RESTART_NEEDLE,
    SURVIVAL_WORLD_PERSISTENCE_SERVER_STATE_NEEDLE, VANILLA_COMBAT_ARMOR_REFERENCE_HEALTH_NEEDLE,
    VANILLA_COMBAT_REFERENCE_CLIENT_COUNT_NEEDLE, VANILLA_COMBAT_REFERENCE_DAMAGE_NEEDLE,
    VANILLA_COMBAT_REFERENCE_KNOCKBACK_NEEDLE,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Scenario {
    Smoke,
    CompatBotProbe,
    FlagScoreRepeat,
    BlueFlagScore,
    InventoryInteraction,
    InventoryStackSplitMerge,
    InventoryDragTransactions,
    SurvivalBreakPlacePickup,
    SurvivalChestPersistence,
    SurvivalCraftingTable,
    SurvivalFurnacePersistence,
    SurvivalHungerFood,
    SurvivalMobDrop,
    SurvivalRedstoneToggle,
    SurvivalWorldPersistenceRestart,
    SurvivalCrashRecoveryParity,
    SurvivalBlockEntityPersistenceParity,
    SurvivalBiomeDimensionState,
    McpControlledSmoke,
    CombatDamage,
    CombatKnockback,
    VanillaCombatReferenceParity,
    VanillaCombatArmorReferenceParity,
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
    CtfInvalidPickupOwnership,
    CtfInvalidReturnDrop,
    CtfScoreLimitWinCondition,
    CtfSimultaneousPickupCaptureRace,
    CtfSpawnTeamBalanceReset,
}

pub(crate) type ScenarioMilestone = (&'static str, &'static str);

pub(crate) const COMBAT_CLIENT_COUNT_NEEDLE: &str = "mc_compat_combat_client_count=2";
pub(crate) const FLAG_CARRIER_DEATH_CLIENT_COUNT_NEEDLE: &str =
    "mc_compat_flag_carrier_death_client_count=2";
pub(crate) const MULTI_CLIENT_LOAD_COUNT_NEEDLE: &str = "mc_compat_multi_client_count=2";
pub(crate) const EQUIPMENT_UPDATE_CLIENT_COUNT_NEEDLE: &str =
    "mc_compat_equipment_update_client_count=2";
pub(crate) const PROJECTILE_HIT_CLIENT_COUNT_NEEDLE: &str =
    "mc_compat_projectile_hit_client_count=2";
pub(crate) const PROJECTILE_DAMAGE_CLIENT_COUNT_NEEDLE: &str =
    "mc_compat_projectile_damage_client_count=2";
pub(crate) const RECONNECT_SESSION_COUNT_NEEDLE: &str = "mc_compat_reconnect_session=2";
pub(crate) const FIRST_CLIENT_INDEX: usize = 0;
pub(crate) const SECOND_CLIENT_INDEX: usize = 1;
pub(crate) const SESSION_INDEX_ENV_OFFSET: usize = 1;
pub(crate) const MULTI_CLIENT_READY_COUNT: usize = 2;
pub(crate) const PROBE_ENABLED_VALUE: &str = "1";
pub(crate) const PROBE_REPEAT_SINGLE: &str = "1";
pub(crate) const PROBE_REPEAT_DOUBLE: &str = "2";
pub(crate) const TEAM_RED_VALUE: &str = "red";
pub(crate) const TEAM_BLUE_VALUE: &str = "blue";
pub(crate) const COMBAT_ATTACKER_ROLE: &str = "attacker";
pub(crate) const COMBAT_VICTIM_ROLE: &str = "victim";
pub(crate) const COMBAT_TARGET_USERNAME: &str = "compatbotb";
pub(crate) const FLAG_CARRIER_DEATH_PICKUP_FIRST_TICK: u32 = 760;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ScenarioSpec {
    pub(crate) scenario: Scenario,
    pub(crate) canonical_name: &'static str,
    pub(crate) aliases: &'static [&'static str],
    pub(crate) client_milestones: &'static [ScenarioMilestone],
    pub(crate) server_milestones: &'static [ScenarioMilestone],
    pub(crate) forbidden_patterns: &'static [ScenarioMilestone],
    pub(crate) behavior: ScenarioBehaviorKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ScenarioLiveCapability {
    pub(crate) scenario: &'static str,
    pub(crate) targeted_row: &'static str,
    pub(crate) packet_rows: &'static [&'static str],
    pub(crate) capability_kind: &'static str,
    pub(crate) backend_path: &'static str,
    pub(crate) client_path: &'static str,
    pub(crate) evidence_mode: &'static str,
    pub(crate) required_signals: &'static [&'static str],
    pub(crate) required_nonclaims: &'static [&'static str],
    pub(crate) blocker_reason: Option<&'static str>,
}

pub(crate) const LIVE_CAPABILITY_KIND_PROBE: &str = "targeted-packet-live-probe";
pub(crate) const LIVE_CAPABILITY_KIND_BLOCKED: &str = "targeted-packet-live-blocker";
pub(crate) const LIVE_EVIDENCE_MODE_OWNED_LOCAL: &str = "owned-local-live";
pub(crate) const LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER: &str = "fixture-bounded-blocker";
pub(crate) const TARGETED_PACKET_LIVE_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
];
const LIVE_CAPABILITY_KINDS: &[&str] = &[LIVE_CAPABILITY_KIND_PROBE, LIVE_CAPABILITY_KIND_BLOCKED];
const LIVE_EVIDENCE_MODES: &[&str] = &[
    LIVE_EVIDENCE_MODE_OWNED_LOCAL,
    LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
];
const TARGETED_PACKET_ROW_IDS: &[&str] = &[
    "block-entity-update-breadth",
    "chat-command-containment",
    "chunk-biome-data-packet",
    "creative-inventory-action",
    "entity-status-effect-packets",
    "recipe-book-client-settings",
    "resource-pack-status",
    "sign-editor-open-update",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ProbeTeam {
    Red,
    Blue,
}

impl ProbeTeam {
    pub(crate) fn env_value(self) -> &'static str {
        match self {
            Self::Red => TEAM_RED_VALUE,
            Self::Blue => TEAM_BLUE_VALUE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScenarioRunStrategy {
    SingleClient,
    ReconnectSequence,
    MultiClient,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct NegativeLiveRailBehavior {
    pub(crate) invalid_action: &'static str,
    pub(crate) postcondition: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ScenarioBehaviorKind {
    Default,
    CompatBotProbe,
    FlagScore {
        team: ProbeTeam,
        reconnect: bool,
    },
    ReconnectFlagState {
        negative_probe: Option<&'static str>,
    },
    InventoryInteraction,
    InventoryStackSplitMerge,
    InventoryDragTransactions,
    NegativeInventory {
        probe: &'static str,
        invalid_action: &'static str,
        postcondition: &'static str,
    },
    NegativeCustomPayload,
    SurvivalBreakPlacePickup,
    SurvivalChestPersistence,
    SurvivalCraftingTable,
    SurvivalFurnacePersistence,
    SurvivalHungerFood,
    SurvivalMobDrop,
    SurvivalRedstoneToggle,
    WorldPersistenceRestart {
        crash_recovery: bool,
        block_entity: bool,
    },
    SurvivalBiomeDimensionState,
    McpControlledSmoke,
    Combat {
        reference_probe: bool,
        armor_reference: bool,
        armor_mitigation: bool,
        flag_carrier_death: bool,
        count_needle: Option<&'static str>,
    },
    EquipmentUpdate,
    Projectile {
        damage: bool,
    },
    MultiClientLoadScore,
    NegativeCtfWrongScore,
    CtfInvalidPickupOwnership,
    CtfInvalidReturnDrop,
    CtfScoreLimitWinCondition,
    CtfSimultaneousPickupCaptureRace,
    CtfSpawnTeamBalanceReset,
}

impl ScenarioBehaviorKind {
    pub(crate) fn run_strategy(&self) -> ScenarioRunStrategy {
        match self {
            Self::ReconnectFlagState { .. }
            | Self::SurvivalChestPersistence
            | Self::SurvivalFurnacePersistence
            | Self::WorldPersistenceRestart { .. } => ScenarioRunStrategy::ReconnectSequence,
            Self::Combat { .. }
            | Self::EquipmentUpdate
            | Self::Projectile { .. }
            | Self::MultiClientLoadScore
            | Self::CtfSimultaneousPickupCaptureRace
            | Self::CtfSpawnTeamBalanceReset => ScenarioRunStrategy::MultiClient,
            _ => ScenarioRunStrategy::SingleClient,
        }
    }

    pub(crate) fn negative_live_rail(&self) -> Option<NegativeLiveRailBehavior> {
        match self {
            Self::NegativeInventory {
                invalid_action,
                postcondition,
                ..
            } => Some(NegativeLiveRailBehavior {
                invalid_action,
                postcondition,
            }),
            Self::NegativeCustomPayload => Some(NegativeLiveRailBehavior {
                invalid_action: "malformed_custom_payload",
                postcondition: "negative_custom_payload_contained",
            }),
            Self::ReconnectFlagState {
                negative_probe: Some(_),
            } => Some(NegativeLiveRailBehavior {
                invalid_action: "duplicate_reconnect_flag_transition",
                postcondition: "negative_reconnect_race_contained",
            }),
            Self::NegativeCtfWrongScore => Some(NegativeLiveRailBehavior {
                invalid_action: "wrong_team_or_wrong_portal_score_attempt",
                postcondition: "negative_wrong_score_contained",
            }),
            Self::CtfInvalidPickupOwnership => Some(NegativeLiveRailBehavior {
                invalid_action: "own_flag_pickup_without_ownership_transfer",
                postcondition: "ctf_invalid_pickup_contained",
            }),
            Self::CtfInvalidReturnDrop => Some(NegativeLiveRailBehavior {
                invalid_action: "own_base_return_without_carrier",
                postcondition: "ctf_invalid_return_drop_contained",
            }),
            _ => None,
        }
    }

    pub(crate) fn uses_dynamic_projectile_health(&self) -> bool {
        matches!(self, Self::Projectile { damage: true })
    }

    pub(crate) fn is_mcp_controlled_smoke(&self) -> bool {
        matches!(self, Self::McpControlledSmoke)
    }
}

pub(crate) const ALL_SCENARIOS: &[Scenario] = &[
    Scenario::Smoke,
    Scenario::CompatBotProbe,
    Scenario::FlagScoreRepeat,
    Scenario::BlueFlagScore,
    Scenario::InventoryInteraction,
    Scenario::InventoryStackSplitMerge,
    Scenario::InventoryDragTransactions,
    Scenario::SurvivalBreakPlacePickup,
    Scenario::SurvivalChestPersistence,
    Scenario::SurvivalCraftingTable,
    Scenario::SurvivalFurnacePersistence,
    Scenario::SurvivalHungerFood,
    Scenario::SurvivalMobDrop,
    Scenario::SurvivalRedstoneToggle,
    Scenario::SurvivalWorldPersistenceRestart,
    Scenario::SurvivalCrashRecoveryParity,
    Scenario::SurvivalBlockEntityPersistenceParity,
    Scenario::SurvivalBiomeDimensionState,
    Scenario::McpControlledSmoke,
    Scenario::CombatDamage,
    Scenario::CombatKnockback,
    Scenario::VanillaCombatReferenceParity,
    Scenario::VanillaCombatArmorReferenceParity,
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
    Scenario::CtfInvalidPickupOwnership,
    Scenario::CtfInvalidReturnDrop,
    Scenario::CtfScoreLimitWinCondition,
    Scenario::CtfSimultaneousPickupCaptureRace,
    Scenario::CtfSpawnTeamBalanceReset,
];

pub(crate) const SCENARIO_SPECS: &[ScenarioSpec] = &[
    ScenarioSpec {
        scenario: Scenario::Smoke,
        canonical_name: "smoke",
        aliases: &["smoke"],
        client_milestones: &[("protocol_detected", "Detected server protocol version")],
        server_milestones: &[],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Default,
    },
    ScenarioSpec {
        scenario: Scenario::CompatBotProbe,
        canonical_name: "valence-compat-bot-probe",
        aliases: &["valence-compat-bot-probe", "compat-bot-probe"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
        ],
        server_milestones: &[],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::CompatBotProbe,
    },
    ScenarioSpec {
        scenario: Scenario::FlagScoreRepeat,
        canonical_name: "flag-score-repeat",
        aliases: &["flag-score-repeat"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_1", "RED: 1"),
            ("score_red_2", "RED: 2"),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            ("server_flag_or_score", "flag"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::FlagScore { team: ProbeTeam::Red, reconnect: false },
    },
    ScenarioSpec {
        scenario: Scenario::BlueFlagScore,
        canonical_name: "blue-flag-score",
        aliases: &["blue-flag-score"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_blue", "You are on team BLUE!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_blue_1", "BLUE: 1"),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            ("server_flag_or_score", "flag"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::FlagScore { team: ProbeTeam::Blue, reconnect: false },
    },
    ScenarioSpec {
        scenario: Scenario::InventoryInteraction,
        canonical_name: "inventory-interaction",
        aliases: &["inventory-interaction"],
        client_milestones: &[
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
        server_milestones: &[
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
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::InventoryInteraction,
    },
    ScenarioSpec {
        scenario: Scenario::InventoryStackSplitMerge,
        canonical_name: "inventory-stack-split-merge",
        aliases: &["inventory-stack-split-merge"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            (
                "inventory_stack_initial_slot",
                INVENTORY_STACK_CLIENT_INITIAL_NEEDLE,
            ),
            (
                "inventory_stack_split_pickup_sent",
                INVENTORY_STACK_CLIENT_SPLIT_PICKUP_NEEDLE,
            ),
            (
                "inventory_stack_split_source_seen",
                INVENTORY_STACK_CLIENT_SPLIT_SOURCE_NEEDLE,
            ),
            (
                "inventory_stack_split_place_sent",
                INVENTORY_STACK_CLIENT_SPLIT_PLACE_NEEDLE,
            ),
            (
                "inventory_stack_destination_seen",
                INVENTORY_STACK_CLIENT_DESTINATION_NEEDLE,
            ),
            (
                "inventory_stack_merge_pickup_sent",
                INVENTORY_STACK_CLIENT_MERGE_PICKUP_NEEDLE,
            ),
            (
                "inventory_stack_merge_destination_empty_seen",
                INVENTORY_STACK_CLIENT_MERGE_EMPTY_NEEDLE,
            ),
            (
                "inventory_stack_merge_place_sent",
                INVENTORY_STACK_CLIENT_MERGE_PLACE_NEEDLE,
            ),
            (
                "inventory_stack_final_source_seen",
                INVENTORY_STACK_CLIENT_FINAL_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_inventory_stack_split_pickup",
                INVENTORY_STACK_SERVER_SPLIT_PICKUP_NEEDLE,
            ),
            (
                "server_inventory_stack_split",
                INVENTORY_STACK_SERVER_SPLIT_NEEDLE,
            ),
            (
                "server_inventory_stack_merge_pickup",
                INVENTORY_STACK_SERVER_MERGE_PICKUP_NEEDLE,
            ),
            (
                "server_inventory_stack_merge",
                INVENTORY_STACK_SERVER_MERGE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::InventoryStackSplitMerge,
    },
    ScenarioSpec {
        scenario: Scenario::InventoryDragTransactions,
        canonical_name: "inventory-drag-transactions",
        aliases: &["inventory-drag-transactions"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            (
                "inventory_drag_initial_slot",
                INVENTORY_DRAG_CLIENT_INITIAL_NEEDLE,
            ),
            (
                "inventory_drag_pickup_sent",
                INVENTORY_DRAG_CLIENT_PICKUP_NEEDLE,
            ),
            (
                "inventory_drag_source_empty_seen",
                INVENTORY_DRAG_CLIENT_SOURCE_EMPTY_NEEDLE,
            ),
            (
                "inventory_drag_start_sent",
                INVENTORY_DRAG_CLIENT_START_NEEDLE,
            ),
            (
                "inventory_drag_target_a_sent",
                INVENTORY_DRAG_CLIENT_TARGET_A_NEEDLE,
            ),
            (
                "inventory_drag_target_b_sent",
                INVENTORY_DRAG_CLIENT_TARGET_B_NEEDLE,
            ),
            ("inventory_drag_end_sent", INVENTORY_DRAG_CLIENT_END_NEEDLE),
            (
                "inventory_drag_final_distribution_seen",
                INVENTORY_DRAG_CLIENT_FINAL_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            ("server_inventory_drag_pickup", INVENTORY_DRAG_SERVER_PICKUP_NEEDLE),
            ("server_inventory_drag_start", INVENTORY_DRAG_SERVER_START_NEEDLE),
            (
                "server_inventory_drag_target_a",
                INVENTORY_DRAG_SERVER_TARGET_A_NEEDLE,
            ),
            (
                "server_inventory_drag_target_b",
                INVENTORY_DRAG_SERVER_TARGET_B_NEEDLE,
            ),
            ("server_inventory_drag_end", INVENTORY_DRAG_SERVER_END_NEEDLE),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::InventoryDragTransactions,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalBreakPlacePickup,
        canonical_name: "survival-break-place-pickup",
        aliases: &["survival-break-place-pickup"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("survival_break_sent", "survival_probe_break_block_sent"),
            ("survival_break_update", "survival_probe_block_update"),
            ("survival_pickup_seen", "survival_probe_pickup_seen"),
            ("survival_place_sent", "survival_probe_place_block_sent"),
            ("survival_place_update", "survival_probe_place_update"),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            ("server_survival_join", "survival_join"),
            ("server_survival_break", "survival_block_break"),
            ("server_survival_pickup", "survival_pickup_item"),
            ("server_survival_place", "survival_block_place"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalBreakPlacePickup,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalChestPersistence,
        canonical_name: "survival-chest-persistence",
        aliases: &["survival-chest-persistence"],
        client_milestones: &[
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
        server_milestones: &[
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
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalChestPersistence,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalCraftingTable,
        canonical_name: "survival-crafting-table",
        aliases: &["survival-crafting-table"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_crafting_table_open_seen",
                SURVIVAL_CRAFTING_CLIENT_OPEN_NEEDLE,
            ),
            (
                "survival_crafting_input_a_sent",
                SURVIVAL_CRAFTING_CLIENT_INPUT_A_NEEDLE,
            ),
            (
                "survival_crafting_input_b_sent",
                SURVIVAL_CRAFTING_CLIENT_INPUT_B_NEEDLE,
            ),
            (
                "survival_crafting_result_seen",
                SURVIVAL_CRAFTING_CLIENT_RESULT_NEEDLE,
            ),
            (
                "survival_crafting_result_collected",
                SURVIVAL_CRAFTING_CLIENT_COLLECT_NEEDLE,
            ),
            (
                "survival_crafting_inventory_updated",
                SURVIVAL_CRAFTING_CLIENT_INVENTORY_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_crafting_table_open",
                SURVIVAL_CRAFTING_SERVER_OPEN_NEEDLE,
            ),
            (
                "server_survival_crafting_input_a",
                SURVIVAL_CRAFTING_SERVER_INPUT_A_NEEDLE,
            ),
            (
                "server_survival_crafting_input_b",
                SURVIVAL_CRAFTING_SERVER_INPUT_B_NEEDLE,
            ),
            (
                "server_survival_crafting_result",
                SURVIVAL_CRAFTING_SERVER_RESULT_NEEDLE,
            ),
            (
                "server_survival_crafting_collect",
                SURVIVAL_CRAFTING_SERVER_COLLECT_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalCraftingTable,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalFurnacePersistence,
        canonical_name: "survival-furnace-persistence",
        aliases: &["survival-furnace-persistence"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_furnace_open_seen",
                SURVIVAL_FURNACE_CLIENT_OPEN_NEEDLE,
            ),
            (
                "survival_furnace_input_sent",
                SURVIVAL_FURNACE_CLIENT_INPUT_NEEDLE,
            ),
            (
                "survival_furnace_fuel_sent",
                SURVIVAL_FURNACE_CLIENT_FUEL_NEEDLE,
            ),
            (
                "survival_furnace_burn_progress_seen",
                SURVIVAL_FURNACE_CLIENT_BURN_NEEDLE,
            ),
            (
                "survival_furnace_output_seen",
                SURVIVAL_FURNACE_CLIENT_OUTPUT_NEEDLE,
            ),
            (
                "survival_furnace_output_collected",
                SURVIVAL_FURNACE_CLIENT_COLLECT_NEEDLE,
            ),
            (
                "survival_furnace_inventory_updated",
                SURVIVAL_FURNACE_CLIENT_INVENTORY_NEEDLE,
            ),
            (
                "survival_furnace_reconnect_sent",
                SURVIVAL_FURNACE_CLIENT_RECONNECT_NEEDLE,
            ),
            (
                "survival_furnace_reopen_seen",
                SURVIVAL_FURNACE_CLIENT_REOPEN_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_furnace_open",
                SURVIVAL_FURNACE_SERVER_OPEN_NEEDLE,
            ),
            (
                "server_survival_furnace_input",
                SURVIVAL_FURNACE_SERVER_INPUT_NEEDLE,
            ),
            (
                "server_survival_furnace_fuel",
                SURVIVAL_FURNACE_SERVER_FUEL_NEEDLE,
            ),
            (
                "server_survival_furnace_burn_progress",
                SURVIVAL_FURNACE_SERVER_BURN_NEEDLE,
            ),
            (
                "server_survival_furnace_output_available",
                SURVIVAL_FURNACE_SERVER_OUTPUT_NEEDLE,
            ),
            (
                "server_survival_furnace_output_collect",
                SURVIVAL_FURNACE_SERVER_COLLECT_NEEDLE,
            ),
            (
                "server_survival_furnace_reconnect_reopen",
                SURVIVAL_FURNACE_SERVER_REOPEN_NEEDLE,
            ),
            (
                "server_survival_furnace_state",
                SURVIVAL_FURNACE_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalFurnacePersistence,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalHungerFood,
        canonical_name: "survival-hunger-food",
        aliases: &["survival-hunger-food"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_hunger_food_item_seen",
                SURVIVAL_HUNGER_FOOD_CLIENT_ITEM_NEEDLE,
            ),
            (
                "survival_hunger_food_pre_seen",
                SURVIVAL_HUNGER_FOOD_CLIENT_PRE_NEEDLE,
            ),
            (
                "survival_hunger_food_use_sent",
                SURVIVAL_HUNGER_FOOD_CLIENT_USE_NEEDLE,
            ),
            (
                "survival_hunger_food_post_seen",
                SURVIVAL_HUNGER_FOOD_CLIENT_POST_NEEDLE,
            ),
            (
                "survival_hunger_food_inventory_updated",
                SURVIVAL_HUNGER_FOOD_CLIENT_INVENTORY_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_hunger_food_pre",
                SURVIVAL_HUNGER_FOOD_SERVER_PRE_NEEDLE,
            ),
            (
                "server_survival_hunger_food_consume_start",
                SURVIVAL_HUNGER_FOOD_SERVER_CONSUME_START_NEEDLE,
            ),
            (
                "server_survival_hunger_food_consume_finish",
                SURVIVAL_HUNGER_FOOD_SERVER_CONSUME_FINISH_NEEDLE,
            ),
            (
                "server_survival_hunger_food_inventory",
                SURVIVAL_HUNGER_FOOD_SERVER_INVENTORY_NEEDLE,
            ),
            (
                "server_survival_hunger_food_state",
                SURVIVAL_HUNGER_FOOD_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalHungerFood,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalMobDrop,
        canonical_name: "survival-mob-drop",
        aliases: &["survival-mob-drop"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_mob_drop_mob_seen",
                SURVIVAL_MOB_DROP_CLIENT_MOB_NEEDLE,
            ),
            (
                "survival_mob_drop_attack_sent",
                SURVIVAL_MOB_DROP_CLIENT_ATTACK_NEEDLE,
            ),
            (
                "survival_mob_drop_death_seen",
                SURVIVAL_MOB_DROP_CLIENT_DEATH_NEEDLE,
            ),
            (
                "survival_mob_drop_drop_seen",
                SURVIVAL_MOB_DROP_CLIENT_DROP_NEEDLE,
            ),
            (
                "survival_mob_drop_pickup_seen",
                SURVIVAL_MOB_DROP_CLIENT_PICKUP_NEEDLE,
            ),
            (
                "survival_mob_drop_inventory_updated",
                SURVIVAL_MOB_DROP_CLIENT_INVENTORY_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_mob_drop_spawn",
                SURVIVAL_MOB_DROP_SERVER_SPAWN_NEEDLE,
            ),
            (
                "server_survival_mob_drop_attack",
                SURVIVAL_MOB_DROP_SERVER_ATTACK_NEEDLE,
            ),
            (
                "server_survival_mob_drop_death",
                SURVIVAL_MOB_DROP_SERVER_DEATH_NEEDLE,
            ),
            (
                "server_survival_mob_drop_drop_spawn",
                SURVIVAL_MOB_DROP_SERVER_DROP_NEEDLE,
            ),
            (
                "server_survival_mob_drop_pickup",
                SURVIVAL_MOB_DROP_SERVER_PICKUP_NEEDLE,
            ),
            (
                "server_survival_mob_drop_inventory",
                SURVIVAL_MOB_DROP_SERVER_INVENTORY_NEEDLE,
            ),
            (
                "server_survival_mob_drop_state",
                SURVIVAL_MOB_DROP_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalMobDrop,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalRedstoneToggle,
        canonical_name: "survival-redstone-toggle",
        aliases: &["survival-redstone-toggle"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_redstone_toggle_input_sent",
                SURVIVAL_REDSTONE_TOGGLE_CLIENT_INPUT_ON_NEEDLE,
            ),
            (
                "survival_redstone_toggle_output_update",
                SURVIVAL_REDSTONE_TOGGLE_CLIENT_OUTPUT_ON_NEEDLE,
            ),
            (
                "survival_redstone_toggle_return_input_sent",
                SURVIVAL_REDSTONE_TOGGLE_CLIENT_INPUT_OFF_NEEDLE,
            ),
            (
                "survival_redstone_toggle_return_update",
                SURVIVAL_REDSTONE_TOGGLE_CLIENT_OUTPUT_OFF_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_redstone_toggle_input",
                SURVIVAL_REDSTONE_TOGGLE_SERVER_INPUT_NEEDLE,
            ),
            (
                "server_survival_redstone_toggle_powered_on",
                SURVIVAL_REDSTONE_TOGGLE_SERVER_ON_NEEDLE,
            ),
            (
                "server_survival_redstone_toggle_powered_off",
                SURVIVAL_REDSTONE_TOGGLE_SERVER_OFF_NEEDLE,
            ),
            (
                "server_survival_redstone_toggle_state",
                SURVIVAL_REDSTONE_TOGGLE_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalRedstoneToggle,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalWorldPersistenceRestart,
        canonical_name: "survival-world-persistence-restart",
        aliases: &["survival-world-persistence-restart"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_world_persistence_mutation_sent",
                SURVIVAL_WORLD_PERSISTENCE_CLIENT_MUTATION_NEEDLE,
            ),
            (
                "survival_world_persistence_pre_restart_update",
                SURVIVAL_WORLD_PERSISTENCE_CLIENT_PRE_RESTART_NEEDLE,
            ),
            (
                "survival_world_persistence_reconnect_sent",
                SURVIVAL_WORLD_PERSISTENCE_CLIENT_RECONNECT_NEEDLE,
            ),
            (
                "survival_world_persistence_post_restart_update",
                SURVIVAL_WORLD_PERSISTENCE_CLIENT_POST_RESTART_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_world_persistence_mutation",
                SURVIVAL_WORLD_PERSISTENCE_SERVER_MUTATION_NEEDLE,
            ),
            (
                "server_survival_world_persistence_clean_shutdown",
                SURVIVAL_WORLD_PERSISTENCE_SERVER_CLEAN_NEEDLE,
            ),
            (
                "server_survival_world_persistence_backend_restart",
                SURVIVAL_WORLD_PERSISTENCE_SERVER_RESTART_NEEDLE,
            ),
            (
                "server_survival_world_persistence_post_restart",
                SURVIVAL_WORLD_PERSISTENCE_SERVER_POST_NEEDLE,
            ),
            (
                "server_survival_world_persistence_state",
                SURVIVAL_WORLD_PERSISTENCE_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::WorldPersistenceRestart { crash_recovery: false, block_entity: false },
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalCrashRecoveryParity,
        canonical_name: "survival-crash-recovery-parity",
        aliases: &["survival-crash-recovery-parity"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_crash_recovery_mutation_sent",
                SURVIVAL_CRASH_RECOVERY_CLIENT_MUTATION_NEEDLE,
            ),
            (
                "survival_crash_recovery_pre_crash_update",
                SURVIVAL_CRASH_RECOVERY_CLIENT_PRE_CRASH_NEEDLE,
            ),
            (
                "survival_crash_recovery_reconnect_sent",
                SURVIVAL_CRASH_RECOVERY_CLIENT_RECONNECT_NEEDLE,
            ),
            (
                "survival_crash_recovery_post_crash_update",
                SURVIVAL_CRASH_RECOVERY_CLIENT_POST_CRASH_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_crash_recovery_mutation",
                SURVIVAL_CRASH_RECOVERY_SERVER_MUTATION_NEEDLE,
            ),
            (
                "server_survival_crash_recovery_forced_stop",
                SURVIVAL_CRASH_RECOVERY_SERVER_FORCED_STOP_NEEDLE,
            ),
            (
                "server_survival_crash_recovery_backend_restart",
                SURVIVAL_CRASH_RECOVERY_SERVER_RESTART_NEEDLE,
            ),
            (
                "server_survival_crash_recovery_post_crash",
                SURVIVAL_CRASH_RECOVERY_SERVER_POST_NEEDLE,
            ),
            (
                "server_survival_crash_recovery_state",
                SURVIVAL_CRASH_RECOVERY_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::WorldPersistenceRestart { crash_recovery: true, block_entity: false },
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalBlockEntityPersistenceParity,
        canonical_name: "survival-block-entity-persistence-parity",
        aliases: &["survival-block-entity-persistence-parity"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_block_entity_pre_restart_update",
                SURVIVAL_BLOCK_ENTITY_CLIENT_PRE_RESTART_NEEDLE,
            ),
            (
                "survival_block_entity_reconnect_sent",
                SURVIVAL_BLOCK_ENTITY_CLIENT_RECONNECT_NEEDLE,
            ),
            (
                "survival_block_entity_post_restart_update",
                SURVIVAL_BLOCK_ENTITY_CLIENT_POST_RESTART_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_block_entity_mutation",
                SURVIVAL_BLOCK_ENTITY_SERVER_MUTATION_NEEDLE,
            ),
            (
                "server_survival_block_entity_clean_shutdown",
                SURVIVAL_BLOCK_ENTITY_SERVER_CLEAN_NEEDLE,
            ),
            (
                "server_survival_block_entity_backend_restart",
                SURVIVAL_BLOCK_ENTITY_SERVER_RESTART_NEEDLE,
            ),
            (
                "server_survival_block_entity_post_restart",
                SURVIVAL_BLOCK_ENTITY_SERVER_POST_NEEDLE,
            ),
            (
                "server_survival_block_entity_state",
                SURVIVAL_BLOCK_ENTITY_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::WorldPersistenceRestart { crash_recovery: false, block_entity: true },
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalBiomeDimensionState,
        canonical_name: "survival-biome-dimension-state",
        aliases: &["survival-biome-dimension-state"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_biome_dimension_state",
                SURVIVAL_BIOME_DIMENSION_CLIENT_STATE_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_biome_dimension_state",
                SURVIVAL_BIOME_DIMENSION_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalBiomeDimensionState,
    },
    ScenarioSpec {
        scenario: Scenario::McpControlledSmoke,
        canonical_name: MCP_CONTROLLED_SMOKE_SCENARIO,
        aliases: &[MCP_CONTROLLED_SMOKE_SCENARIO],
        client_milestones: &[
            ("mcp_initialize", "mcp_initialize"),
            ("mcp_tools_list", "mcp_tools_list"),
            ("mcp_status_call", "mcp_status_call"),
            ("mcp_command_outcomes", "mcp_command_outcomes"),
        ],
        server_milestones: &[],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::McpControlledSmoke,
    },
    ScenarioSpec {
        scenario: Scenario::CombatDamage,
        canonical_name: "combat-damage",
        aliases: &["combat-damage"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_combat_damage", "combat_damage"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Combat { reference_probe: false, armor_reference: false, armor_mitigation: false, flag_carrier_death: false, count_needle: Some(COMBAT_CLIENT_COUNT_NEEDLE) },
    },
    ScenarioSpec {
        scenario: Scenario::CombatKnockback,
        canonical_name: "combat-knockback",
        aliases: &["combat-knockback"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_combat_damage", "combat_damage"),
            ("server_combat_knockback", "combat_knockback"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Combat { reference_probe: false, armor_reference: false, armor_mitigation: false, flag_carrier_death: false, count_needle: Some(COMBAT_CLIENT_COUNT_NEEDLE) },
    },
    ScenarioSpec {
        scenario: Scenario::VanillaCombatReferenceParity,
        canonical_name: "vanilla-combat-reference-parity",
        aliases: &["vanilla-combat-reference-parity"],
        client_milestones: &[
            (
                "multi_client_count",
                VANILLA_COMBAT_REFERENCE_CLIENT_COUNT_NEEDLE,
            ),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("remote_player_spawn", "remote_player_spawn"),
            ("combat_attack_sent", "combat_probe_attack_sent"),
            ("combat_health_update", "update_health health=14.0"),
            ("combat_velocity_update", "combat_probe_velocity_observed"),
        ],
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            (
                "server_vanilla_combat_reference_damage",
                VANILLA_COMBAT_REFERENCE_DAMAGE_NEEDLE,
            ),
            (
                "server_vanilla_combat_reference_knockback",
                VANILLA_COMBAT_REFERENCE_KNOCKBACK_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Combat { reference_probe: true, armor_reference: false, armor_mitigation: false, flag_carrier_death: false, count_needle: None },
    },
    ScenarioSpec {
        scenario: Scenario::VanillaCombatArmorReferenceParity,
        canonical_name: "vanilla-combat-armor-reference-parity",
        aliases: &["vanilla-combat-armor-reference-parity"],
        client_milestones: &[
            (
                "multi_client_count",
                VANILLA_COMBAT_REFERENCE_CLIENT_COUNT_NEEDLE,
            ),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("remote_player_spawn", "remote_player_spawn"),
            ("combat_attack_sent", "combat_probe_attack_sent"),
            (
                "combat_health_update",
                VANILLA_COMBAT_ARMOR_REFERENCE_HEALTH_NEEDLE,
            ),
            ("combat_velocity_update", "combat_probe_velocity_observed"),
        ],
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            (
                "server_vanilla_combat_reference_damage",
                VANILLA_COMBAT_REFERENCE_DAMAGE_NEEDLE,
            ),
            (
                "server_vanilla_combat_reference_knockback",
                VANILLA_COMBAT_REFERENCE_KNOCKBACK_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Combat { reference_probe: true, armor_reference: true, armor_mitigation: true, flag_carrier_death: false, count_needle: Some(COMBAT_CLIENT_COUNT_NEEDLE) },
    },
    ScenarioSpec {
        scenario: Scenario::ArmorEquipmentMitigation,
        canonical_name: "armor-equipment-mitigation",
        aliases: &["armor-equipment-mitigation"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_equipment_state", "armor_equipment_state"),
            ("server_combat_damage", "combat_damage"),
            ("server_armor_mitigation", "combat_armor_mitigation"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Combat { reference_probe: false, armor_reference: false, armor_mitigation: true, flag_carrier_death: false, count_needle: Some(COMBAT_CLIENT_COUNT_NEEDLE) },
    },
    ScenarioSpec {
        scenario: Scenario::ArmorLoadoutEnchantmentStatusMatrix,
        canonical_name: "armor-loadout-enchantment-status-matrix",
        aliases: &["armor-loadout-enchantment-status-matrix"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_equipment_state", "armor_equipment_state"),
            ("server_combat_damage", "combat_damage"),
            ("server_armor_mitigation", "combat_armor_mitigation"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Combat { reference_probe: false, armor_reference: false, armor_mitigation: true, flag_carrier_death: false, count_needle: Some(COMBAT_CLIENT_COUNT_NEEDLE) },
    },
    ScenarioSpec {
        scenario: Scenario::EquipmentUpdateObservation,
        canonical_name: "equipment-update-observation",
        aliases: &["equipment-update-observation"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_equipment_update_state", "equipment_update_state"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::EquipmentUpdate,
    },
    ScenarioSpec {
        scenario: Scenario::EquipmentSlotItemMatrixExpansion,
        canonical_name: "equipment-slot-item-matrix-expansion",
        aliases: &["equipment-slot-item-matrix-expansion"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_equipment_update_state", "equipment_update_state"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::EquipmentUpdate,
    },
    ScenarioSpec {
        scenario: Scenario::ProjectileHit,
        canonical_name: "projectile-hit",
        aliases: &["projectile-hit"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_projectile_loadout", "projectile_loadout"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Projectile { damage: false },
    },
    ScenarioSpec {
        scenario: Scenario::ProjectileDamageAttribution,
        canonical_name: "projectile-damage-attribution",
        aliases: &["projectile-damage-attribution"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_projectile_loadout", "projectile_loadout"),
            ("server_projectile_use", "projectile_use"),
            ("server_projectile_hit", "projectile_hit"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::Projectile { damage: true },
    },
    ScenarioSpec {
        scenario: Scenario::FlagCarrierDeathReturn,
        canonical_name: "flag-carrier-death-return",
        aliases: &["flag-carrier-death-return"],
        client_milestones: &[
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
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_flag_pickup", "flag_pickup"),
            ("server_flag_carrier_death", "flag_carrier_death"),
            ("server_flag_return", "flag_return"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("unexpected_flag_capture", "You captured the flag!"),
            ("unexpected_flag_capture_milestone", "flag_capture"),
            ("unexpected_red_score", "RED: 1"),
            ("unexpected_blue_score", "BLUE: 1"),
        ],
        behavior: ScenarioBehaviorKind::Combat { reference_probe: false, armor_reference: false, armor_mitigation: false, flag_carrier_death: true, count_needle: Some(FLAG_CARRIER_DEATH_CLIENT_COUNT_NEEDLE) },
    },
    ScenarioSpec {
        scenario: Scenario::ReconnectFlagState,
        canonical_name: "reconnect-flag-state",
        aliases: &["reconnect-flag-state"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("reconnect_session", "mc_compat_reconnect_session=2"),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            ("server_flag_pickup", "flag_pickup"),
            ("server_flag_disconnect_return", "flag_disconnect_return"),
            (
                "server_reconnect_state_coherent",
                "reconnect_state_coherent",
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("unexpected_flag_capture", "You captured the flag!"),
            ("unexpected_flag_capture_milestone", "flag_capture"),
            ("unexpected_red_score", "RED: 1"),
            ("unexpected_blue_score", "BLUE: 1"),
        ],
        behavior: ScenarioBehaviorKind::ReconnectFlagState { negative_probe: None },
    },
    ScenarioSpec {
        scenario: Scenario::ReconnectFlagScore,
        canonical_name: "reconnect-flag-score",
        aliases: &["reconnect-flag-score"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_1", "RED: 1"),
            ("reconnect_session", "mc_compat_reconnect_session=2"),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            ("server_flag_or_score", "flag"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::FlagScore { team: ProbeTeam::Red, reconnect: true },
    },
    ScenarioSpec {
        scenario: Scenario::MultiClientLoadScore,
        canonical_name: "multi-client-load-score",
        aliases: &["multi-client-load-score"],
        client_milestones: &[
            ("multi_client_count", "mc_compat_multi_client_count=2"),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_1", "RED: 1"),
        ],
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_flag_or_score", "flag"),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::MultiClientLoadScore,
    },
    ScenarioSpec {
        scenario: Scenario::NegativeInventoryStaleState,
        canonical_name: "negative-inventory-stale-state",
        aliases: &["negative-inventory-stale-state"],
        client_milestones: &[
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
        server_milestones: &[("server_username_seen", "compatbot")],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::NegativeInventory { probe: "inventory_stale_state", invalid_action: "stale_inventory_state_id", postcondition: "negative_inventory_stale_state_contained" },
    },
    ScenarioSpec {
        scenario: Scenario::NegativeInventoryInvalidClick,
        canonical_name: "negative-inventory-invalid-click",
        aliases: &["negative-inventory-invalid-click"],
        client_milestones: &[
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
        server_milestones: &[("server_username_seen", "compatbot")],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::NegativeInventory { probe: "inventory_invalid_click", invalid_action: "invalid_slot_or_window_click", postcondition: "negative_inventory_invalid_click_restored" },
    },
    ScenarioSpec {
        scenario: Scenario::NegativeCustomPayload,
        canonical_name: "negative-custom-payload",
        aliases: &["negative-custom-payload"],
        client_milestones: &[
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
        server_milestones: &[("server_username_seen", "compatbot")],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::NegativeCustomPayload,
    },
    ScenarioSpec {
        scenario: Scenario::NegativeReconnectRace,
        canonical_name: "negative-reconnect-race",
        aliases: &["negative-reconnect-race"],
        client_milestones: &[
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
        server_milestones: &[("server_username_seen", "compatbot")],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("unexpected_flag_capture", "You captured the flag!"),
            ("unexpected_flag_capture_milestone", "flag_capture"),
            ("unexpected_red_score", "RED: 1"),
            ("unexpected_blue_score", "BLUE: 1"),
        ],
        behavior: ScenarioBehaviorKind::ReconnectFlagState { negative_probe: Some("reconnect_race") },
    },
    ScenarioSpec {
        scenario: Scenario::NegativeCtfWrongScore,
        canonical_name: "negative-ctf-wrong-score",
        aliases: &["negative-ctf-wrong-score"],
        client_milestones: &[
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
        server_milestones: &[("server_username_seen", "compatbot")],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("unexpected_flag_capture", "You captured the flag!"),
            ("unexpected_flag_capture_milestone", "flag_capture"),
            ("unexpected_red_score", "RED: 1"),
            ("unexpected_blue_score", "BLUE: 1"),
        ],
        behavior: ScenarioBehaviorKind::NegativeCtfWrongScore,
    },
    ScenarioSpec {
        scenario: Scenario::CtfInvalidPickupOwnership,
        canonical_name: "ctf-invalid-pickup-ownership",
        aliases: &["ctf-invalid-pickup-ownership"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "ctf_invalid_pickup_attempted",
                "ctf_invalid_pickup_attempted",
            ),
            (
                "ctf_invalid_pickup_contained",
                "ctf_invalid_pickup_contained",
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_invalid_pickup_rejected",
                "invalid_flag_pickup_rejected username=compatbot player_team=Red flag_team=Red pre_owner=none post_owner=none red_score=0 blue_score=0 outcome=no_owner_transfer_no_score",
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("unexpected_flag_pickup_chat", "You have the flag!"),
            (
                "unexpected_flag_pickup_milestone",
                "flag_probe_have_flag_chat",
            ),
            (
                "unexpected_server_flag_pickup",
                "MC-COMPAT-MILESTONE flag_pickup username=",
            ),
            ("unexpected_flag_capture", "You captured the flag!"),
            ("unexpected_flag_capture_milestone", "flag_capture"),
            ("unexpected_red_score", "RED: 1"),
            ("unexpected_blue_score", "BLUE: 1"),
        ],
        behavior: ScenarioBehaviorKind::CtfInvalidPickupOwnership,
    },
    ScenarioSpec {
        scenario: Scenario::CtfInvalidReturnDrop,
        canonical_name: "ctf-invalid-return-drop",
        aliases: &["ctf-invalid-return-drop"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "ctf_invalid_return_drop_attempted",
                "ctf_invalid_return_drop_attempted",
            ),
            (
                "ctf_invalid_return_drop_contained",
                "ctf_invalid_return_drop_contained",
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_invalid_return_drop_rejected",
                "invalid_flag_return_drop_rejected username=compatbot actor_team=Red flag_team=Red pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score",
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("unexpected_flag_pickup_chat", "You have the flag!"),
            (
                "unexpected_server_flag_pickup",
                "MC-COMPAT-MILESTONE flag_pickup username=",
            ),
            ("unexpected_flag_return", "MC-COMPAT-MILESTONE flag_return"),
            (
                "unexpected_flag_disconnect_return",
                "MC-COMPAT-MILESTONE flag_disconnect_return",
            ),
            ("unexpected_flag_capture", "You captured the flag!"),
            ("unexpected_flag_capture_milestone", "flag_capture"),
            ("unexpected_red_score", "RED: 1"),
            ("unexpected_blue_score", "BLUE: 1"),
        ],
        behavior: ScenarioBehaviorKind::CtfInvalidReturnDrop,
    },
    ScenarioSpec {
        scenario: Scenario::CtfScoreLimitWinCondition,
        canonical_name: "ctf-score-limit-win-condition",
        aliases: &["ctf-score-limit-win-condition"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_2", "RED: 2"),
            (
                "ctf_score_limit_win_seen",
                CTF_SCORE_LIMIT_CLIENT_WIN_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_score_limit_pre_state",
                CTF_SCORE_LIMIT_SERVER_PRE_STATE_NEEDLE,
            ),
            (
                "server_score_limit_final_capture",
                CTF_SCORE_LIMIT_SERVER_FINAL_CAPTURE_NEEDLE,
            ),
            (
                "server_score_limit_win_condition",
                CTF_SCORE_LIMIT_SERVER_WIN_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("score_limit_duplicate_win", "score_limit_duplicate_win"),
            (
                "score_limit_post_win_score_mutation",
                "score_limit_post_win_score_mutation",
            ),
            ("unexpected_red_score_3", "RED: 3"),
            ("unexpected_blue_score_1", "BLUE: 1"),
        ],
        behavior: ScenarioBehaviorKind::CtfScoreLimitWinCondition,
    },
    ScenarioSpec {
        scenario: Scenario::CtfSimultaneousPickupCaptureRace,
        canonical_name: "ctf-simultaneous-pickup-capture-race",
        aliases: &["ctf-simultaneous-pickup-capture-race"],
        client_milestones: &[
            ("ctf_race_client_count", CTF_RACE_CLIENT_COUNT_NEEDLE),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_1", "RED: 1"),
        ],
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            (
                "server_ctf_race_accepted_transition",
                CTF_RACE_ACCEPTED_SERVER_NEEDLE,
            ),
            (
                "server_ctf_race_rejected_transition",
                CTF_RACE_REJECTED_SERVER_NEEDLE,
            ),
            ("server_ctf_race_final_state", CTF_RACE_FINAL_SERVER_NEEDLE),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("ctf_race_double_accept", "ctf_race_double_accept"),
            ("unexpected_blue_score_1", "BLUE: 1"),
            ("unexpected_red_score_2", "RED: 2"),
        ],
        behavior: ScenarioBehaviorKind::CtfSimultaneousPickupCaptureRace,
    },
    ScenarioSpec {
        scenario: Scenario::CtfSpawnTeamBalanceReset,
        canonical_name: "ctf-spawn-team-balance-reset",
        aliases: &["ctf-spawn-team-balance-reset"],
        client_milestones: &[
            (
                "ctf_spawn_team_reset_client_count",
                CTF_SPAWN_TEAM_RESET_CLIENT_COUNT_NEEDLE,
            ),
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            ("team_red", "You are on team RED!"),
            ("team_blue", "You are on team BLUE!"),
            ("flag_pickup", "You have the flag!"),
            ("flag_capture", "You captured the flag!"),
            ("score_red_1", "RED: 1"),
        ],
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            (
                "server_ctf_spawn_red_assignment",
                CTF_SPAWN_TEAM_RED_ASSIGNMENT_NEEDLE,
            ),
            (
                "server_ctf_spawn_blue_assignment",
                CTF_SPAWN_TEAM_BLUE_ASSIGNMENT_NEEDLE,
            ),
            ("server_ctf_spawn_team_balance", CTF_SPAWN_TEAM_BALANCE_NEEDLE),
            (
                "server_ctf_spawn_resource_reset",
                CTF_SPAWN_RESOURCE_RESET_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
            ("spawn_team_imbalance", "ctf_spawn_team_imbalance"),
            (
                "spawn_resource_stale",
                "ctf_spawn_resource_stale_after_reset",
            ),
            ("unexpected_blue_score_1", "BLUE: 1"),
            ("unexpected_red_score_2", "RED: 2"),
        ],
        behavior: ScenarioBehaviorKind::CtfSpawnTeamBalanceReset,
    },
];

const BLOCK_ENTITY_PACKET_ROWS: &[&str] = &["play/clientbound/0x08 BlockEntityUpdateS2CPacket"];
const CHAT_COMMAND_PACKET_ROWS: &[&str] = &[
    "play/serverbound/0x05 ChatMessageC2SPacket",
    "play/serverbound/0x04 CommandExecutionC2SPacket",
];
const CHUNK_BIOME_PACKET_ROWS: &[&str] = &["play/clientbound/0x0d ChunkBiomeDataS2CPacket"];
const CREATIVE_PACKET_ROWS: &[&str] = &["play/serverbound/0x2b CreativeInventoryActionC2SPacket"];
const STATUS_EFFECT_PACKET_ROWS: &[&str] = &[
    "play/clientbound/0x6c EntityStatusEffectS2CPacket",
    "play/clientbound/0x3f RemoveEntityStatusEffectS2CPacket",
];
const RECIPE_BOOK_PACKET_ROWS: &[&str] = &["play/serverbound/0x22 RecipeBookDataC2SPacket"];
const RESOURCE_PACK_PACKET_ROWS: &[&str] = &[
    "play/clientbound/0x40 ResourcePackSendS2CPacket",
    "play/serverbound/0x24 ResourcePackStatusC2SPacket",
];
const SIGN_EDITOR_PACKET_ROWS: &[&str] = &[
    "play/clientbound/0x31 SignEditorOpenS2CPacket",
    "play/serverbound/0x2e UpdateSignC2SPacket",
];

const BLOCK_ENTITY_SIGNALS: &[&str] = &["non-sign-block-entity-payload", "backend-correlation"];
const CHAT_COMMAND_SIGNALS: &[&str] = &["harmless-chat-payload", "server-containment-correlation"];
const CHUNK_BIOME_SIGNALS: &[&str] = &["chunk-biome-data-payload", "parser-or-fixture-correlation"];
const CREATIVE_SIGNALS: &[&str] = &[
    "creative-mode-precondition",
    "creative-slot-mutation",
    "server-correlation",
];
const STATUS_EFFECT_SIGNALS: &[&str] = &[
    "status-effect-apply",
    "status-effect-remove",
    "server-correlation",
];
const RECIPE_BOOK_SIGNALS: &[&str] = &["recipe-book-settings-transition", "server-correlation"];
const RESOURCE_PACK_SIGNALS: &[&str] = &[
    "local-resource-pack-offer",
    "status-response",
    "no-external-fetch",
];
const SIGN_EDITOR_SIGNALS: &[&str] = &[
    "sign-editor-open",
    "sign-update-submit",
    "server-accepted-update",
];

const BLOCK_ENTITY_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "all_block_entities",
    "arbitrary_nbt_parity",
];
const CHAT_COMMAND_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "chat_signing_security",
    "all_commands",
];
const CHUNK_BIOME_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "all_biome_semantics",
    "all_chunk_semantics",
];
const CREATIVE_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "all_creative_inventory_semantics",
    "all_slots",
    "all_items",
];
const STATUS_EFFECT_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "all_effects",
    "gameplay_modifiers",
];
const RECIPE_BOOK_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "recipe_book_ui_behavior",
    "all_recipes",
];
const RESOURCE_PACK_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "asset_download_application",
    "trust_security_validation",
];
const SIGN_EDITOR_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "sign_editing_ui_behavior",
    "all_sign_variants",
    "all_block_entities",
];

pub(crate) const SCENARIO_LIVE_CAPABILITIES: &[ScenarioLiveCapability] = &[
    ScenarioLiveCapability {
        scenario: "survival-block-entity-persistence-parity",
        targeted_row: "block-entity-update-breadth",
        packet_rows: BLOCK_ENTITY_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: "valence-sign-persistence-rail",
        client_path: "stevenarella-sign-persistence-rail",
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_signals: BLOCK_ENTITY_SIGNALS,
        required_nonclaims: BLOCK_ENTITY_NONCLAIMS,
        blocker_reason: Some(
            "sign persistence rail does not prove non-sign block-entity update breadth",
        ),
    },
    ScenarioLiveCapability {
        scenario: "mcp-controlled-smoke",
        targeted_row: "chat-command-containment",
        packet_rows: CHAT_COMMAND_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: "owned-local-chat-or-command-rail-missing",
        client_path: "stevenarella-mcp-chat-control-candidate",
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_signals: CHAT_COMMAND_SIGNALS,
        required_nonclaims: CHAT_COMMAND_NONCLAIMS,
        blocker_reason: Some(
            "MCP control exists but no targeted chat/command containment receipt is maintained",
        ),
    },
    ScenarioLiveCapability {
        scenario: "survival-biome-dimension-state",
        targeted_row: "chunk-biome-data-packet",
        packet_rows: CHUNK_BIOME_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: "valence-chunk-biome-context-rail",
        client_path: "stevenarella-chunk-context-rail",
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_signals: CHUNK_BIOME_SIGNALS,
        required_nonclaims: CHUNK_BIOME_NONCLAIMS,
        blocker_reason: Some(
            "chunk/biome context rail does not prove ChunkBiomeDataS2CPacket payload semantics",
        ),
    },
    ScenarioLiveCapability {
        scenario: "inventory-interaction",
        targeted_row: "creative-inventory-action",
        packet_rows: CREATIVE_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: "creative-mode-rail-missing",
        client_path: "stevenarella-inventory-control-candidate",
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_signals: CREATIVE_SIGNALS,
        required_nonclaims: CREATIVE_NONCLAIMS,
        blocker_reason: Some(
            "inventory rail is survival-scoped and lacks deterministic creative-mode mutation",
        ),
    },
    ScenarioLiveCapability {
        scenario: "combat-damage",
        targeted_row: "entity-status-effect-packets",
        packet_rows: STATUS_EFFECT_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: "status-effect-rail-missing",
        client_path: "stevenarella-effect-observation-candidate",
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_signals: STATUS_EFFECT_SIGNALS,
        required_nonclaims: STATUS_EFFECT_NONCLAIMS,
        blocker_reason: Some("combat rail does not apply and remove a bounded status effect"),
    },
    ScenarioLiveCapability {
        scenario: "survival-crafting-table",
        targeted_row: "recipe-book-client-settings",
        packet_rows: RECIPE_BOOK_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: "recipe-book-settings-rail-missing",
        client_path: "stevenarella-crafting-rail",
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_signals: RECIPE_BOOK_SIGNALS,
        required_nonclaims: RECIPE_BOOK_NONCLAIMS,
        blocker_reason: Some("crafting-table rail does not toggle recipe-book client settings"),
    },
    ScenarioLiveCapability {
        scenario: "mcp-controlled-smoke",
        targeted_row: "resource-pack-status",
        packet_rows: RESOURCE_PACK_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: "local-resource-pack-offer-rail-missing",
        client_path: "stevenarella-resource-pack-status-candidate",
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_signals: RESOURCE_PACK_SIGNALS,
        required_nonclaims: RESOURCE_PACK_NONCLAIMS,
        blocker_reason: Some(
            "no owned-local resource-pack offer/status rail with no-external-fetch proof exists",
        ),
    },
    ScenarioLiveCapability {
        scenario: "survival-block-entity-persistence-parity",
        targeted_row: "sign-editor-open-update",
        packet_rows: SIGN_EDITOR_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: "sign-editor-open-update-rail-missing",
        client_path: "stevenarella-sign-edit-candidate",
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_signals: SIGN_EDITOR_SIGNALS,
        required_nonclaims: SIGN_EDITOR_NONCLAIMS,
        blocker_reason: Some(
            "sign persistence rail does not automate sign editor open/update interaction",
        ),
    },
];

pub(crate) fn parse_scenario(value: &str) -> Result<Scenario, String> {
    SCENARIO_SPECS
        .iter()
        .find(|spec| spec.aliases.iter().any(|alias| *alias == value))
        .map(|spec| spec.scenario)
        .ok_or_else(|| format!("unknown scenario: {value}"))
}

pub(crate) fn scenario_spec(scenario: Scenario) -> &'static ScenarioSpec {
    SCENARIO_SPECS
        .iter()
        .find(|spec| spec.scenario == scenario)
        .unwrap_or_else(|| panic!("scenario spec missing for {scenario:?}"))
}

pub(crate) fn scenario_behavior_kind(scenario: Scenario) -> &'static ScenarioBehaviorKind {
    &scenario_spec(scenario).behavior
}

pub(crate) fn scenario_name(scenario: Scenario) -> &'static str {
    scenario_spec(scenario).canonical_name
}

pub(crate) fn scenario_required_milestones(scenario: Scenario) -> &'static [ScenarioMilestone] {
    scenario_spec(scenario).client_milestones
}

pub(crate) fn scenario_forbidden_patterns(scenario: Scenario) -> &'static [ScenarioMilestone] {
    scenario_spec(scenario).forbidden_patterns
}

pub(crate) fn server_required_milestones(scenario: Scenario) -> &'static [ScenarioMilestone] {
    scenario_spec(scenario).server_milestones
}

pub(crate) fn validate_static_scenario_specs(specs: &[ScenarioSpec]) -> Result<(), String> {
    validate_static_scenario_coverage(specs)?;
    validate_static_scenario_rows(specs)?;
    validate_static_live_capabilities(SCENARIO_LIVE_CAPABILITIES, specs)
}

pub(crate) fn scenario_live_capabilities_for_row(
    targeted_row: &str,
) -> Vec<&'static ScenarioLiveCapability> {
    SCENARIO_LIVE_CAPABILITIES
        .iter()
        .filter(|capability| capability.targeted_row == targeted_row)
        .collect()
}

pub(crate) fn validate_static_live_capabilities(
    capabilities: &[ScenarioLiveCapability],
    specs: &[ScenarioSpec],
) -> Result<(), String> {
    if capabilities.is_empty() {
        return Err("scenario live capability registry is empty".to_string());
    }
    let mut scenario_row_pairs = Vec::new();
    for capability in capabilities {
        validate_live_capability(capability, specs)?;
        let pair = (capability.scenario, capability.targeted_row);
        if scenario_row_pairs.contains(&pair) {
            return Err(format!(
                "duplicate live capability for scenario {} and row {}",
                capability.scenario, capability.targeted_row
            ));
        }
        scenario_row_pairs.push(pair);
    }
    Ok(())
}

fn validate_live_capability(
    capability: &ScenarioLiveCapability,
    specs: &[ScenarioSpec],
) -> Result<(), String> {
    if !specs
        .iter()
        .any(|spec| spec.canonical_name == capability.scenario)
    {
        return Err(format!(
            "live capability row {} names unknown scenario {}",
            capability.targeted_row, capability.scenario
        ));
    }
    if !TARGETED_PACKET_ROW_IDS.contains(&capability.targeted_row) {
        return Err(format!(
            "live capability names unknown packet row {}",
            capability.targeted_row
        ));
    }
    if capability.packet_rows.is_empty() {
        return Err(format!(
            "live capability {} has empty packet rows",
            capability.targeted_row
        ));
    }
    if !LIVE_CAPABILITY_KINDS.contains(&capability.capability_kind) {
        return Err(format!(
            "live capability {} has unsupported capability kind {}",
            capability.targeted_row, capability.capability_kind
        ));
    }
    if !LIVE_EVIDENCE_MODES.contains(&capability.evidence_mode) {
        return Err(format!(
            "live capability {} has unsupported evidence mode {}",
            capability.targeted_row, capability.evidence_mode
        ));
    }
    if capability.backend_path.is_empty() {
        return Err(format!(
            "live capability {} has empty backend path",
            capability.targeted_row
        ));
    }
    if capability.client_path.is_empty() {
        return Err(format!(
            "live capability {} has empty client path",
            capability.targeted_row
        ));
    }
    if capability.required_signals.is_empty() {
        return Err(format!(
            "live capability {} has empty required signals",
            capability.targeted_row
        ));
    }
    for nonclaim in TARGETED_PACKET_LIVE_NONCLAIMS {
        if !capability.required_nonclaims.contains(nonclaim) {
            return Err(format!(
                "live capability {} missing nonclaim {}",
                capability.targeted_row, nonclaim
            ));
        }
    }
    match (capability.capability_kind, capability.blocker_reason) {
        (LIVE_CAPABILITY_KIND_BLOCKED, Some(reason)) if !reason.is_empty() => Ok(()),
        (LIVE_CAPABILITY_KIND_BLOCKED, _) => Err(format!(
            "blocked live capability {} lacks blocker reason",
            capability.targeted_row
        )),
        (LIVE_CAPABILITY_KIND_PROBE, None) => Ok(()),
        (LIVE_CAPABILITY_KIND_PROBE, Some(_)) => Err(format!(
            "live probe capability {} unexpectedly has blocker reason",
            capability.targeted_row
        )),
        _ => Err(format!(
            "live capability {} has inconsistent kind {}",
            capability.targeted_row, capability.capability_kind
        )),
    }
}

fn validate_static_scenario_coverage(specs: &[ScenarioSpec]) -> Result<(), String> {
    if specs.len() != ALL_SCENARIOS.len() {
        return Err(format!(
            "scenario spec count mismatch: specs={} expected={}",
            specs.len(),
            ALL_SCENARIOS.len()
        ));
    }
    for scenario in ALL_SCENARIOS {
        let count = specs
            .iter()
            .filter(|spec| spec.scenario == *scenario)
            .count();
        if count != 1 {
            return Err(format!(
                "scenario {scenario:?} has {count} specs; expected exactly one"
            ));
        }
    }
    Ok(())
}

fn validate_static_scenario_rows(specs: &[ScenarioSpec]) -> Result<(), String> {
    let mut canonical_names = Vec::new();
    for spec in specs {
        if spec.canonical_name.is_empty() {
            return Err(format!(
                "scenario {:?} has empty canonical name",
                spec.scenario
            ));
        }
        if canonical_names.contains(&spec.canonical_name) {
            return Err(format!("duplicated canonical name {}", spec.canonical_name));
        }
        canonical_names.push(spec.canonical_name);
        if !spec.aliases.contains(&spec.canonical_name) {
            return Err(format!(
                "scenario {} aliases omit canonical name",
                spec.canonical_name
            ));
        }
        if spec.client_milestones.is_empty() {
            return Err(format!(
                "scenario {} has missing client milestones",
                spec.canonical_name
            ));
        }
        if spec.forbidden_patterns.is_empty() {
            return Err(format!(
                "scenario {} has missing forbidden patterns",
                spec.canonical_name
            ));
        }
        validate_scenario_spec_manifest_parity(spec)?;
        validate_scenario_behavior_capabilities(spec)?;
    }
    Ok(())
}

fn validate_scenario_spec_manifest_parity(spec: &ScenarioSpec) -> Result<(), String> {
    let Some(row) = scenario_manifest_generated::SCENARIO_MANIFEST_ROWS
        .iter()
        .find(|row| row.name == spec.canonical_name)
    else {
        return Ok(());
    };
    validate_static_str_slice_equal("aliases", spec.canonical_name, spec.aliases, row.aliases)?;
    validate_static_str_slice_equal(
        "client milestones",
        spec.canonical_name,
        &scenario_milestone_ids(spec.client_milestones),
        row.client_milestones,
    )?;
    validate_static_str_slice_equal(
        "server milestones",
        spec.canonical_name,
        &scenario_milestone_ids(spec.server_milestones),
        row.server_milestones,
    )?;
    validate_static_str_slice_equal(
        "forbidden patterns",
        spec.canonical_name,
        &scenario_milestone_ids(spec.forbidden_patterns),
        row.forbidden_patterns,
    )?;
    Ok(())
}

fn scenario_milestone_ids(milestones: &[ScenarioMilestone]) -> Vec<&'static str> {
    milestones.iter().map(|(id, _)| *id).collect()
}

fn validate_static_str_slice_equal(
    label: &'static str,
    scenario: &'static str,
    actual: &[&'static str],
    expected: &[&'static str],
) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "scenario {scenario} {label} drift: actual={actual:?} expected={expected:?}"
        ))
    }
}

fn validate_scenario_behavior_capabilities(spec: &ScenarioSpec) -> Result<(), String> {
    if spec.scenario == Scenario::ProjectileDamageAttribution
        && !spec.behavior.uses_dynamic_projectile_health()
    {
        return Err("projectile-damage-attribution missing projectile damage hook".to_string());
    }
    if spec.scenario == Scenario::McpControlledSmoke && !spec.behavior.is_mcp_controlled_smoke() {
        return Err("mcp-controlled-smoke missing MCP control hook".to_string());
    }
    if expected_negative_live_rail_postcondition(spec.scenario).is_some()
        && spec.behavior.negative_live_rail().is_none()
    {
        return Err(format!(
            "scenario {} missing negative live rail hook",
            spec.canonical_name
        ));
    }
    if spec.behavior.run_strategy() == ScenarioRunStrategy::MultiClient
        && !spec
            .client_milestones
            .iter()
            .any(|(id, _)| id.contains("client_count") || *id == "multi_client_count")
    {
        return Err(format!(
            "scenario {} missing multi-client count milestone",
            spec.canonical_name
        ));
    }
    Ok(())
}

fn expected_negative_live_rail_postcondition(scenario: Scenario) -> Option<&'static str> {
    match scenario {
        Scenario::NegativeInventoryStaleState => Some("negative_inventory_stale_state_contained"),
        Scenario::NegativeInventoryInvalidClick => {
            Some("negative_inventory_invalid_click_restored")
        }
        Scenario::NegativeCustomPayload => Some("negative_custom_payload_contained"),
        Scenario::NegativeReconnectRace => Some("negative_reconnect_race_contained"),
        Scenario::NegativeCtfWrongScore => Some("negative_wrong_score_contained"),
        Scenario::CtfInvalidPickupOwnership => Some("ctf_invalid_pickup_contained"),
        Scenario::CtfInvalidReturnDrop => Some("ctf_invalid_return_drop_contained"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_MILESTONES: &[ScenarioMilestone] = &[];
    const EMPTY_FORBIDDEN_PATTERNS: &[ScenarioMilestone] = &[];
    const EMPTY_LIVE_SIGNALS: &[&str] = &[];
    const TARGETED_PACKET_LIVE_NONCLAIMS_WITHOUT_PRODUCTION: &[&str] = &[
        "full_protocol_763_compatibility",
        "broad_minecraft_compatibility",
        "public_server_safety",
    ];
    const COMPAT_ALIAS_MISSING_LEGACY: &[&str] = &["valence-compat-bot-probe"];

    #[test]
    fn scenario_core_validates_static_specs_and_lookup_parity() {
        validate_static_scenario_specs(SCENARIO_SPECS).expect("static scenario specs validate");
        assert_eq!(SCENARIO_SPECS.len(), ALL_SCENARIOS.len());

        for spec in SCENARIO_SPECS {
            assert_eq!(parse_scenario(spec.canonical_name), Ok(spec.scenario));
            assert_eq!(scenario_name(spec.scenario), spec.canonical_name);
            assert_eq!(
                scenario_required_milestones(spec.scenario),
                spec.client_milestones
            );
            assert_eq!(
                server_required_milestones(spec.scenario),
                spec.server_milestones
            );
            assert_eq!(
                scenario_forbidden_patterns(spec.scenario),
                spec.forbidden_patterns
            );
            assert_eq!(scenario_behavior_kind(spec.scenario), &spec.behavior);
        }

        let creative_capabilities = scenario_live_capabilities_for_row("creative-inventory-action");
        assert_eq!(creative_capabilities.len(), 1);
        assert_eq!(creative_capabilities[0].scenario, "inventory-interaction");
        assert_eq!(
            creative_capabilities[0].evidence_mode,
            LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER
        );
    }

    #[test]
    fn scenario_core_rejects_invalid_live_capabilities() {
        validate_static_live_capabilities(SCENARIO_LIVE_CAPABILITIES, SCENARIO_SPECS)
            .expect("static live capabilities validate");

        let mut duplicate = SCENARIO_LIVE_CAPABILITIES.to_vec();
        duplicate.push(duplicate[0]);
        let err = validate_static_live_capabilities(&duplicate, SCENARIO_SPECS).unwrap_err();
        assert!(err.contains("duplicate live capability"), "{err}");

        let mut unknown_scenario = SCENARIO_LIVE_CAPABILITIES.to_vec();
        unknown_scenario[0].scenario = "missing-scenario";
        let err = validate_static_live_capabilities(&unknown_scenario, SCENARIO_SPECS).unwrap_err();
        assert!(err.contains("unknown scenario"), "{err}");

        let mut unknown_row = SCENARIO_LIVE_CAPABILITIES.to_vec();
        unknown_row[0].targeted_row = "missing-packet-row";
        let err = validate_static_live_capabilities(&unknown_row, SCENARIO_SPECS).unwrap_err();
        assert!(err.contains("unknown packet row"), "{err}");

        let mut unsupported_mode = SCENARIO_LIVE_CAPABILITIES.to_vec();
        unsupported_mode[0].evidence_mode = "magic-mode";
        let err = validate_static_live_capabilities(&unsupported_mode, SCENARIO_SPECS).unwrap_err();
        assert!(err.contains("unsupported evidence mode"), "{err}");

        let mut empty_signals = SCENARIO_LIVE_CAPABILITIES.to_vec();
        empty_signals[0].required_signals = EMPTY_LIVE_SIGNALS;
        let err = validate_static_live_capabilities(&empty_signals, SCENARIO_SPECS).unwrap_err();
        assert!(err.contains("empty required signals"), "{err}");

        let mut missing_nonclaim = SCENARIO_LIVE_CAPABILITIES.to_vec();
        missing_nonclaim[0].required_nonclaims = TARGETED_PACKET_LIVE_NONCLAIMS_WITHOUT_PRODUCTION;
        let err = validate_static_live_capabilities(&missing_nonclaim, SCENARIO_SPECS).unwrap_err();
        assert!(
            err.contains("missing nonclaim production_readiness"),
            "{err}"
        );

        let mut missing_blocker = SCENARIO_LIVE_CAPABILITIES.to_vec();
        missing_blocker[0].blocker_reason = None;
        let err = validate_static_live_capabilities(&missing_blocker, SCENARIO_SPECS).unwrap_err();
        assert!(err.contains("lacks blocker reason"), "{err}");
    }

    #[test]
    fn scenario_core_rejects_invalid_static_specs() {
        let compat_index = scenario_index(Scenario::CompatBotProbe);
        let smoke_index = scenario_index(Scenario::Smoke);
        let projectile_index = scenario_index(Scenario::ProjectileDamageAttribution);

        let mut missing_alias = SCENARIO_SPECS.to_vec();
        missing_alias[compat_index].aliases = COMPAT_ALIAS_MISSING_LEGACY;
        let err = validate_static_scenario_specs(&missing_alias).unwrap_err();
        assert!(err.contains("aliases drift"), "{err}");

        let mut duplicated_name = SCENARIO_SPECS.to_vec();
        duplicated_name[compat_index].canonical_name = "smoke";
        let err = validate_static_scenario_specs(&duplicated_name).unwrap_err();
        assert!(err.contains("duplicated canonical name smoke"), "{err}");

        let mut missing_milestones = SCENARIO_SPECS.to_vec();
        missing_milestones[smoke_index].client_milestones = EMPTY_MILESTONES;
        let err = validate_static_scenario_specs(&missing_milestones).unwrap_err();
        assert!(err.contains("missing client milestones"), "{err}");

        let mut missing_forbidden = SCENARIO_SPECS.to_vec();
        missing_forbidden[smoke_index].forbidden_patterns = EMPTY_FORBIDDEN_PATTERNS;
        let err = validate_static_scenario_specs(&missing_forbidden).unwrap_err();
        assert!(err.contains("missing forbidden patterns"), "{err}");

        let mut missing_hook = SCENARIO_SPECS.to_vec();
        missing_hook[projectile_index].behavior = ScenarioBehaviorKind::Default;
        let err = validate_static_scenario_specs(&missing_hook).unwrap_err();
        assert!(err.contains("projectile damage hook"), "{err}");
    }

    fn scenario_index(scenario: Scenario) -> usize {
        SCENARIO_SPECS
            .iter()
            .position(|spec| spec.scenario == scenario)
            .unwrap_or_else(|| panic!("missing scenario {scenario:?}"))
    }
}
