# mc-compatibility Change Spec: Valence CTF fixture modules

## Requirements

### Requirement: Valence CTF fixture module boundaries

r[mc_compatibility.valence_fixture_modularity.ctf_boundaries] The Valence CTF compatibility fixture SHOULD expose cohesive module boundaries for runtime config, arena setup, team and flag rules, scoring rules, inventory probes, combat and projectile probes, schedule contracts, and milestone formatting.

#### Scenario: CTF responsibility has one owner

r[mc_compatibility.valence_fixture_modularity.ctf_boundaries.ownership]
- GIVEN a CTF fixture responsibility is reviewed
- WHEN maintainers inspect the fixture module tree
- THEN that responsibility is owned by a focused module or pure fixture-core component
- AND unrelated CTF responsibilities are not added back to the root example shell.

### Requirement: Valence CTF fixture functional core

r[mc_compatibility.valence_fixture_modularity.ctf_functional_core] Non-trivial CTF rule and probe decisions SHOULD live in pure deterministic cores that return explicit decisions for Bevy system shells to apply.

#### Scenario: CTF rule is testable without ECS

r[mc_compatibility.valence_fixture_modularity.ctf_functional_core.testable]
- GIVEN CTF logic decides flag ownership, scoring, inventory probe state, combat evidence, projectile evidence, team reset state, or milestone text
- WHEN the logic is extracted
- THEN the decision can be tested with in-memory inputs
- AND Bevy ECS queries, resource mutation, packet/event emission, filesystem reads, and logging remain in shells.

### Requirement: Valence CTF fixture parity

r[mc_compatibility.valence_fixture_modularity.ctf_parity] CTF fixture modularization MUST preserve existing env flags, milestone vocabulary, schedule contracts, dry-run and live evidence boundaries, and non-claims.

#### Scenario: CTF evidence boundary remains stable

r[mc_compatibility.valence_fixture_modularity.ctf_parity.stable]
- GIVEN a supported pre-refactor CTF fixture probe input
- WHEN the modularized fixture processes the same input
- THEN the emitted milestones, schedule contract behavior, and non-claim boundaries remain equivalent
- AND no new CTF correctness or broad compatibility claim is promoted.

### Requirement: Valence CTF fixture positive tests

r[mc_compatibility.valence_fixture_modularity.ctf_positive_tests] The change MUST include positive tests for representative flag, score, team-balance, inventory, combat, projectile, runtime-config, and milestone decisions.

#### Scenario: Supported CTF decisions pass

r[mc_compatibility.valence_fixture_modularity.ctf_positive_tests.coverage]
- GIVEN representative supported CTF fixture inputs
- WHEN extracted CTF cores process them
- THEN tests prove the expected decisions, state transitions, or milestone text are produced.

### Requirement: Valence CTF fixture negative tests

r[mc_compatibility.valence_fixture_modularity.ctf_negative_tests] The change MUST include negative tests for disabled probes, invalid runtime config, stale flag ownership, duplicate scoring or win emission, malformed inventory events, and unsupported arrow policy input.

#### Scenario: Invalid CTF decisions fail closed

r[mc_compatibility.valence_fixture_modularity.ctf_negative_tests.fail_closed]
- GIVEN invalid or unsupported CTF fixture inputs
- WHEN extracted CTF cores process them
- THEN tests prove the inputs are rejected, ignored, or contained according to current fixture behavior without corrupting state.

### Requirement: Valence CTF fixture validation

r[mc_compatibility.valence_fixture_modularity.ctf_validation] The change MUST record focused Valence/example tests, affected mc-compat dry-runs, Cairn proposal/design/tasks gates, and Cairn validation before archive.

#### Scenario: Reviewable validation exists

r[mc_compatibility.valence_fixture_modularity.ctf_validation.logs]
- GIVEN CTF fixture modularization is complete
- WHEN the change is closed
- THEN reviewable logs show positive and negative fixture-core tests plus affected dry-runs and Cairn gates passing.
