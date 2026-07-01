## ADDED Requirements

### Requirement: Furnace smelting Valence shell inventory

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.inventory] Furnace smelting Valence shell-contract work MUST inventory the accepted selected-row furnace artifacts, selected-row evidence boundary, and relevant Valence plugin/schedule contract sources before defining a runtime shell boundary.

#### Scenario: Shell prerequisites are reviewable

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.inventory.reviewable]
- GIVEN furnace smelting shell-contract work starts
- WHEN reviewers inspect the inventory
- THEN the behavior card, selected-row core, validated fixture, receipt handoff, target Java/protocol scope, and relevant Valence plugin/schedule contract requirements are named
- AND unresolved runtime API or schedule facts are recorded as implementation-time inspection items instead of assumed behavior.

### Requirement: Furnace smelting Valence shell contract

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract] Furnace smelting runtime work MUST define a Valence Bevy/ECS shell contract before adding any furnace shell system, plugin registration, schedule wiring, or default plugin membership.

#### Scenario: Shell boundary is explicit

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.boundary]
- GIVEN the selected-row core and receipt handoff exist
- WHEN reviewers inspect the shell contract
- THEN it maps future ECS snapshots to selected-row core inputs and maps core outputs to shell-owned mutations or diagnostics
- AND it names planned opt-in plugin ownership, resources, components, events, candidate schedule phase, ordering dependencies, disabled-plugin behavior, data-loading boundaries, packet/logging boundaries, and mutation boundaries.

#### Scenario: Shell contract remains bounded

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.bounded]
- GIVEN a shell contract is accepted
- WHEN later furnace work uses it
- THEN the contract permits only selected-row runtime planning until separate evidence broadens scope
- AND it explicitly rejects DefaultPlugins membership changes, all-recipe breadth, all-fuel breadth, smoker behavior, blast-furnace behavior, hoppers, XP, recipe-book synchronization, chunk-unload semantics, broad vanilla parity, public-server safety, and production readiness.

### Requirement: Furnace smelting Valence shell contract validation

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.validation] Shell-contract work MUST include focused positive and negative validation for contract completeness, core/shell separation, schedule facts, disabled-plugin behavior, test coverage, and overclaim rejection.

#### Scenario: Complete shell contract passes

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.validation.positive]
- GIVEN a shell contract records target scope, selected-row prerequisites, core input/output mapping, shell-owned resources/components/events, schedule facts, disabled-plugin behavior, positive tests, negative tests, evidence requirements, and non-claims
- WHEN the focused contract validator runs
- THEN it passes with deterministic diagnostics naming the accepted shell boundary.

#### Scenario: Incomplete or overbroad shell contract fails

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.validation.negative]
- GIVEN a shell contract is missing target scope, selected-row prerequisites, core/shell separation, shell ownership, schedule facts, disabled-plugin behavior, positive or negative tests, evidence requirements, or required non-claims
- OR it claims DefaultPlugins membership changes, all-recipe breadth, all-fuel breadth, broad furnace parity, broad vanilla parity, public-server safety, or production readiness
- WHEN the focused contract validator runs
- THEN it fails with a diagnostic naming the missing boundary or rejected overclaim.

### Requirement: Furnace smelting Valence shell contract documentation

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.docs] Furnace selected-row documentation MUST identify the shell contract as the required prerequisite before Valence runtime behavior claims.

#### Scenario: Runtime prerequisites are visible

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.docs.reviewable]
- GIVEN reviewers inspect furnace selected-row docs after shell-contract work
- WHEN they compare behavior-card, core, fixture, receipt-handoff, and shell-contract docs
- THEN they can identify which artifacts prove local selected-row semantics, which artifacts bridge selected receipt evidence, and which shell-contract prerequisites remain before runtime integration.

### Requirement: Furnace smelting Valence shell contract closeout

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.closeout] Shell-contract work MUST record baseline fixture/core/receipt-handoff validation, focused contract validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, and schedule checks when plugin wiring or schedule registration changes.

#### Scenario: Shell contract closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_valence_shell_contract.closeout.log]
- GIVEN shell-contract work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline furnace checks, positive and negative contract validation, Cairn gates, Cairn validation, task-evidence validation, accepted spec requirement IDs, evidence-manifest freshness, and any required Valence schedule evidence
- AND the evidence preserves non-claims for Valence runtime integration until separately implemented, default plugin membership, broad Minecraft compatibility, broad vanilla parity, all recipes, all block entities, public-server safety, and production readiness.
