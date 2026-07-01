## ADDED Requirements

### Requirement: Furnace smelting Valence runtime shell inventory

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.inventory] Furnace smelting Valence runtime-shell work MUST inventory the accepted selected-row furnace artifacts, current baseline validation, exact Valence API surfaces, schedule surfaces, and disabled-plugin evidence needs before adding runtime shell code.

#### Scenario: Runtime shell prerequisites are reviewable

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.inventory.reviewable]
- GIVEN selected-row furnace shell implementation starts
- WHEN reviewers inspect the inventory
- THEN the behavior card, selected-row core, selected fixture, receipt handoff, shell contract, target Java/protocol scope, current baseline checks, and inspected Valence furnace/inventory/block-entity/layer/schedule APIs are named
- AND unresolved API or schedule facts stop implementation or are recorded as explicit implementation risks before runtime claims are promoted.

### Requirement: Furnace smelting Valence runtime shell

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell] Furnace smelting runtime-shell work MUST provide an explicit opt-in Valence Bevy/ECS shell for only the selected standard-furnace row, using the existing pure core for rule decisions and preserving default Valence behavior.

#### Scenario: Opt-in shell calls the pure core

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.core_boundary]
- GIVEN the opt-in furnace shell runs over a selected standard-furnace runtime state
- WHEN it evaluates one tick
- THEN it snapshots furnace kind, input slot, fuel slot, output slot, cook ticks, burn ticks, selected recipe row, selected fuel row, and named limits into plain core inputs
- AND it applies only the returned furnace state, transition, or typed error from the pure core.

#### Scenario: Shell remains bounded and explicit

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.bounded]
- GIVEN the runtime shell is installed in a Valence app or focused example fixture
- WHEN reviewers inspect plugin membership and evidence
- THEN the shell is explicit opt-in and `DefaultPlugins` membership remains unchanged
- AND evidence claims only selected-row opt-in shell behavior, not all recipes, all fuels, smoker behavior, blast-furnace behavior, hoppers, XP, recipe-book synchronization, chunk-unload semantics, broad furnace parity, broad vanilla parity, public-server safety, or production readiness.

### Requirement: Furnace smelting Valence runtime shell tests

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests] The selected-row furnace runtime shell MUST include focused positive and negative tests for enabled behavior, rejected inputs, blocked state, stale state, malformed data, and disabled-plugin behavior before runtime-shell evidence is promoted.

#### Scenario: Positive selected-row shell behavior passes

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests.positive]
- GIVEN the opt-in shell is installed with the validated selected-row recipe and fuel fixture
- WHEN focused tests run fuel start, active burn progress, output production, and compatible output merge cases
- THEN the shell snapshots state into the pure core, commits the expected returned state, and emits only documented selected-row transitions or events.

#### Scenario: Negative selected-row shell behavior fails safely

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.tests.negative]
- GIVEN invalid input, missing fuel, blocked output, unsupported furnace kind, malformed data, stale or unloaded block entity/state, or a missing furnace plugin
- WHEN focused tests run the shell or inspect the disabled app
- THEN no false inventory, block-entity, layer, packet, milestone, or gameplay mutation occurs
- AND diagnostics or absence checks name the rejected boundary.

### Requirement: Furnace smelting Valence runtime shell evidence

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.evidence] Runtime-shell work MUST promote reviewable evidence for focused shell tests, schedule facts when wiring changes, disabled-plugin behavior, and retained selected-row non-claims before claiming opt-in Valence shell behavior.

#### Scenario: Shell evidence is non-overclaiming

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.evidence.non_overclaiming]
- GIVEN focused shell evidence is promoted
- WHEN reviewers inspect the evidence and manifests
- THEN cited logs or receipts live under `docs/evidence/`, BLAKE3 manifests cover task-cited artifacts, schedule evidence names plugin configuration and expected systems or sets when applicable, and disabled-plugin evidence is recorded when plugin wiring is introduced
- AND no live Paper rerun, all-recipe breadth, default Valence gameplay, broad Minecraft compatibility, public-server safety, or production readiness claim is implied.

### Requirement: Furnace smelting Valence runtime shell documentation

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.docs] Furnace selected-row documentation MUST distinguish local pure-core semantics, selected-row receipt handoff evidence, opt-in Valence runtime-shell evidence, and deferred breadth or live-rail claims.

#### Scenario: Runtime shell boundary is visible

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.docs.reviewable]
- GIVEN reviewers inspect furnace selected-row docs after runtime-shell work
- WHEN they compare docs with promoted evidence
- THEN they can identify which artifacts prove pure-core semantics, which artifacts prove selected-row receipt handoff, which artifacts prove opt-in runtime-shell behavior, and which claims remain deferred.

### Requirement: Furnace smelting Valence runtime shell closeout

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.closeout] Selected-row runtime-shell work MUST record baseline fixture/core/receipt/contract validation, focused positive and negative shell tests, schedule hygiene when wiring changes, Cairn gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, affected Valence checks, and archive receipts before closeout.

#### Scenario: Runtime shell closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_valence_runtime_shell.closeout.log]
- GIVEN runtime-shell work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline checks, positive and negative shell tests, disabled-plugin coverage, required schedule evidence, Cairn gates, Cairn validation, task-evidence validation, accepted spec IDs, evidence-manifest freshness, affected Valence checks, and archive receipts
- AND the evidence preserves non-claims for DefaultPlugins membership changes, all-recipe breadth, all-fuel breadth, broad furnace parity, broad vanilla parity, broad Minecraft compatibility, public-server safety, and production readiness.
