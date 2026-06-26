use bevy_app::App;
use bevy_ecs::prelude::{Component, Resource, SystemSet};

pub(crate) const UPDATE_SCHEDULE_LABEL: &str = "Update";
pub(crate) const EVENT_LOOP_UPDATE_SCHEDULE_LABEL: &str = "EventLoopUpdate";
pub(crate) const EVENT_LOOP_PRE_UPDATE_SCHEDULE_LABEL: &str = "EventLoopPreUpdate";

pub(crate) const CTF_PRIMARY_ARENA_ID: &str = "ctf-primary";
pub(crate) const SURVIVAL_PRIMARY_ARENA_ID: &str = "survival-primary";
pub(crate) const TERRAIN_PRIMARY_ARENA_ID: &str = "terrain-primary";

#[derive(SystemSet, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) enum GameplayPhase {
    Input,
    RuleEvaluation,
    WorldMutation,
    Presentation,
    Cleanup,
}

pub(crate) const GAMEPLAY_PHASE_ORDER: &[GameplayPhase] = &[
    GameplayPhase::Input,
    GameplayPhase::RuleEvaluation,
    GameplayPhase::WorldMutation,
    GameplayPhase::Presentation,
    GameplayPhase::Cleanup,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GameplayInstallMode {
    ExplicitOptIn,
    SourceAdapter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GameplayScopeModel {
    LayerOwnedFixture,
    ArenaOwnedLayer,
    SourceOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GameplayScheduleContract {
    pub(crate) label: &'static str,
    pub(crate) phases: &'static [GameplayPhase],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GameplayPluginContract {
    pub(crate) plugin: &'static str,
    pub(crate) install_mode: GameplayInstallMode,
    pub(crate) scope_model: GameplayScopeModel,
    pub(crate) schedules: &'static [GameplayScheduleContract],
    pub(crate) owned_resources: &'static [&'static str],
    pub(crate) owned_events: &'static [&'static str],
    pub(crate) non_claims: &'static [&'static str],
}

#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct GameplayPluginContracts {
    contracts: Vec<GameplayPluginContract>,
}

impl GameplayPluginContracts {
    pub(crate) fn record(&mut self, contract: GameplayPluginContract) {
        if let Some(existing) = self
            .contracts
            .iter_mut()
            .find(|existing| existing.plugin == contract.plugin)
        {
            *existing = contract;
            return;
        }
        self.contracts.push(contract);
    }

    pub(crate) fn find(&self, plugin: &str) -> Option<GameplayPluginContract> {
        self.contracts
            .iter()
            .copied()
            .find(|contract| contract.plugin == plugin)
    }
}

pub(crate) fn register_gameplay_plugin_contract(app: &mut App, contract: GameplayPluginContract) {
    app.init_resource::<GameplayPluginContracts>();
    app.world_mut()
        .resource_mut::<GameplayPluginContracts>()
        .record(contract);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GameplayMode {
    Ctf,
    Survival,
    Terrain,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GameplayArenaId(&'static str);

impl GameplayArenaId {
    pub(crate) const fn new(value: &'static str) -> Self {
        Self(value)
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GameplayScope {
    pub(crate) mode: GameplayMode,
    pub(crate) arena: GameplayArenaId,
}

impl GameplayScope {
    pub(crate) const fn new(mode: GameplayMode, arena: GameplayArenaId) -> Self {
        Self { mode, arena }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GameplayScopeCheck {
    Match,
    Missing,
    ModeMismatch,
    ArenaMismatch,
}

pub(crate) fn gameplay_scope_check(
    observed: Option<&GameplayScope>,
    expected: GameplayScope,
) -> GameplayScopeCheck {
    let Some(observed) = observed else {
        return GameplayScopeCheck::Missing;
    };
    if observed.mode != expected.mode {
        return GameplayScopeCheck::ModeMismatch;
    }
    if observed.arena != expected.arena {
        return GameplayScopeCheck::ArenaMismatch;
    }
    GameplayScopeCheck::Match
}

pub(crate) fn gameplay_scope_matches(
    observed: Option<&GameplayScope>,
    expected: GameplayScope,
) -> bool {
    gameplay_scope_check(observed, expected) == GameplayScopeCheck::Match
}

#[cfg(test)]
pub(crate) fn assert_gameplay_contract_present(app: &App, plugin: &str) -> GameplayPluginContract {
    let registry = app
        .world()
        .get_resource::<GameplayPluginContracts>()
        .unwrap_or_else(|| panic!("missing shared gameplay contract registry for {plugin}"));
    registry
        .find(plugin)
        .unwrap_or_else(|| panic!("missing shared gameplay contract for {plugin}"))
}

#[cfg(test)]
pub(crate) fn assert_gameplay_contract_absent(app: &App, plugin: &str) {
    let Some(registry) = app.world().get_resource::<GameplayPluginContracts>() else {
        return;
    };
    assert!(
        registry.find(plugin).is_none(),
        "unexpected shared gameplay contract for {plugin}"
    );
}

#[cfg(test)]
pub(crate) fn assert_schedule_phases(
    contract: GameplayPluginContract,
    schedule_label: &str,
    expected: &[GameplayPhase],
) {
    let schedule = contract
        .schedules
        .iter()
        .find(|schedule| schedule.label == schedule_label)
        .unwrap_or_else(|| {
            panic!(
                "missing schedule {schedule_label} in shared gameplay contract {}",
                contract.plugin
            )
        });
    assert_eq!(schedule.phases, expected);
}
