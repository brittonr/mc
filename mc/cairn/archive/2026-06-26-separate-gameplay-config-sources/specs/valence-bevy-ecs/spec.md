# valence-bevy-ecs Change Spec: Gameplay config source separation

## Requirements

### Requirement: Gameplay config source inventory

r[valence_bevy_ecs.gameplay_config_sources.inventory] Gameplay config source work MUST inventory selected env, CLI, file, default, runtime refresh, validation, test, and receipt-facing input contracts before moving config ownership.

#### Scenario: Config inputs are visible

r[valence_bevy_ecs.gameplay_config_sources.inventory.visible]
- GIVEN a CTF, survival compatibility, terrain, or selected example config path is selected for source separation
- WHEN reviewers inspect the inventory
- THEN each env variable, CLI flag, default value, runtime refresh system, parser, validation rule, test assumption, and receipt-facing input contract is recorded
- AND process-global state, arena-scoped state, and fixture-only toggles are classified explicitly.

### Requirement: Typed gameplay config cores

r[valence_bevy_ecs.gameplay_config_sources.typed] Gameplay config parsing and validation SHOULD be pure deterministic cores over explicit inputs that return typed config values or typed errors.

#### Scenario: Config validation is testable without process state

r[valence_bevy_ecs.gameplay_config_sources.typed.pure]
- GIVEN explicit config inputs are provided by a test, env adapter, CLI adapter, or default provider
- WHEN the config parser validates them
- THEN it returns a typed config value or typed diagnostic without reading environment variables, files, clocks, ECS state, logging, or global mutable state
- AND malformed, missing, non-finite, out-of-range, or contradictory inputs fail closed.

### Requirement: Gameplay config source boundary

r[valence_bevy_ecs.gameplay_config_sources.source_boundary] Gameplay plugin systems MUST consume typed config resources or arena-owned config components rather than reading process environment, CLI state, or files directly during gameplay phases.

#### Scenario: Source adapters own side effects

r[valence_bevy_ecs.gameplay_config_sources.source_boundary.adapters]
- GIVEN an env, CLI, or file source is used for a gameplay example or compatibility fixture
- WHEN the source is read
- THEN a source adapter or startup shell performs the I/O and writes typed config into the documented resource or arena scope
- AND gameplay systems read only the typed config surface.

### Requirement: Explicit gameplay config reload

r[valence_bevy_ecs.gameplay_config_sources.reload] Runtime gameplay config reloads MUST be explicit and scoped when config can affect multiple arenas, modes, or fixture instances.

#### Scenario: Reload affects intended scope only

r[valence_bevy_ecs.gameplay_config_sources.reload.scoped]
- GIVEN a runtime config reload is requested while multiple gameplay scopes may exist
- WHEN reload systems apply the new typed config
- THEN only the requested arena, mode, default profile, or explicitly global config changes
- AND stale or wrong-scope reload requests are ignored or diagnosed deterministically.

### Requirement: Gameplay config source tests

r[valence_bevy_ecs.gameplay_config_sources.tests] Gameplay config source separation MUST include positive typed-config/default/source-adapter tests and negative malformed, missing, stale, wrong-scope, disabled-source, and process-env-isolation tests.

#### Scenario: Valid config reaches gameplay systems

r[valence_bevy_ecs.gameplay_config_sources.tests.positive]
- GIVEN valid explicit inputs, defaults, or source-adapter values are supplied
- WHEN selected gameplay plugins run
- THEN systems observe the expected typed config in the documented scope and preserve selected fixture or example behavior.

#### Scenario: Invalid config fails closed

r[valence_bevy_ecs.gameplay_config_sources.tests.negative]
- GIVEN malformed, missing, stale, wrong-scope, disabled-source, or process-env-mutated inputs are present
- WHEN parsers, source adapters, reload systems, or gameplay systems run
- THEN invalid config is rejected, scoped defaults are preserved when appropriate, unrelated arenas are unchanged, and no panic or false milestone occurs.

### Requirement: Gameplay config source validation

r[valence_bevy_ecs.gameplay_config_sources.validation] Gameplay config source work MUST record focused config checks, selected CTF/survival/terrain/example checks, schedule hygiene when wiring changes, Cairn gates, Cairn validation, task-evidence validation, and evidence manifests before archive.

#### Scenario: Config source closeout is reviewable

r[valence_bevy_ecs.gameplay_config_sources.validation.log]
- GIVEN gameplay config source separation is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show pure config tests, source-adapter tests, positive and negative reload/scope tests, selected gameplay/example checks, schedule hygiene when applicable, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, and refreshed evidence manifests.
