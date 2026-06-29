use crate::gameplay_contracts::{
    GameplayArenaId, GameplayInstallMode, GameplayMode, GameplayPhase as CtfGameplayPhase,
    GameplayPluginContract, GameplayScheduleContract, GameplayScope, GameplayScopeModel,
    CTF_PRIMARY_ARENA_ID, EVENT_LOOP_UPDATE_SCHEDULE_LABEL, GAMEPLAY_PHASE_ORDER,
    UPDATE_SCHEDULE_LABEL,
};

pub(crate) const CTF_GAMEPLAY_PLUGIN_NAME: &str = "CtfGameplayPlugin";
pub(crate) const CTF_RUNTIME_CONFIG_SOURCE_PLUGIN_NAME: &str = "CtfRuntimeConfigSourcePlugin";
pub(crate) const CTF_RUNTIME_CONFIG_RELOAD_EVENT_NAME: &str = "CtfRuntimeConfigReloadEvent";
pub(crate) const CTF_PRIMARY_SCOPE: GameplayScope = GameplayScope::new(
    GameplayMode::Ctf,
    GameplayArenaId::new(CTF_PRIMARY_ARENA_ID),
);
pub(crate) const CTF_GAMEPLAY_PHASE_ORDER: &[CtfGameplayPhase] = GAMEPLAY_PHASE_ORDER;
pub(crate) const CTF_GAMEPLAY_SCHEDULES: &[GameplayScheduleContract] = &[
    GameplayScheduleContract {
        label: UPDATE_SCHEDULE_LABEL,
        phases: CTF_GAMEPLAY_PHASE_ORDER,
    },
    GameplayScheduleContract {
        label: EVENT_LOOP_UPDATE_SCHEDULE_LABEL,
        phases: CTF_GAMEPLAY_PHASE_ORDER,
    },
];
pub(crate) const CTF_SOURCE_SCHEDULES: &[GameplayScheduleContract] = CTF_GAMEPLAY_SCHEDULES;
pub(crate) const CTF_GAMEPLAY_OWNED_RESOURCES: &[&str] = &[
    "ArrowPolicyState",
    "CtfGlobals",
    "CtfGameplayPluginContract",
    "CtfLayers",
    "FlagManager",
    "Score",
];
pub(crate) const CTF_SOURCE_OWNED_RESOURCES: &[&str] = &["CtfRuntimeConfig"];
pub(crate) const CTF_SOURCE_OWNED_EVENTS: &[&str] = &[CTF_RUNTIME_CONFIG_RELOAD_EVENT_NAME];
pub(crate) const CTF_NO_OWNED_EVENTS: &[&str] = &[];
pub(crate) const CTF_NON_CLAIMS: &[&str] = &[
    "dynamic runtime plugins",
    "default Valence gameplay",
    "BedWars or Hyperion scope",
    "vanilla parity",
    "production readiness",
];
pub(crate) const CTF_GAMEPLAY_CONTRACT: GameplayPluginContract = GameplayPluginContract {
    plugin: CTF_GAMEPLAY_PLUGIN_NAME,
    install_mode: GameplayInstallMode::ExplicitOptIn,
    scope_model: GameplayScopeModel::ArenaOwnedLayer,
    schedules: CTF_GAMEPLAY_SCHEDULES,
    owned_resources: CTF_GAMEPLAY_OWNED_RESOURCES,
    owned_events: CTF_NO_OWNED_EVENTS,
    non_claims: CTF_NON_CLAIMS,
};
pub(crate) const CTF_RUNTIME_CONFIG_SOURCE_CONTRACT: GameplayPluginContract =
    GameplayPluginContract {
        plugin: CTF_RUNTIME_CONFIG_SOURCE_PLUGIN_NAME,
        install_mode: GameplayInstallMode::SourceAdapter,
        scope_model: GameplayScopeModel::SourceOnly,
        schedules: CTF_SOURCE_SCHEDULES,
        owned_resources: CTF_SOURCE_OWNED_RESOURCES,
        owned_events: CTF_SOURCE_OWNED_EVENTS,
        non_claims: CTF_NON_CLAIMS,
    };
