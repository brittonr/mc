# valence-bevy-ecs Change Spec: Valence compatibility fixture modules

## Requirements

### Requirement: Compatibility fixture modularization inventory

r[valence_bevy_ecs.compat_fixture_modularization.inventory] Valence compatibility fixture modularization work MUST inventory CTF and survival example responsibilities, fixture core ownership, plugin/schedule contracts, scenario environment contracts, non-claims, and baseline validation before extraction.

#### Scenario: Fixture ownership is reviewable

r[valence_bevy_ecs.compat_fixture_modularization.inventory.reviewable]
- GIVEN Valence compatibility fixture modularization is selected
- WHEN reviewers inspect the inventory
- THEN CTF, survival, fixture core, runtime config, Bevy system, plugin/schedule, scenario env, and typed milestone responsibilities are named
- AND baseline validation commands are recorded before core changes.

### Requirement: Compatibility fixture module boundaries

r[valence_bevy_ecs.compat_fixture_modularization.module_boundaries] Valence CTF and survival compatibility examples SHOULD be thin app/plugin shells over focused fixture modules for gameplay rules, probe behavior, runtime configuration, schedule contracts, and typed milestone logging.

#### Scenario: Fixture behavior has focused owners

r[valence_bevy_ecs.compat_fixture_modularization.module_boundaries.focused]
- GIVEN a fixture responsibility is reviewed
- WHEN maintainers inspect the example and fixture module tree
- THEN the responsibility is owned by the focused module for its domain
- AND unrelated CTF, survival, runtime config, probe, and schedule concerns are not reintroduced into one large runnable example file.

### Requirement: Compatibility fixture core and shell boundary

r[valence_bevy_ecs.compat_fixture_modularization.core_shell] Deterministic fixture decisions SHOULD be pure over explicit inputs, while Bevy app setup, resources, events, packet writes, world mutation, filesystem reads, and logging remain in thin shells.

#### Scenario: Fixture rules are testable without a server

r[valence_bevy_ecs.compat_fixture_modularization.core_shell.testable]
- GIVEN explicit fixture state, config inputs, and event facts
- WHEN an extracted fixture core computes score, flag, inventory, survival, or probe decisions
- THEN tests can verify the decision without starting a Valence server, mutating a `World`, writing packets, reading files, or emitting logs
- AND Bevy systems own those side effects.

### Requirement: Compatibility fixture parity

r[valence_bevy_ecs.compat_fixture_modularization.parity] Fixture modularization MUST preserve explicit opt-in plugin behavior, schedule contracts, scenario environment variables, typed milestone vocabulary, fixture behavior, runner receipt shapes, and non-claim boundaries.

#### Scenario: Existing fixture rails remain stable

r[valence_bevy_ecs.compat_fixture_modularization.parity.stable]
- GIVEN a supported pre-refactor CTF or survival fixture scenario
- WHEN the modularized fixture runs under the same selected scenario inputs
- THEN schedule ownership, plugin opt-in state, milestones, env contracts, fixture state transitions, and receipt-observed output remain equivalent
- AND no default Valence gameplay, vanilla parity, public-server safety, or production-readiness claim is introduced.

### Requirement: Compatibility fixture modularization tests

r[valence_bevy_ecs.compat_fixture_modularization.tests] The change MUST include positive tests for extracted fixture contracts and negative tests for invalid environment/config inputs, missing schedule facts, disabled plugins, stale fixture state, invalid probe transitions, and overclaim markers.

#### Scenario: Valid fixture contracts pass

r[valence_bevy_ecs.compat_fixture_modularization.tests.positive]
- GIVEN valid representative CTF and survival fixture inputs
- WHEN extracted fixture modules process them
- THEN tests prove expected rules, config decisions, schedule facts, and milestone decisions are produced.

#### Scenario: Invalid fixture contracts fail clearly

r[valence_bevy_ecs.compat_fixture_modularization.tests.negative]
- GIVEN invalid environment values, malformed config, missing schedule facts, disabled plugin wiring, stale fixture state, invalid probe transitions, or overclaim markers
- WHEN extracted fixture modules or schedule checks process them
- THEN tests prove diagnostics are specific and no stale or overclaiming fixture evidence is promoted.

### Requirement: Compatibility fixture modularization validation

r[valence_bevy_ecs.compat_fixture_modularization.validation] The change MUST record affected Valence tests, schedule hygiene, affected mc-compat dry-runs or live rails required by tasks, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence checks before archive.

#### Scenario: Fixture modularization closeout is reviewable

r[valence_bevy_ecs.compat_fixture_modularization.validation.logs]
- GIVEN Valence compatibility fixture modularization is complete
- WHEN the change is closed
- THEN reviewable logs show baseline and post-change fixture tests, schedule hygiene, affected dry-runs or required live rails, positive and negative regression coverage, Cairn gates, and Cairn validation passing.
