//! Shared gameplay plugin contract helpers for compiled Valence examples.
//!
//! New gameplay plugins define a `GameplayPluginContract` with plugin identity,
//! install mode, scope model, optional concrete scope, schedule phases, owned
//! resources/events, and explicit non-claims. The helper validates and records
//! that descriptor; gameplay logic, Bevy schedule wiring, packet I/O, resource
//! mutation, and compatibility milestone emission remain in plugin-owned shells.
//! Runtime-loaded plugins, scripting, sandboxing, default Valence gameplay,
//! vanilla parity, and production readiness are not claimed here.

// Each example path-includes this helper and intentionally uses a different subset.
#![allow(dead_code)]

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
    pub(crate) scope: Option<GameplayScope>,
    pub(crate) schedules: &'static [GameplayScheduleContract],
    pub(crate) owned_resources: &'static [&'static str],
    pub(crate) owned_events: &'static [&'static str],
    pub(crate) non_claims: &'static [&'static str],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GameplayPluginTemplate {
    contract: GameplayPluginContract,
}

impl GameplayPluginTemplate {
    pub(crate) const fn new(contract: GameplayPluginContract) -> Self {
        Self { contract }
    }

    pub(crate) const fn contract(self) -> GameplayPluginContract {
        self.contract
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct GameplayPluginInstanceKey {
    pub(crate) plugin: &'static str,
    pub(crate) scope: Option<GameplayScope>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GameplayContractError {
    MissingPlugin,
    MissingSchedule {
        plugin: &'static str,
    },
    MissingPhase {
        plugin: &'static str,
        schedule: &'static str,
    },
    MissingOwnership {
        plugin: &'static str,
    },
    MissingNonClaim {
        plugin: &'static str,
    },
    MissingScope {
        plugin: &'static str,
    },
    UnexpectedScope {
        plugin: &'static str,
    },
    InvalidSourceScopeModel {
        plugin: &'static str,
    },
    DuplicatePlugin {
        plugin: &'static str,
    },
    StaleResource {
        plugin: &'static str,
        resource: &'static str,
    },
    StaleEvent {
        plugin: &'static str,
        event: &'static str,
    },
    StaleNonClaim {
        plugin: &'static str,
        non_claim: &'static str,
    },
    StaleSchedule {
        plugin: &'static str,
        schedule: &'static str,
    },
    StalePhaseOrder {
        plugin: &'static str,
        schedule: &'static str,
    },
}

#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct GameplayPluginContracts {
    contracts: Vec<GameplayPluginContract>,
}

impl GameplayPluginContracts {
    pub(crate) fn record(&mut self, contract: GameplayPluginContract) {
        self.try_record(contract)
            .unwrap_or_else(|error| panic!("invalid shared gameplay contract: {error:?}"));
    }

    pub(crate) fn try_record(
        &mut self,
        contract: GameplayPluginContract,
    ) -> Result<(), GameplayContractError> {
        validate_gameplay_plugin_contract(contract)?;
        let instance_key = gameplay_plugin_instance_key(contract);
        if self
            .contracts
            .iter()
            .copied()
            .any(|existing| gameplay_plugin_instance_key(existing) == instance_key)
        {
            return Err(GameplayContractError::DuplicatePlugin {
                plugin: contract.plugin,
            });
        }
        self.contracts.push(contract);
        Ok(())
    }

    pub(crate) fn find(&self, plugin: &str) -> Option<GameplayPluginContract> {
        self.contracts
            .iter()
            .copied()
            .find(|contract| contract.plugin == plugin)
    }

    pub(crate) fn find_instance(
        &self,
        key: GameplayPluginInstanceKey,
    ) -> Option<GameplayPluginContract> {
        self.contracts
            .iter()
            .copied()
            .find(|contract| gameplay_plugin_instance_key(*contract) == key)
    }
}

pub(crate) fn register_gameplay_plugin_contract(app: &mut App, contract: GameplayPluginContract) {
    register_gameplay_plugin_template(app, GameplayPluginTemplate::new(contract));
}

pub(crate) fn register_gameplay_plugin_template(app: &mut App, template: GameplayPluginTemplate) {
    validate_gameplay_plugin_template(template)
        .unwrap_or_else(|error| panic!("invalid shared gameplay plugin template: {error:?}"));
    app.init_resource::<GameplayPluginContracts>();
    app.world_mut()
        .resource_mut::<GameplayPluginContracts>()
        .record(template.contract());
}

pub(crate) fn validate_gameplay_plugin_template(
    template: GameplayPluginTemplate,
) -> Result<(), GameplayContractError> {
    validate_gameplay_plugin_contract(template.contract())
}

pub(crate) fn validate_gameplay_plugin_contract(
    contract: GameplayPluginContract,
) -> Result<(), GameplayContractError> {
    if contract.plugin.is_empty() {
        return Err(GameplayContractError::MissingPlugin);
    }
    if contract.schedules.is_empty() {
        return Err(GameplayContractError::MissingSchedule {
            plugin: contract.plugin,
        });
    }
    for schedule in contract.schedules {
        if schedule.phases.is_empty() {
            return Err(GameplayContractError::MissingPhase {
                plugin: contract.plugin,
                schedule: schedule.label,
            });
        }
    }
    if contract.owned_resources.is_empty() && contract.owned_events.is_empty() {
        return Err(GameplayContractError::MissingOwnership {
            plugin: contract.plugin,
        });
    }
    if contract.non_claims.is_empty() {
        return Err(GameplayContractError::MissingNonClaim {
            plugin: contract.plugin,
        });
    }
    validate_scope_contract(contract)
}

fn validate_scope_contract(contract: GameplayPluginContract) -> Result<(), GameplayContractError> {
    match (contract.install_mode, contract.scope_model, contract.scope) {
        (GameplayInstallMode::ExplicitOptIn, GameplayScopeModel::SourceOnly, _) => {
            Err(GameplayContractError::InvalidSourceScopeModel {
                plugin: contract.plugin,
            })
        }
        (GameplayInstallMode::ExplicitOptIn, _, None) => Err(GameplayContractError::MissingScope {
            plugin: contract.plugin,
        }),
        (GameplayInstallMode::ExplicitOptIn, _, Some(_)) => Ok(()),
        (GameplayInstallMode::SourceAdapter, GameplayScopeModel::SourceOnly, None) => Ok(()),
        (GameplayInstallMode::SourceAdapter, GameplayScopeModel::SourceOnly, Some(_)) => {
            Err(GameplayContractError::UnexpectedScope {
                plugin: contract.plugin,
            })
        }
        (GameplayInstallMode::SourceAdapter, _, _) => {
            Err(GameplayContractError::InvalidSourceScopeModel {
                plugin: contract.plugin,
            })
        }
    }
}

pub(crate) fn gameplay_plugin_instance_key(
    contract: GameplayPluginContract,
) -> GameplayPluginInstanceKey {
    GameplayPluginInstanceKey {
        plugin: contract.plugin,
        scope: contract.scope,
    }
}

pub(crate) fn contract_has_resource(
    contract: GameplayPluginContract,
    resource: &'static str,
) -> Result<(), GameplayContractError> {
    if contract.owned_resources.contains(&resource) {
        return Ok(());
    }
    Err(GameplayContractError::StaleResource {
        plugin: contract.plugin,
        resource,
    })
}

pub(crate) fn contract_has_event(
    contract: GameplayPluginContract,
    event: &'static str,
) -> Result<(), GameplayContractError> {
    if contract.owned_events.contains(&event) {
        return Ok(());
    }
    Err(GameplayContractError::StaleEvent {
        plugin: contract.plugin,
        event,
    })
}

pub(crate) fn contract_has_non_claim(
    contract: GameplayPluginContract,
    non_claim: &'static str,
) -> Result<(), GameplayContractError> {
    if contract.non_claims.contains(&non_claim) {
        return Ok(());
    }
    Err(GameplayContractError::StaleNonClaim {
        plugin: contract.plugin,
        non_claim,
    })
}

pub(crate) fn contract_has_schedule_phases(
    contract: GameplayPluginContract,
    schedule_label: &'static str,
    expected: &[GameplayPhase],
) -> Result<(), GameplayContractError> {
    let Some(schedule) = contract
        .schedules
        .iter()
        .find(|schedule| schedule.label == schedule_label)
    else {
        return Err(GameplayContractError::StaleSchedule {
            plugin: contract.plugin,
            schedule: schedule_label,
        });
    };
    if schedule.phases != expected {
        return Err(GameplayContractError::StalePhaseOrder {
            plugin: contract.plugin,
            schedule: schedule_label,
        });
    }
    Ok(())
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
    schedule_label: &'static str,
    expected: &[GameplayPhase],
) {
    contract_has_schedule_phases(contract, schedule_label, expected)
        .unwrap_or_else(|error| panic!("invalid shared gameplay schedule metadata: {error:?}"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_ecs::schedule::Schedules;

    const TEST_PLUGIN: &str = "TestGameplayPlugin";
    const TEST_SOURCE_PLUGIN: &str = "TestRuntimeConfigSourcePlugin";
    const TEST_RESOURCE: &str = "TestGameplayPluginContract";
    const TEST_EVENT: &str = "TestRuntimeConfigReloadEvent";
    const TEST_NON_CLAIM: &str = "dynamic runtime plugins";
    const TEST_MISSING_RESOURCE: &str = "MissingResource";
    const TEST_MISSING_EVENT: &str = "MissingEvent";
    const TEST_MISSING_NON_CLAIM: &str = "vanilla parity";
    const TEST_ALT_ARENA_ID: &str = "test-alt";
    const TEST_SCOPE: GameplayScope = GameplayScope::new(
        GameplayMode::Ctf,
        GameplayArenaId::new(CTF_PRIMARY_ARENA_ID),
    );
    const TEST_ALT_SCOPE: GameplayScope =
        GameplayScope::new(GameplayMode::Ctf, GameplayArenaId::new(TEST_ALT_ARENA_ID));
    const TEST_SCHEDULES: &[GameplayScheduleContract] = &[GameplayScheduleContract {
        label: UPDATE_SCHEDULE_LABEL,
        phases: GAMEPLAY_PHASE_ORDER,
    }];
    const TEST_EMPTY_PHASE_SCHEDULES: &[GameplayScheduleContract] = &[GameplayScheduleContract {
        label: UPDATE_SCHEDULE_LABEL,
        phases: &[],
    }];
    const TEST_RESOURCES: &[&str] = &[TEST_RESOURCE];
    const TEST_EVENTS: &[&str] = &[TEST_EVENT];
    const TEST_NO_EVENTS: &[&str] = &[];
    const TEST_NON_CLAIMS: &[&str] = &[TEST_NON_CLAIM];
    const TEST_REGRESSED_PHASE_ORDER: &[GameplayPhase] =
        &[GameplayPhase::Cleanup, GameplayPhase::Input];

    fn explicit_contract(plugin: &'static str, scope: GameplayScope) -> GameplayPluginContract {
        GameplayPluginContract {
            plugin,
            install_mode: GameplayInstallMode::ExplicitOptIn,
            scope_model: GameplayScopeModel::ArenaOwnedLayer,
            scope: Some(scope),
            schedules: TEST_SCHEDULES,
            owned_resources: TEST_RESOURCES,
            owned_events: TEST_NO_EVENTS,
            non_claims: TEST_NON_CLAIMS,
        }
    }

    fn source_contract() -> GameplayPluginContract {
        GameplayPluginContract {
            plugin: TEST_SOURCE_PLUGIN,
            install_mode: GameplayInstallMode::SourceAdapter,
            scope_model: GameplayScopeModel::SourceOnly,
            scope: None,
            schedules: TEST_SCHEDULES,
            owned_resources: &[],
            owned_events: TEST_EVENTS,
            non_claims: TEST_NON_CLAIMS,
        }
    }

    fn schedule_count(app: &App) -> Option<usize> {
        app.world()
            .get_resource::<Schedules>()
            .map(|schedules| schedules.iter().count())
    }

    #[test]
    fn gameplay_template_registers_contract_without_hidden_schedule_mutation() {
        let mut app = App::new();
        let initial_schedule_count = schedule_count(&app);
        let template = GameplayPluginTemplate::new(explicit_contract(TEST_PLUGIN, TEST_SCOPE));

        register_gameplay_plugin_template(&mut app, template);

        let contract = assert_gameplay_contract_present(&app, TEST_PLUGIN);
        assert_eq!(contract.scope, Some(TEST_SCOPE));
        assert_eq!(contract.install_mode, GameplayInstallMode::ExplicitOptIn);
        assert_eq!(schedule_count(&app), initial_schedule_count);
        contract_has_resource(contract, TEST_RESOURCE).unwrap();
        contract_has_non_claim(contract, TEST_NON_CLAIM).unwrap();
        contract_has_schedule_phases(contract, UPDATE_SCHEDULE_LABEL, GAMEPLAY_PHASE_ORDER)
            .unwrap();
    }

    #[test]
    fn source_adapter_template_registers_events_without_runtime_scope() {
        let mut app = App::new();

        register_gameplay_plugin_template(&mut app, GameplayPluginTemplate::new(source_contract()));

        let contract = assert_gameplay_contract_present(&app, TEST_SOURCE_PLUGIN);
        assert_eq!(contract.scope, None);
        assert_eq!(contract.scope_model, GameplayScopeModel::SourceOnly);
        contract_has_event(contract, TEST_EVENT).unwrap();
    }

    #[test]
    fn scoped_instance_keys_distinguish_same_plugin_across_arenas() {
        let first = explicit_contract(TEST_PLUGIN, TEST_SCOPE);
        let second = explicit_contract(TEST_PLUGIN, TEST_ALT_SCOPE);
        let mut registry = GameplayPluginContracts::default();

        registry.try_record(first).unwrap();
        registry.try_record(second).unwrap();

        assert_ne!(
            gameplay_plugin_instance_key(first),
            gameplay_plugin_instance_key(second)
        );
        assert_eq!(
            registry.find_instance(gameplay_plugin_instance_key(first)),
            Some(first)
        );
        assert_eq!(
            registry.find_instance(gameplay_plugin_instance_key(second)),
            Some(second)
        );
    }

    #[test]
    fn duplicate_instance_registration_fails_clearly() {
        let contract = explicit_contract(TEST_PLUGIN, TEST_SCOPE);
        let mut registry = GameplayPluginContracts::default();

        registry.try_record(contract).unwrap();

        assert_eq!(
            registry.try_record(contract),
            Err(GameplayContractError::DuplicatePlugin {
                plugin: TEST_PLUGIN,
            })
        );
    }

    #[test]
    fn template_validation_rejects_missing_and_stale_metadata() {
        let missing_plugin = GameplayPluginContract {
            plugin: "",
            ..explicit_contract(TEST_PLUGIN, TEST_SCOPE)
        };
        let missing_schedule = GameplayPluginContract {
            schedules: &[],
            ..explicit_contract(TEST_PLUGIN, TEST_SCOPE)
        };
        let missing_phase = GameplayPluginContract {
            schedules: TEST_EMPTY_PHASE_SCHEDULES,
            ..explicit_contract(TEST_PLUGIN, TEST_SCOPE)
        };
        let missing_ownership = GameplayPluginContract {
            owned_resources: &[],
            owned_events: &[],
            ..explicit_contract(TEST_PLUGIN, TEST_SCOPE)
        };
        let missing_non_claim = GameplayPluginContract {
            non_claims: &[],
            ..explicit_contract(TEST_PLUGIN, TEST_SCOPE)
        };
        let missing_scope = GameplayPluginContract {
            scope: None,
            ..explicit_contract(TEST_PLUGIN, TEST_SCOPE)
        };
        let unexpected_scope = GameplayPluginContract {
            scope: Some(TEST_SCOPE),
            ..source_contract()
        };

        assert_eq!(
            validate_gameplay_plugin_contract(missing_plugin),
            Err(GameplayContractError::MissingPlugin)
        );
        assert_eq!(
            validate_gameplay_plugin_contract(missing_schedule),
            Err(GameplayContractError::MissingSchedule {
                plugin: TEST_PLUGIN,
            })
        );
        assert_eq!(
            validate_gameplay_plugin_contract(missing_phase),
            Err(GameplayContractError::MissingPhase {
                plugin: TEST_PLUGIN,
                schedule: UPDATE_SCHEDULE_LABEL,
            })
        );
        assert_eq!(
            validate_gameplay_plugin_contract(missing_ownership),
            Err(GameplayContractError::MissingOwnership {
                plugin: TEST_PLUGIN,
            })
        );
        assert_eq!(
            validate_gameplay_plugin_contract(missing_non_claim),
            Err(GameplayContractError::MissingNonClaim {
                plugin: TEST_PLUGIN,
            })
        );
        assert_eq!(
            validate_gameplay_plugin_contract(missing_scope),
            Err(GameplayContractError::MissingScope {
                plugin: TEST_PLUGIN,
            })
        );
        assert_eq!(
            validate_gameplay_plugin_contract(unexpected_scope),
            Err(GameplayContractError::UnexpectedScope {
                plugin: TEST_SOURCE_PLUGIN,
            })
        );
    }

    #[test]
    fn stale_contract_fact_checks_fail_by_fact_name() {
        let contract = explicit_contract(TEST_PLUGIN, TEST_SCOPE);

        assert_eq!(
            contract_has_resource(contract, TEST_MISSING_RESOURCE),
            Err(GameplayContractError::StaleResource {
                plugin: TEST_PLUGIN,
                resource: TEST_MISSING_RESOURCE,
            })
        );
        assert_eq!(
            contract_has_event(contract, TEST_MISSING_EVENT),
            Err(GameplayContractError::StaleEvent {
                plugin: TEST_PLUGIN,
                event: TEST_MISSING_EVENT,
            })
        );
        assert_eq!(
            contract_has_non_claim(contract, TEST_MISSING_NON_CLAIM),
            Err(GameplayContractError::StaleNonClaim {
                plugin: TEST_PLUGIN,
                non_claim: TEST_MISSING_NON_CLAIM,
            })
        );
        assert_eq!(
            contract_has_schedule_phases(
                contract,
                EVENT_LOOP_UPDATE_SCHEDULE_LABEL,
                GAMEPLAY_PHASE_ORDER
            ),
            Err(GameplayContractError::StaleSchedule {
                plugin: TEST_PLUGIN,
                schedule: EVENT_LOOP_UPDATE_SCHEDULE_LABEL,
            })
        );
        assert_eq!(
            contract_has_schedule_phases(
                contract,
                UPDATE_SCHEDULE_LABEL,
                TEST_REGRESSED_PHASE_ORDER
            ),
            Err(GameplayContractError::StalePhaseOrder {
                plugin: TEST_PLUGIN,
                schedule: UPDATE_SCHEDULE_LABEL,
            })
        );
    }

    #[test]
    fn gameplay_scope_checks_fail_closed_for_invalid_scopes() {
        let wrong_mode = GameplayScope::new(
            GameplayMode::Survival,
            GameplayArenaId::new(CTF_PRIMARY_ARENA_ID),
        );

        assert_eq!(
            gameplay_scope_check(Some(&TEST_SCOPE), TEST_SCOPE),
            GameplayScopeCheck::Match
        );
        assert_eq!(
            gameplay_scope_check(None, TEST_SCOPE),
            GameplayScopeCheck::Missing
        );
        assert_eq!(
            gameplay_scope_check(Some(&wrong_mode), TEST_SCOPE),
            GameplayScopeCheck::ModeMismatch
        );
        assert_eq!(
            gameplay_scope_check(Some(&TEST_ALT_SCOPE), TEST_SCOPE),
            GameplayScopeCheck::ArenaMismatch
        );
        assert!(!gameplay_scope_matches(Some(&wrong_mode), TEST_SCOPE));
    }
}
