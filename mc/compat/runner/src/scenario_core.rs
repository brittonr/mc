use crate::scenario_catalog::*;
use crate::scenario_manifest_generated;

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
    SurvivalCraftingRecipeBreadth,
    SurvivalFurnacePersistence,
    SurvivalFurnaceSmeltingBreadth,
    SurvivalHungerFood,
    SurvivalHungerHealthCycle,
    SurvivalMobDrop,
    SurvivalMobAiLootBreadth,
    SurvivalRedstoneToggle,
    SurvivalRedstoneCircuitBreadth,
    SurvivalWorldPersistenceRestart,
    SurvivalWorldMultichunkDurability,
    SurvivalCrashRecoveryParity,
    SurvivalBlockEntityPersistenceParity,
    SurvivalContainerBlockEntityBreadth,
    SurvivalBiomeDimensionState,
    SurvivalBiomeDimensionTravel,
    SurvivalSignEditingLive,
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
    CtfInvalidOpponentBaseReturnDrop,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CreativeInventoryLiveContract {
    pub(crate) scenario: &'static str,
    pub(crate) actor: &'static str,
    pub(crate) game_mode: &'static str,
    pub(crate) semantic_slot: &'static str,
    pub(crate) wire_slot: &'static str,
    pub(crate) item: &'static str,
    pub(crate) item_count: &'static str,
    pub(crate) packet_row: &'static str,
    pub(crate) backend_path: &'static str,
    pub(crate) client_path: &'static str,
    pub(crate) expected_server_correlation: &'static str,
    pub(crate) evidence_mode: &'static str,
    pub(crate) required_nonclaims: &'static [&'static str],
    pub(crate) blocker_reason: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ResourcePackStatusLocalContract {
    pub(crate) scenario: &'static str,
    pub(crate) actor: &'static str,
    pub(crate) fixture_identity: &'static str,
    pub(crate) offer_id: &'static str,
    pub(crate) expected_status: &'static str,
    pub(crate) packet_rows: &'static [&'static str],
    pub(crate) no_external_fetch: &'static str,
    pub(crate) redaction_policy: &'static str,
    pub(crate) backend_path: &'static str,
    pub(crate) client_path: &'static str,
    pub(crate) expected_server_correlation: &'static str,
    pub(crate) evidence_mode: &'static str,
    pub(crate) required_nonclaims: &'static [&'static str],
    pub(crate) blocker_reason: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SignEditorLiveContract {
    pub(crate) scenario: &'static str,
    pub(crate) actor: &'static str,
    pub(crate) position: &'static str,
    pub(crate) initial_state: &'static str,
    pub(crate) submitted_payload: &'static str,
    pub(crate) packet_rows: &'static [&'static str],
    pub(crate) backend_path: &'static str,
    pub(crate) client_path: &'static str,
    pub(crate) expected_open_milestone: &'static str,
    pub(crate) expected_update_action: &'static str,
    pub(crate) expected_server_correlation: &'static str,
    pub(crate) evidence_mode: &'static str,
    pub(crate) required_nonclaims: &'static [&'static str],
    pub(crate) blocker_reason: &'static str,
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
    SurvivalCraftingRecipeBreadth,
    SurvivalFurnacePersistence,
    SurvivalFurnaceSmeltingBreadth,
    SurvivalHungerFood,
    SurvivalHungerHealthCycle,
    SurvivalMobDrop,
    SurvivalMobAiLootBreadth,
    SurvivalRedstoneToggle,
    SurvivalRedstoneCircuitBreadth,
    WorldPersistenceRestart {
        crash_recovery: bool,
        block_entity: bool,
    },
    SurvivalWorldMultichunkDurability,
    SurvivalContainerBlockEntityBreadth,
    SurvivalBiomeDimensionState,
    SurvivalBiomeDimensionTravel,
    SurvivalSignEditingLive,
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
    CtfInvalidOpponentBaseReturnDrop,
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
            | Self::WorldPersistenceRestart { .. }
            | Self::SurvivalWorldMultichunkDurability => ScenarioRunStrategy::ReconnectSequence,
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
            Self::CtfInvalidOpponentBaseReturnDrop => Some(NegativeLiveRailBehavior {
                invalid_action: "opponent_base_return_drop_without_carrier",
                postcondition: CTF_OPPONENT_RETURN_DROP_CLIENT_CONTAINED_NEEDLE,
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
    Scenario::SurvivalCraftingRecipeBreadth,
    Scenario::SurvivalFurnacePersistence,
    Scenario::SurvivalFurnaceSmeltingBreadth,
    Scenario::SurvivalHungerFood,
    Scenario::SurvivalHungerHealthCycle,
    Scenario::SurvivalMobDrop,
    Scenario::SurvivalMobAiLootBreadth,
    Scenario::SurvivalRedstoneToggle,
    Scenario::SurvivalRedstoneCircuitBreadth,
    Scenario::SurvivalWorldPersistenceRestart,
    Scenario::SurvivalWorldMultichunkDurability,
    Scenario::SurvivalCrashRecoveryParity,
    Scenario::SurvivalBlockEntityPersistenceParity,
    Scenario::SurvivalContainerBlockEntityBreadth,
    Scenario::SurvivalBiomeDimensionState,
    Scenario::SurvivalBiomeDimensionTravel,
    Scenario::SurvivalSignEditingLive,
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
    Scenario::CtfInvalidOpponentBaseReturnDrop,
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
        scenario: Scenario::SurvivalCraftingRecipeBreadth,
        canonical_name: "survival-crafting-recipe-breadth",
        aliases: &["survival-crafting-recipe-breadth"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_crafting_breadth_shaped_seen",
                SURVIVAL_CRAFTING_BREADTH_CLIENT_SHAPED_NEEDLE,
            ),
            (
                "survival_crafting_breadth_shapeless_seen",
                SURVIVAL_CRAFTING_BREADTH_CLIENT_SHAPELESS_NEEDLE,
            ),
            (
                "survival_crafting_breadth_grid_clear_seen",
                SURVIVAL_CRAFTING_BREADTH_CLIENT_CLEAR_NEEDLE,
            ),
            (
                "survival_crafting_breadth_invalid_seen",
                SURVIVAL_CRAFTING_BREADTH_CLIENT_INVALID_NEEDLE,
            ),
            (
                "survival_crafting_breadth_inventory_updated",
                SURVIVAL_CRAFTING_BREADTH_CLIENT_INVENTORY_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_crafting_breadth_shaped",
                SURVIVAL_CRAFTING_BREADTH_SERVER_SHAPED_NEEDLE,
            ),
            (
                "server_survival_crafting_breadth_shapeless",
                SURVIVAL_CRAFTING_BREADTH_SERVER_SHAPELESS_NEEDLE,
            ),
            (
                "server_survival_crafting_breadth_grid_clear",
                SURVIVAL_CRAFTING_BREADTH_SERVER_CLEAR_NEEDLE,
            ),
            (
                "server_survival_crafting_breadth_invalid_rejected",
                SURVIVAL_CRAFTING_BREADTH_SERVER_INVALID_NEEDLE,
            ),
            (
                "server_survival_crafting_breadth_state",
                SURVIVAL_CRAFTING_BREADTH_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalCraftingRecipeBreadth,
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
        scenario: Scenario::SurvivalFurnaceSmeltingBreadth,
        canonical_name: "survival-furnace-smelting-breadth",
        aliases: &["survival-furnace-smelting-breadth"],
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
                "survival_furnace_invalid_fuel_sent",
                SURVIVAL_FURNACE_CLIENT_INVALID_FUEL_NEEDLE,
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
                "server_survival_furnace_invalid_fuel_rejected",
                SURVIVAL_FURNACE_SERVER_INVALID_FUEL_NEEDLE,
            ),
            (
                "server_survival_furnace_breadth_state",
                SURVIVAL_FURNACE_SERVER_BREADTH_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalFurnaceSmeltingBreadth,
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
        scenario: Scenario::SurvivalHungerHealthCycle,
        canonical_name: "survival-hunger-health-cycle",
        aliases: &["survival-hunger-health-cycle"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_hunger_health_item_seen",
                SURVIVAL_HUNGER_HEALTH_CLIENT_ITEM_NEEDLE,
            ),
            (
                "survival_hunger_health_pre_seen",
                SURVIVAL_HUNGER_HEALTH_CLIENT_PRE_NEEDLE,
            ),
            (
                "survival_hunger_health_consume_sent",
                SURVIVAL_HUNGER_HEALTH_CLIENT_USE_NEEDLE,
            ),
            (
                "survival_hunger_health_recovery_seen",
                SURVIVAL_HUNGER_HEALTH_CLIENT_POST_NEEDLE,
            ),
            (
                "survival_hunger_health_inventory_updated",
                SURVIVAL_HUNGER_HEALTH_CLIENT_INVENTORY_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_hunger_health_pre",
                SURVIVAL_HUNGER_HEALTH_SERVER_PRE_NEEDLE,
            ),
            (
                "server_survival_hunger_health_consume_start",
                SURVIVAL_HUNGER_HEALTH_SERVER_CONSUME_START_NEEDLE,
            ),
            (
                "server_survival_hunger_health_consume_finish",
                SURVIVAL_HUNGER_HEALTH_SERVER_CONSUME_FINISH_NEEDLE,
            ),
            (
                "server_survival_hunger_health_inventory",
                SURVIVAL_HUNGER_HEALTH_SERVER_INVENTORY_NEEDLE,
            ),
            (
                "server_survival_hunger_health_state",
                SURVIVAL_HUNGER_HEALTH_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalHungerHealthCycle,
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
        scenario: Scenario::SurvivalMobAiLootBreadth,
        canonical_name: "survival-mob-ai-loot-breadth",
        aliases: &["survival-mob-ai-loot-breadth"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_mob_ai_loot_mob_seen",
                SURVIVAL_MOB_AI_LOOT_CLIENT_MOB_NEEDLE,
            ),
            (
                "survival_mob_ai_loot_attack_sent",
                SURVIVAL_MOB_AI_LOOT_CLIENT_ATTACK_NEEDLE,
            ),
            (
                "survival_mob_ai_loot_death_seen",
                SURVIVAL_MOB_AI_LOOT_CLIENT_DEATH_NEEDLE,
            ),
            (
                "survival_mob_ai_loot_drop_seen",
                SURVIVAL_MOB_AI_LOOT_CLIENT_DROP_NEEDLE,
            ),
            (
                "survival_mob_ai_loot_pickup_seen",
                SURVIVAL_MOB_AI_LOOT_CLIENT_PICKUP_NEEDLE,
            ),
            (
                "survival_mob_ai_loot_inventory_updated",
                SURVIVAL_MOB_AI_LOOT_CLIENT_INVENTORY_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_mob_ai_loot_spawn",
                SURVIVAL_MOB_AI_LOOT_SERVER_SPAWN_NEEDLE,
            ),
            (
                "server_survival_mob_ai_loot_ai_checkpoint",
                SURVIVAL_MOB_AI_LOOT_SERVER_AI_NEEDLE,
            ),
            (
                "server_survival_mob_ai_loot_attack",
                SURVIVAL_MOB_AI_LOOT_SERVER_ATTACK_NEEDLE,
            ),
            (
                "server_survival_mob_ai_loot_death",
                SURVIVAL_MOB_AI_LOOT_SERVER_DEATH_NEEDLE,
            ),
            (
                "server_survival_mob_ai_loot_drop_spawn",
                SURVIVAL_MOB_AI_LOOT_SERVER_DROP_NEEDLE,
            ),
            (
                "server_survival_mob_ai_loot_pickup",
                SURVIVAL_MOB_AI_LOOT_SERVER_PICKUP_NEEDLE,
            ),
            (
                "server_survival_mob_ai_loot_inventory",
                SURVIVAL_MOB_AI_LOOT_SERVER_INVENTORY_NEEDLE,
            ),
            (
                "server_survival_mob_ai_loot_state",
                SURVIVAL_MOB_AI_LOOT_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalMobAiLootBreadth,
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
        scenario: Scenario::SurvivalRedstoneCircuitBreadth,
        canonical_name: "survival-redstone-circuit-breadth",
        aliases: &["survival-redstone-circuit-breadth"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_redstone_circuit_initial_state",
                SURVIVAL_REDSTONE_CIRCUIT_CLIENT_INITIAL_NEEDLE,
            ),
            (
                "survival_redstone_circuit_input_sent",
                SURVIVAL_REDSTONE_CIRCUIT_CLIENT_INPUT_NEEDLE,
            ),
            (
                "survival_redstone_circuit_output_update",
                SURVIVAL_REDSTONE_CIRCUIT_CLIENT_OUTPUT_ON_NEEDLE,
            ),
            (
                "survival_redstone_circuit_return_input_sent",
                SURVIVAL_REDSTONE_CIRCUIT_CLIENT_RETURN_NEEDLE,
            ),
            (
                "survival_redstone_circuit_return_update",
                SURVIVAL_REDSTONE_CIRCUIT_CLIENT_OUTPUT_OFF_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_redstone_circuit_initial",
                SURVIVAL_REDSTONE_CIRCUIT_SERVER_INITIAL_NEEDLE,
            ),
            (
                "server_survival_redstone_circuit_input",
                SURVIVAL_REDSTONE_CIRCUIT_SERVER_INPUT_NEEDLE,
            ),
            (
                "server_survival_redstone_circuit_powered_on",
                SURVIVAL_REDSTONE_CIRCUIT_SERVER_ON_NEEDLE,
            ),
            (
                "server_survival_redstone_circuit_powered_off",
                SURVIVAL_REDSTONE_CIRCUIT_SERVER_OFF_NEEDLE,
            ),
            (
                "server_survival_redstone_circuit_state",
                SURVIVAL_REDSTONE_CIRCUIT_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalRedstoneCircuitBreadth,
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
        scenario: Scenario::SurvivalWorldMultichunkDurability,
        canonical_name: "survival-world-multichunk-durability",
        aliases: &["survival-world-multichunk-durability"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_world_multichunk_mutation_sent",
                SURVIVAL_WORLD_MULTICHUNK_CLIENT_MUTATION_NEEDLE,
            ),
            (
                "survival_world_multichunk_pre_restart_update",
                SURVIVAL_WORLD_MULTICHUNK_CLIENT_PRE_RESTART_NEEDLE,
            ),
            (
                "survival_world_multichunk_reconnect_sent",
                SURVIVAL_WORLD_MULTICHUNK_CLIENT_RECONNECT_NEEDLE,
            ),
            (
                "survival_world_multichunk_post_restart_update",
                SURVIVAL_WORLD_MULTICHUNK_CLIENT_POST_RESTART_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_world_multichunk_mutation",
                SURVIVAL_WORLD_MULTICHUNK_SERVER_MUTATION_NEEDLE,
            ),
            (
                "server_survival_world_multichunk_clean_shutdown",
                SURVIVAL_WORLD_MULTICHUNK_SERVER_CLEAN_NEEDLE,
            ),
            (
                "server_survival_world_multichunk_backend_restart",
                SURVIVAL_WORLD_MULTICHUNK_SERVER_RESTART_NEEDLE,
            ),
            (
                "server_survival_world_multichunk_post_restart",
                SURVIVAL_WORLD_MULTICHUNK_SERVER_POST_NEEDLE,
            ),
            (
                "server_survival_world_multichunk_state",
                SURVIVAL_WORLD_MULTICHUNK_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalWorldMultichunkDurability,
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
        scenario: Scenario::SurvivalContainerBlockEntityBreadth,
        canonical_name: "survival-container-block-entity-breadth",
        aliases: &["survival-container-block-entity-breadth"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_container_block_entity_open_seen",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_OPEN_NEEDLE,
            ),
            (
                "survival_container_block_entity_transfer_sent",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_TRANSFER_NEEDLE,
            ),
            (
                "survival_container_block_entity_payload_seen",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_PAYLOAD_NEEDLE,
            ),
            (
                "survival_container_block_entity_metadata_seen",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_METADATA_NEEDLE,
            ),
            (
                "survival_container_block_entity_reopen_seen",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_CLIENT_REOPEN_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_container_block_entity_open",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_OPEN_NEEDLE,
            ),
            (
                "server_survival_container_block_entity_transfer",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_TRANSFER_NEEDLE,
            ),
            (
                "server_survival_container_block_entity_payload",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_PAYLOAD_NEEDLE,
            ),
            (
                "server_survival_container_block_entity_metadata",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_METADATA_NEEDLE,
            ),
            (
                "server_survival_container_block_entity_state",
                SURVIVAL_CONTAINER_BLOCK_ENTITY_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalContainerBlockEntityBreadth,
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
        scenario: Scenario::SurvivalBiomeDimensionTravel,
        canonical_name: "survival-biome-dimension-travel",
        aliases: &["survival-biome-dimension-travel"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_biome_dimension_travel_origin",
                SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_ORIGIN_NEEDLE,
            ),
            (
                "survival_biome_dimension_travel_transition_sent",
                SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_TRANSITION_NEEDLE,
            ),
            (
                "survival_biome_dimension_travel_destination_seen",
                SURVIVAL_BIOME_DIMENSION_TRAVEL_CLIENT_DESTINATION_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_biome_dimension_travel_origin",
                SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_ORIGIN_NEEDLE,
            ),
            (
                "server_survival_biome_dimension_travel_transition",
                SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_TRANSITION_NEEDLE,
            ),
            (
                "server_survival_biome_dimension_travel_state",
                SURVIVAL_BIOME_DIMENSION_TRAVEL_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalBiomeDimensionTravel,
    },
    ScenarioSpec {
        scenario: Scenario::SurvivalSignEditingLive,
        canonical_name: "survival-sign-editing-live",
        aliases: &["survival-sign-editing-live"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "survival_sign_editing_open_seen",
                SURVIVAL_SIGN_EDITING_CLIENT_OPEN_NEEDLE,
            ),
            (
                "survival_sign_editing_update_sent",
                SURVIVAL_SIGN_EDITING_CLIENT_UPDATE_NEEDLE,
            ),
            (
                "survival_sign_editing_post_update_seen",
                SURVIVAL_SIGN_EDITING_CLIENT_POST_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_survival_sign_editing_open",
                SURVIVAL_SIGN_EDITING_SERVER_OPEN_NEEDLE,
            ),
            (
                "server_survival_sign_editing_update_accepted",
                SURVIVAL_SIGN_EDITING_SERVER_UPDATE_NEEDLE,
            ),
            (
                "server_survival_sign_editing_state",
                SURVIVAL_SIGN_EDITING_SERVER_STATE_NEEDLE,
            ),
        ],
        forbidden_patterns: &[
            ("panic", "panicked"),
            ("unexpected_eof", "UnexpectedEof"),
            ("protocol_mismatch", "protocol mismatch"),
            ("decode_error", "decode error"),
        ],
        behavior: ScenarioBehaviorKind::SurvivalSignEditingLive,
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
            ("projectile_spawn_visible", "projectile_probe_spawn_visible"),
            ("projectile_swing_sent", "projectile_probe_swing_sent"),
            ("projectile_travel_observed", "projectile_probe_travel_observed"),
        ],
        server_milestones: &[
            ("server_client_a_seen", "compatbota"),
            ("server_client_b_seen", "compatbotb"),
            ("server_projectile_loadout", "projectile_loadout"),
            ("server_projectile_use", "projectile_use"),
            ("server_projectile_travel_sample", "projectile_travel_sample"),
            ("server_projectile_collision", "projectile_collision"),
            ("server_projectile_hit", "projectile_hit"),
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
        scenario: Scenario::CtfInvalidOpponentBaseReturnDrop,
        canonical_name: "ctf-invalid-opponent-base-return-drop",
        aliases: &["ctf-invalid-opponent-base-return-drop"],
        client_milestones: &[
            ("protocol_detected", "Detected server protocol version"),
            ("join_game", "join_game"),
            ("render_tick", "render_tick_with_player"),
            (
                "ctf_invalid_opponent_base_return_drop_attempted",
                CTF_OPPONENT_RETURN_DROP_CLIENT_ATTEMPT_NEEDLE,
            ),
            (
                "ctf_invalid_opponent_base_return_drop_contained",
                CTF_OPPONENT_RETURN_DROP_CLIENT_CONTAINED_NEEDLE,
            ),
        ],
        server_milestones: &[
            ("server_username_seen", "compatbot"),
            (
                "server_invalid_opponent_base_return_drop_rejected",
                "invalid_opponent_base_return_drop_rejected username=compatbot actor_team=Red flag_team=Blue pre_state=at_base post_state=at_base red_score=0 blue_score=0 outcome=no_flag_state_mutation_no_score",
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
        behavior: ScenarioBehaviorKind::CtfInvalidOpponentBaseReturnDrop,
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
const CREATIVE_INVENTORY_PACKET_ROW: &str =
    "play/serverbound/0x2b CreativeInventoryActionC2SPacket";
const CREATIVE_PACKET_ROWS: &[&str] = &[CREATIVE_INVENTORY_PACKET_ROW];
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

const CREATIVE_INVENTORY_LIVE_SCENARIO: &str = "inventory-interaction";
const CREATIVE_INVENTORY_LIVE_ACTOR: &str = "compatbot";
const CREATIVE_INVENTORY_LIVE_GAME_MODE: &str = "creative";
const CREATIVE_INVENTORY_LIVE_SEMANTIC_SLOT: &str = "hotbar_0";
const CREATIVE_INVENTORY_LIVE_WIRE_SLOT: &str = "36";
const CREATIVE_INVENTORY_LIVE_ITEM: &str = "minecraft:stone";
const CREATIVE_INVENTORY_LIVE_ITEM_COUNT: &str = "64";
const CREATIVE_INVENTORY_LIVE_BACKEND_PATH: &str = "deterministic-creative-fixture-contract";
const CREATIVE_INVENTORY_LIVE_CLIENT_PATH: &str = "stevenarella-creative-action-driver-missing";
const CREATIVE_INVENTORY_LIVE_SERVER_CORRELATION: &str = "creative_slot_mutation_accepted";
const CREATIVE_INVENTORY_LIVE_BLOCKER_REASON: &str =
    "no maintained live Stevenarella creative-mode mutation driver exists";

pub(crate) const CREATIVE_INVENTORY_LIVE_CONTRACT: CreativeInventoryLiveContract =
    CreativeInventoryLiveContract {
        scenario: CREATIVE_INVENTORY_LIVE_SCENARIO,
        actor: CREATIVE_INVENTORY_LIVE_ACTOR,
        game_mode: CREATIVE_INVENTORY_LIVE_GAME_MODE,
        semantic_slot: CREATIVE_INVENTORY_LIVE_SEMANTIC_SLOT,
        wire_slot: CREATIVE_INVENTORY_LIVE_WIRE_SLOT,
        item: CREATIVE_INVENTORY_LIVE_ITEM,
        item_count: CREATIVE_INVENTORY_LIVE_ITEM_COUNT,
        packet_row: CREATIVE_INVENTORY_PACKET_ROW,
        backend_path: CREATIVE_INVENTORY_LIVE_BACKEND_PATH,
        client_path: CREATIVE_INVENTORY_LIVE_CLIENT_PATH,
        expected_server_correlation: CREATIVE_INVENTORY_LIVE_SERVER_CORRELATION,
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_nonclaims: CREATIVE_NONCLAIMS,
        blocker_reason: CREATIVE_INVENTORY_LIVE_BLOCKER_REASON,
    };
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

const RESOURCE_PACK_LOCAL_SCENARIO: &str = "mcp-controlled-smoke";
const RESOURCE_PACK_LOCAL_ACTOR: &str = "compatbot";
const RESOURCE_PACK_LOCAL_FIXTURE_IDENTITY: &str = "owned-local-resource-pack-offer-fixture";
const RESOURCE_PACK_LOCAL_OFFER_ID: &str = "mc-compat-local-resource-pack";
const RESOURCE_PACK_LOCAL_EXPECTED_STATUS: &str = "declined";
const RESOURCE_PACK_LOCAL_NO_EXTERNAL_FETCH: &str = "true";
const RESOURCE_PACK_LOCAL_REDACTION_POLICY: &str = "no-secrets-no-public-addresses";
const RESOURCE_PACK_LOCAL_BACKEND_PATH: &str = "deterministic-resource-pack-offer-contract";
const RESOURCE_PACK_LOCAL_CLIENT_PATH: &str = "stevenarella-resource-pack-status-driver";
const RESOURCE_PACK_LOCAL_SERVER_CORRELATION: &str = "resource_pack_status_declined_observed";
const RESOURCE_PACK_LOCAL_BLOCKER_REASON: &str =
    "Stevenarella resource-pack status driver exists, but no maintained live server-correlation receipt exists";

pub(crate) const RESOURCE_PACK_STATUS_LOCAL_CONTRACT: ResourcePackStatusLocalContract =
    ResourcePackStatusLocalContract {
        scenario: RESOURCE_PACK_LOCAL_SCENARIO,
        actor: RESOURCE_PACK_LOCAL_ACTOR,
        fixture_identity: RESOURCE_PACK_LOCAL_FIXTURE_IDENTITY,
        offer_id: RESOURCE_PACK_LOCAL_OFFER_ID,
        expected_status: RESOURCE_PACK_LOCAL_EXPECTED_STATUS,
        packet_rows: RESOURCE_PACK_PACKET_ROWS,
        no_external_fetch: RESOURCE_PACK_LOCAL_NO_EXTERNAL_FETCH,
        redaction_policy: RESOURCE_PACK_LOCAL_REDACTION_POLICY,
        backend_path: RESOURCE_PACK_LOCAL_BACKEND_PATH,
        client_path: RESOURCE_PACK_LOCAL_CLIENT_PATH,
        expected_server_correlation: RESOURCE_PACK_LOCAL_SERVER_CORRELATION,
        evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
        required_nonclaims: RESOURCE_PACK_NONCLAIMS,
        blocker_reason: RESOURCE_PACK_LOCAL_BLOCKER_REASON,
    };
const SIGN_EDITOR_NONCLAIMS: &[&str] = &[
    "full_protocol_763_compatibility",
    "broad_minecraft_compatibility",
    "public_server_safety",
    "production_readiness",
    "sign_editing_ui_behavior",
    "all_sign_variants",
    "all_block_entities",
];

const SIGN_EDITOR_LIVE_SCENARIO: &str = "survival-block-entity-persistence-parity";
const SIGN_EDITOR_LIVE_ACTOR: &str = "compatbot";
const SIGN_EDITOR_LIVE_POSITION: &str = "28,64,0";
const SIGN_EDITOR_LIVE_INITIAL_STATE: &str = "blank";
const SIGN_EDITOR_LIVE_PAYLOAD: &str = "MC|Compat|Sign|Edit";
const SIGN_EDITOR_LIVE_BACKEND_PATH: &str = "deterministic-sign-editor-contract";
const SIGN_EDITOR_LIVE_CLIENT_PATH: &str = "stevenarella-sign-editor-driver";
const SIGN_EDITOR_LIVE_OPEN_MILESTONE: &str = "sign_editor_open_observed";
const SIGN_EDITOR_LIVE_UPDATE_ACTION: &str = "sign_update_sent";
const SIGN_EDITOR_LIVE_SERVER_CORRELATION: &str = "sign_update_accepted_observed";
const SIGN_EDITOR_LIVE_BLOCKER_REASON: &str =
    "Stevenarella sign-editor driver exists, but no maintained live server-correlation receipt exists";

pub(crate) const SIGN_EDITOR_LIVE_CONTRACT: SignEditorLiveContract = SignEditorLiveContract {
    scenario: SIGN_EDITOR_LIVE_SCENARIO,
    actor: SIGN_EDITOR_LIVE_ACTOR,
    position: SIGN_EDITOR_LIVE_POSITION,
    initial_state: SIGN_EDITOR_LIVE_INITIAL_STATE,
    submitted_payload: SIGN_EDITOR_LIVE_PAYLOAD,
    packet_rows: SIGN_EDITOR_PACKET_ROWS,
    backend_path: SIGN_EDITOR_LIVE_BACKEND_PATH,
    client_path: SIGN_EDITOR_LIVE_CLIENT_PATH,
    expected_open_milestone: SIGN_EDITOR_LIVE_OPEN_MILESTONE,
    expected_update_action: SIGN_EDITOR_LIVE_UPDATE_ACTION,
    expected_server_correlation: SIGN_EDITOR_LIVE_SERVER_CORRELATION,
    evidence_mode: LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER,
    required_nonclaims: SIGN_EDITOR_NONCLAIMS,
    blocker_reason: SIGN_EDITOR_LIVE_BLOCKER_REASON,
};

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
        scenario: CREATIVE_INVENTORY_LIVE_CONTRACT.scenario,
        targeted_row: "creative-inventory-action",
        packet_rows: CREATIVE_PACKET_ROWS,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: CREATIVE_INVENTORY_LIVE_CONTRACT.backend_path,
        client_path: CREATIVE_INVENTORY_LIVE_CONTRACT.client_path,
        evidence_mode: CREATIVE_INVENTORY_LIVE_CONTRACT.evidence_mode,
        required_signals: CREATIVE_SIGNALS,
        required_nonclaims: CREATIVE_INVENTORY_LIVE_CONTRACT.required_nonclaims,
        blocker_reason: Some(CREATIVE_INVENTORY_LIVE_CONTRACT.blocker_reason),
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
        scenario: RESOURCE_PACK_STATUS_LOCAL_CONTRACT.scenario,
        targeted_row: "resource-pack-status",
        packet_rows: RESOURCE_PACK_STATUS_LOCAL_CONTRACT.packet_rows,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: RESOURCE_PACK_STATUS_LOCAL_CONTRACT.backend_path,
        client_path: RESOURCE_PACK_STATUS_LOCAL_CONTRACT.client_path,
        evidence_mode: RESOURCE_PACK_STATUS_LOCAL_CONTRACT.evidence_mode,
        required_signals: RESOURCE_PACK_SIGNALS,
        required_nonclaims: RESOURCE_PACK_STATUS_LOCAL_CONTRACT.required_nonclaims,
        blocker_reason: Some(RESOURCE_PACK_STATUS_LOCAL_CONTRACT.blocker_reason),
    },
    ScenarioLiveCapability {
        scenario: SIGN_EDITOR_LIVE_CONTRACT.scenario,
        targeted_row: "sign-editor-open-update",
        packet_rows: SIGN_EDITOR_LIVE_CONTRACT.packet_rows,
        capability_kind: LIVE_CAPABILITY_KIND_BLOCKED,
        backend_path: SIGN_EDITOR_LIVE_CONTRACT.backend_path,
        client_path: SIGN_EDITOR_LIVE_CONTRACT.client_path,
        evidence_mode: SIGN_EDITOR_LIVE_CONTRACT.evidence_mode,
        required_signals: SIGN_EDITOR_SIGNALS,
        required_nonclaims: SIGN_EDITOR_LIVE_CONTRACT.required_nonclaims,
        blocker_reason: Some(SIGN_EDITOR_LIVE_CONTRACT.blocker_reason),
    },
];

pub(crate) fn parse_scenario(value: &str) -> Result<Scenario, String> {
    SCENARIO_SPECS
        .iter()
        .find(|spec| spec.aliases.contains(&value))
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
    validate_creative_inventory_live_contract(&CREATIVE_INVENTORY_LIVE_CONTRACT)?;
    validate_resource_pack_status_local_contract(&RESOURCE_PACK_STATUS_LOCAL_CONTRACT)?;
    validate_sign_editor_live_contract(&SIGN_EDITOR_LIVE_CONTRACT)?;
    validate_static_live_capabilities(SCENARIO_LIVE_CAPABILITIES, specs)
}

pub(crate) fn validate_creative_inventory_live_contract(
    contract: &CreativeInventoryLiveContract,
) -> Result<(), String> {
    if contract.scenario != CREATIVE_INVENTORY_LIVE_SCENARIO {
        return Err(format!(
            "creative live contract names unexpected scenario {}",
            contract.scenario
        ));
    }
    if contract.actor != CREATIVE_INVENTORY_LIVE_ACTOR {
        return Err(format!(
            "creative live contract names unexpected actor {}",
            contract.actor
        ));
    }
    if contract.game_mode != CREATIVE_INVENTORY_LIVE_GAME_MODE {
        return Err(format!(
            "creative live contract names unexpected game mode {}",
            contract.game_mode
        ));
    }
    if contract.semantic_slot != CREATIVE_INVENTORY_LIVE_SEMANTIC_SLOT {
        return Err(format!(
            "creative live contract names unexpected semantic slot {}",
            contract.semantic_slot
        ));
    }
    if contract.wire_slot != CREATIVE_INVENTORY_LIVE_WIRE_SLOT {
        return Err(format!(
            "creative live contract names unexpected wire slot {}",
            contract.wire_slot
        ));
    }
    if contract.item != CREATIVE_INVENTORY_LIVE_ITEM {
        return Err(format!(
            "creative live contract names unexpected item {}",
            contract.item
        ));
    }
    if contract.item_count != CREATIVE_INVENTORY_LIVE_ITEM_COUNT {
        return Err(format!(
            "creative live contract names unexpected item count {}",
            contract.item_count
        ));
    }
    if contract.packet_row != CREATIVE_INVENTORY_PACKET_ROW {
        return Err(format!(
            "creative live contract names unexpected packet row {}",
            contract.packet_row
        ));
    }
    if contract.backend_path.is_empty() {
        return Err("creative live contract has empty backend path".to_string());
    }
    if contract.client_path.is_empty() {
        return Err("creative live contract has empty client path".to_string());
    }
    if contract.expected_server_correlation.is_empty() {
        return Err("creative live contract has empty server correlation".to_string());
    }
    if contract.evidence_mode != LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER {
        return Err(format!(
            "creative live contract has unsupported evidence mode {}",
            contract.evidence_mode
        ));
    }
    for nonclaim in CREATIVE_NONCLAIMS {
        if !contract.required_nonclaims.contains(nonclaim) {
            return Err(format!(
                "creative live contract missing nonclaim {nonclaim}"
            ));
        }
    }
    if contract.blocker_reason.is_empty() {
        return Err("creative live contract has empty blocker reason".to_string());
    }
    Ok(())
}

pub(crate) fn validate_resource_pack_status_local_contract(
    contract: &ResourcePackStatusLocalContract,
) -> Result<(), String> {
    if contract.scenario != RESOURCE_PACK_LOCAL_SCENARIO {
        return Err(format!(
            "resource-pack local contract names unexpected scenario {}",
            contract.scenario
        ));
    }
    if contract.actor != RESOURCE_PACK_LOCAL_ACTOR {
        return Err(format!(
            "resource-pack local contract names unexpected actor {}",
            contract.actor
        ));
    }
    if contract.fixture_identity != RESOURCE_PACK_LOCAL_FIXTURE_IDENTITY {
        return Err(format!(
            "resource-pack local contract names unexpected fixture {}",
            contract.fixture_identity
        ));
    }
    if contract.offer_id != RESOURCE_PACK_LOCAL_OFFER_ID {
        return Err(format!(
            "resource-pack local contract names unexpected offer {}",
            contract.offer_id
        ));
    }
    if contract.expected_status != RESOURCE_PACK_LOCAL_EXPECTED_STATUS {
        return Err(format!(
            "resource-pack local contract names unexpected status {}",
            contract.expected_status
        ));
    }
    if contract.packet_rows != RESOURCE_PACK_PACKET_ROWS {
        return Err("resource-pack local contract packet rows drifted".to_string());
    }
    if contract.no_external_fetch != RESOURCE_PACK_LOCAL_NO_EXTERNAL_FETCH {
        return Err(format!(
            "resource-pack local contract has unsupported no-external-fetch value {}",
            contract.no_external_fetch
        ));
    }
    if contract.redaction_policy != RESOURCE_PACK_LOCAL_REDACTION_POLICY {
        return Err(format!(
            "resource-pack local contract names unexpected redaction policy {}",
            contract.redaction_policy
        ));
    }
    if contract.backend_path.is_empty() {
        return Err("resource-pack local contract has empty backend path".to_string());
    }
    if contract.client_path.is_empty() {
        return Err("resource-pack local contract has empty client path".to_string());
    }
    if contract.expected_server_correlation.is_empty() {
        return Err("resource-pack local contract has empty server correlation".to_string());
    }
    if contract.evidence_mode != LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER {
        return Err(format!(
            "resource-pack local contract has unsupported evidence mode {}",
            contract.evidence_mode
        ));
    }
    for nonclaim in RESOURCE_PACK_NONCLAIMS {
        if !contract.required_nonclaims.contains(nonclaim) {
            return Err(format!(
                "resource-pack local contract missing nonclaim {nonclaim}"
            ));
        }
    }
    if contract.blocker_reason.is_empty() {
        return Err("resource-pack local contract has empty blocker reason".to_string());
    }
    Ok(())
}

pub(crate) fn validate_sign_editor_live_contract(
    contract: &SignEditorLiveContract,
) -> Result<(), String> {
    if contract.scenario != SIGN_EDITOR_LIVE_SCENARIO {
        return Err(format!(
            "sign editor contract names unexpected scenario {}",
            contract.scenario
        ));
    }
    if contract.actor != SIGN_EDITOR_LIVE_ACTOR {
        return Err(format!(
            "sign editor contract names unexpected actor {}",
            contract.actor
        ));
    }
    if contract.position != SIGN_EDITOR_LIVE_POSITION {
        return Err(format!(
            "sign editor contract names unexpected position {}",
            contract.position
        ));
    }
    if contract.initial_state != SIGN_EDITOR_LIVE_INITIAL_STATE {
        return Err(format!(
            "sign editor contract names unexpected initial state {}",
            contract.initial_state
        ));
    }
    if contract.submitted_payload != SIGN_EDITOR_LIVE_PAYLOAD {
        return Err(format!(
            "sign editor contract names unexpected payload {}",
            contract.submitted_payload
        ));
    }
    if contract.packet_rows != SIGN_EDITOR_PACKET_ROWS {
        return Err("sign editor contract packet rows drifted".to_string());
    }
    if contract.backend_path.is_empty() {
        return Err("sign editor contract has empty backend path".to_string());
    }
    if contract.client_path.is_empty() {
        return Err("sign editor contract has empty client path".to_string());
    }
    if contract.expected_open_milestone.is_empty() {
        return Err("sign editor contract has empty open milestone".to_string());
    }
    if contract.expected_update_action.is_empty() {
        return Err("sign editor contract has empty update action".to_string());
    }
    if contract.expected_server_correlation.is_empty() {
        return Err("sign editor contract has empty server correlation".to_string());
    }
    if contract.evidence_mode != LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER {
        return Err(format!(
            "sign editor contract has unsupported evidence mode {}",
            contract.evidence_mode
        ));
    }
    for nonclaim in SIGN_EDITOR_NONCLAIMS {
        if !contract.required_nonclaims.contains(nonclaim) {
            return Err(format!("sign editor contract missing nonclaim {nonclaim}"));
        }
    }
    if contract.blocker_reason.is_empty() {
        return Err("sign editor contract has empty blocker reason".to_string());
    }
    Ok(())
}

#[cfg(test)]
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
        Scenario::CtfInvalidOpponentBaseReturnDrop => {
            Some(CTF_OPPONENT_RETURN_DROP_CLIENT_CONTAINED_NEEDLE)
        }
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
    const WRONG_CREATIVE_PACKET_ROW: &str = "play/serverbound/0x00 WrongPacket";
    const WRONG_RESOURCE_PACK_STATUS: &str = "accepted";
    const WRONG_RESOURCE_PACK_NO_EXTERNAL_FETCH: &str = "false";
    const WRONG_SIGN_EDITOR_POSITION: &str = "0,0,0";
    const WRONG_SIGN_EDITOR_PAYLOAD: &str = "wrong";
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
        assert_eq!(
            creative_capabilities[0].scenario,
            CREATIVE_INVENTORY_LIVE_CONTRACT.scenario
        );
        assert_eq!(
            creative_capabilities[0].backend_path,
            CREATIVE_INVENTORY_LIVE_CONTRACT.backend_path
        );
        assert_eq!(
            creative_capabilities[0].evidence_mode,
            LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER
        );
        validate_creative_inventory_live_contract(&CREATIVE_INVENTORY_LIVE_CONTRACT)
            .expect("creative live contract validates");

        let resource_pack_capabilities = scenario_live_capabilities_for_row("resource-pack-status");
        assert_eq!(resource_pack_capabilities.len(), 1);
        assert_eq!(
            resource_pack_capabilities[0].backend_path,
            RESOURCE_PACK_STATUS_LOCAL_CONTRACT.backend_path
        );
        assert_eq!(
            resource_pack_capabilities[0].evidence_mode,
            LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER
        );
        validate_resource_pack_status_local_contract(&RESOURCE_PACK_STATUS_LOCAL_CONTRACT)
            .expect("resource-pack local contract validates");

        let sign_editor_capabilities =
            scenario_live_capabilities_for_row("sign-editor-open-update");
        assert_eq!(sign_editor_capabilities.len(), 1);
        assert_eq!(
            sign_editor_capabilities[0].backend_path,
            SIGN_EDITOR_LIVE_CONTRACT.backend_path
        );
        assert_eq!(
            sign_editor_capabilities[0].evidence_mode,
            LIVE_EVIDENCE_MODE_FIXTURE_BOUNDED_BLOCKER
        );
        validate_sign_editor_live_contract(&SIGN_EDITOR_LIVE_CONTRACT)
            .expect("sign editor contract validates");
    }

    #[test]
    fn scenario_core_rejects_invalid_creative_inventory_live_contracts() {
        let mut wrong_packet = CREATIVE_INVENTORY_LIVE_CONTRACT;
        wrong_packet.packet_row = WRONG_CREATIVE_PACKET_ROW;
        let err = validate_creative_inventory_live_contract(&wrong_packet).unwrap_err();
        assert!(err.contains("unexpected packet row"), "{err}");

        let mut missing_correlation = CREATIVE_INVENTORY_LIVE_CONTRACT;
        missing_correlation.expected_server_correlation = "";
        let err = validate_creative_inventory_live_contract(&missing_correlation).unwrap_err();
        assert!(err.contains("empty server correlation"), "{err}");

        let mut missing_nonclaim = CREATIVE_INVENTORY_LIVE_CONTRACT;
        missing_nonclaim.required_nonclaims = TARGETED_PACKET_LIVE_NONCLAIMS_WITHOUT_PRODUCTION;
        let err = validate_creative_inventory_live_contract(&missing_nonclaim).unwrap_err();
        assert!(
            err.contains("missing nonclaim production_readiness"),
            "{err}"
        );

        let mut unsupported_mode = CREATIVE_INVENTORY_LIVE_CONTRACT;
        unsupported_mode.evidence_mode = LIVE_EVIDENCE_MODE_OWNED_LOCAL;
        let err = validate_creative_inventory_live_contract(&unsupported_mode).unwrap_err();
        assert!(err.contains("unsupported evidence mode"), "{err}");
    }

    #[test]
    fn scenario_core_rejects_invalid_resource_pack_status_local_contracts() {
        let mut wrong_status = RESOURCE_PACK_STATUS_LOCAL_CONTRACT;
        wrong_status.expected_status = WRONG_RESOURCE_PACK_STATUS;
        let err = validate_resource_pack_status_local_contract(&wrong_status).unwrap_err();
        assert!(err.contains("unexpected status"), "{err}");

        let mut missing_local_scope = RESOURCE_PACK_STATUS_LOCAL_CONTRACT;
        missing_local_scope.no_external_fetch = WRONG_RESOURCE_PACK_NO_EXTERNAL_FETCH;
        let err = validate_resource_pack_status_local_contract(&missing_local_scope).unwrap_err();
        assert!(err.contains("no-external-fetch"), "{err}");

        let mut missing_correlation = RESOURCE_PACK_STATUS_LOCAL_CONTRACT;
        missing_correlation.expected_server_correlation = "";
        let err = validate_resource_pack_status_local_contract(&missing_correlation).unwrap_err();
        assert!(err.contains("empty server correlation"), "{err}");

        let mut missing_nonclaim = RESOURCE_PACK_STATUS_LOCAL_CONTRACT;
        missing_nonclaim.required_nonclaims = TARGETED_PACKET_LIVE_NONCLAIMS_WITHOUT_PRODUCTION;
        let err = validate_resource_pack_status_local_contract(&missing_nonclaim).unwrap_err();
        assert!(
            err.contains("missing nonclaim production_readiness"),
            "{err}"
        );
    }

    #[test]
    fn scenario_core_rejects_invalid_sign_editor_live_contracts() {
        let mut wrong_position = SIGN_EDITOR_LIVE_CONTRACT;
        wrong_position.position = WRONG_SIGN_EDITOR_POSITION;
        let err = validate_sign_editor_live_contract(&wrong_position).unwrap_err();
        assert!(err.contains("unexpected position"), "{err}");

        let mut wrong_payload = SIGN_EDITOR_LIVE_CONTRACT;
        wrong_payload.submitted_payload = WRONG_SIGN_EDITOR_PAYLOAD;
        let err = validate_sign_editor_live_contract(&wrong_payload).unwrap_err();
        assert!(err.contains("unexpected payload"), "{err}");

        let mut missing_update = SIGN_EDITOR_LIVE_CONTRACT;
        missing_update.expected_update_action = "";
        let err = validate_sign_editor_live_contract(&missing_update).unwrap_err();
        assert!(err.contains("empty update action"), "{err}");

        let mut missing_nonclaim = SIGN_EDITOR_LIVE_CONTRACT;
        missing_nonclaim.required_nonclaims = TARGETED_PACKET_LIVE_NONCLAIMS_WITHOUT_PRODUCTION;
        let err = validate_sign_editor_live_contract(&missing_nonclaim).unwrap_err();
        assert!(
            err.contains("missing nonclaim production_readiness"),
            "{err}"
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
