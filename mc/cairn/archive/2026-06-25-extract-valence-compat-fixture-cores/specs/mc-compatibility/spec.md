# mc-compatibility Change Spec: Valence compatibility fixture cores

## Requirements

### Requirement: Valence fixture inventory

r[mc_compatibility.valence_fixture_core.inventory] Valence compatibility fixture work MUST inventory CTF and survival fixture responsibilities, rule decisions, Bevy shell code, milestone strings, env toggles, global state, and non-goals before extraction.

#### Scenario: Fixture responsibilities are reviewable

r[mc_compatibility.valence_fixture_core.inventory.reviewable]
- GIVEN `ctf.rs` or `survival_compat.rs` is selected for fixture-core extraction
- WHEN reviewers inspect the inventory
- THEN rule decisions, Bevy systems, resources, env toggles, milestone emitters, global state, and evidence boundaries are classified
- AND non-goals such as production gameplay, full CTF correctness, full survival correctness, and vanilla parity are explicit.

### Requirement: Valence fixture boundaries

r[mc_compatibility.valence_fixture_core.boundaries] Compatibility fixtures SHOULD separate deterministic fixture cores from Bevy ECS adapter systems.

#### Scenario: Adapter owns ECS access

r[mc_compatibility.valence_fixture_core.boundaries.adapter]
- GIVEN a fixture rule is extracted
- WHEN reviewers inspect its boundary
- THEN the pure core consumes explicit snapshots/events and returns decisions, state transitions, or milestone text
- AND Bevy queries, commands, resources, timers, logging, file markers, and world mutation remain in adapter systems.

### Requirement: CTF fixture core

r[mc_compatibility.valence_fixture_core.ctf_core] CTF compatibility fixture behavior SHOULD move deterministic flag, score, race, spawn-reset, inventory, combat, and milestone decisions into testable core functions.

#### Scenario: CTF transition is deterministic

r[mc_compatibility.valence_fixture_core.ctf_core.transition]
- GIVEN explicit CTF state and an observed event such as pickup, return, capture, race attempt, inventory click, or combat hit
- WHEN the CTF fixture core evaluates it
- THEN it returns the documented accepted, rejected, milestone, or no-op transition deterministically
- AND it does not depend on Bevy iteration order or global mutable state.

### Requirement: Survival fixture core

r[mc_compatibility.valence_fixture_core.survival_core] Survival compatibility fixture behavior SHOULD move deterministic block, container, crafting, furnace, hunger, mob, redstone, persistence, block-entity, biome/dimension, and milestone decisions into testable core functions.

#### Scenario: Survival fixture decision is deterministic

r[mc_compatibility.valence_fixture_core.survival_core.decision]
- GIVEN explicit survival fixture state and an observed interaction or packet-derived event
- WHEN the survival fixture core evaluates it
- THEN it returns the documented mutation, rejection, milestone, marker write request, or no-op decision
- AND filesystem marker writes and Bevy world mutations stay in the adapter shell.

### Requirement: Valence fixture state ownership

r[mc_compatibility.valence_fixture_core.state_ownership] Global mutable fixture state SHOULD be replaced by explicit Bevy resources, fixture state structs, or documented temporary compatibility shims.

#### Scenario: Policy state is explicit

r[mc_compatibility.valence_fixture_core.state_ownership.policy]
- GIVEN fixture policy or reload state is needed by a Valence compatibility example
- WHEN the state is read or updated
- THEN ownership is represented by a resource, fixture state value, or explicit input/output boundary
- AND any remaining global state is documented with its safety assumptions and retirement path.

### Requirement: Valence fixture compatibility preservation

r[mc_compatibility.valence_fixture_core.compatibility] Fixture-core extraction MUST preserve example commands, env var contracts, milestone text, scenario behavior, and evidence non-claim boundaries unless another Cairn changes them.

#### Scenario: Fixture evidence remains comparable

r[mc_compatibility.valence_fixture_core.compatibility.stable]
- GIVEN selected mc-compat scenarios run against the extracted fixtures
- WHEN receipts and logs are compared to the pre-extraction contract
- THEN required milestones, forbidden milestones, env toggles, and non-claim fields remain compatible
- AND no default Valence gameplay, production readiness, or vanilla parity claim is added.

### Requirement: Valence fixture tests

r[mc_compatibility.valence_fixture_core.tests] Fixture-core extraction MUST include positive transition tests and negative fail-closed tests for migrated CTF and survival decisions.

#### Scenario: Valid fixture transitions pass

r[mc_compatibility.valence_fixture_core.tests.positive]
- GIVEN valid CTF and survival fixture states and events
- WHEN core tests run
- THEN accepted transitions, expected milestones, and adapter requests match the existing fixture contract.

#### Scenario: Invalid fixture transitions fail closed

r[mc_compatibility.valence_fixture_core.tests.negative]
- GIVEN duplicate wins, duplicate pickups, wrong-team returns, invalid inventory events, missing persistence markers, malformed block-entity state, unsupported policy output, or out-of-order fixture events
- WHEN core tests run
- THEN deterministic rejection/no-op diagnostics are produced
- AND no false success milestone is emitted.

### Requirement: Valence fixture-core validation

r[mc_compatibility.valence_fixture_core.validation] Fixture-core extraction MUST record focused Valence tests/example checks, selected mc-compat rails, Cairn gates, and task-evidence checks before archive.

#### Scenario: Fixture-core closeout is reviewable

r[mc_compatibility.valence_fixture_core.validation.log]
- GIVEN fixture-core extraction is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show positive and negative fixture-core tests, focused Valence/example checks, selected mc-compat dry-runs or live rails as scoped, Cairn proposal/design/tasks gates, Cairn validation, and task-evidence validation.
