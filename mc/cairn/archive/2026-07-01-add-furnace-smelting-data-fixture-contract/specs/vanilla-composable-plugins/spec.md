## ADDED Requirements

### Requirement: Furnace smelting selected-row data fixture

r[vanilla_composable_plugins.furnace_smelting_data_fixture] Furnace selected-row follow-on work MUST define a Java Edition 1.20.1 / protocol 763 data fixture contract before using fixture rows for stronger behavior evidence.

#### Scenario: Fixture contract is target scoped

r[vanilla_composable_plugins.furnace_smelting_data_fixture.target_scope]
- GIVEN the selected standard-furnace core needs target-version data
- WHEN reviewers inspect the fixture contract
- THEN it records source/provenance fields, target edition, target game version, target protocol, one selected standard-furnace recipe row, one selected fuel row, named cook/burn/stack constants, and explicit non-claims
- AND it does not claim all recipes, smoker behavior, blast-furnace behavior, hopper automation, XP behavior, recipe-book synchronization, chunk-unload semantics, Paper/vanilla parity, Valence runtime integration, public-server safety, or production readiness.

### Requirement: Furnace smelting data fixture validation

r[vanilla_composable_plugins.furnace_smelting_data_fixture.validation] The selected-row fixture implementation MUST include focused validation with positive and negative tests before fixture rows are used by the core.

#### Scenario: Valid selected fixture passes

r[vanilla_composable_plugins.furnace_smelting_data_fixture.validation.positive]
- GIVEN a fixture declares Java Edition 1.20.1 / protocol 763, one standard-furnace recipe row, one fuel row, valid item IDs, valid counts, valid cook ticks, valid burn ticks, and required non-claims
- WHEN the focused fixture validator runs
- THEN it passes and records deterministic evidence under `docs/evidence/`.

#### Scenario: Invalid selected fixture fails clearly

r[vanilla_composable_plugins.furnace_smelting_data_fixture.validation.negative]
- GIVEN a fixture is missing target scope, has malformed item IDs, missing recipe rows, missing fuel rows, zero counts, zero ticks, unsupported furnace kinds, or omits required non-claims
- WHEN the focused fixture validator runs
- THEN it fails with diagnostics naming the rejected field or rule.

### Requirement: Furnace smelting fixture-to-core handoff

r[vanilla_composable_plugins.furnace_smelting_data_fixture.core_handoff] Selected-row fixture work MAY wire the validated fixture into the existing selected-row core checker only as local unit evidence and MUST preserve broader non-claims.

#### Scenario: Fixture handoff remains local

r[vanilla_composable_plugins.furnace_smelting_data_fixture.core_handoff.local]
- GIVEN a validated selected-row fixture feeds the selected-row core checker
- WHEN evidence is promoted
- THEN it claims only local selected-row core behavior over the validated fixture
- AND it does not claim Paper/vanilla parity, all-recipe breadth, Valence Bevy/ECS shell behavior, default plugin membership, public-server safety, or production readiness.

### Requirement: Furnace smelting data fixture closeout

r[vanilla_composable_plugins.furnace_smelting_data_fixture.closeout] Selected-row fixture work MUST record focused validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, flake evidence checks, and archive receipts before closeout.

#### Scenario: Fixture closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_data_fixture.closeout.log]
- GIVEN the selected-row furnace data fixture change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused positive/negative fixture validation, core handoff validation when touched, Cairn gates, Cairn validation, task-evidence validation, accepted spec requirement IDs, evidence-manifest freshness, flake checks, and archive receipts
- AND the evidence preserves non-claims for all-recipe breadth, Paper/vanilla parity, Valence runtime integration, default plugin membership, broad Minecraft compatibility, public-server safety, and production readiness.
