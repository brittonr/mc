# Vanilla Composable Plugins Specification

## Purpose

Defines the `vanilla-composable-plugins` capability.

## Requirements

### Requirement: Minecraft Wiki source inventory

r[vanilla_composable_plugins.wiki_inventory] Wiki-guided plugin work MUST inventory selected Minecraft Wiki entry points, target edition/version/protocol assumptions, related extracted-data sources, existing Valence plugin surfaces, and existing compatibility rails before deriving implementation slices.

#### Scenario: Wiki sources are version scoped

r[vanilla_composable_plugins.wiki_inventory.version_scoped]
- GIVEN a wiki page, category, or protocol document is selected as a guide for plugin work
- WHEN reviewers inspect the inventory
- THEN the source URL, page title, target edition, target game version, target protocol when relevant, retrieval date or evidence snapshot, and known version-drift risk are recorded
- AND newer-release behavior is not treated as target-version behavior without a separate version decision.

### Requirement: Domain-to-plugin taxonomy

r[vanilla_composable_plugins.taxonomy] The roadmap SHOULD map wiki domains to candidate composable Valence plugin groups, individual feature plugins, dependency edges, schedule impact, evidence requirements, and explicit non-claims.

#### Scenario: Domain maps to bounded plugins

r[vanilla_composable_plugins.taxonomy.bounded]
- GIVEN a domain such as crafting, smelting, effects, equipment, block interactions, block entities, redstone, mobs, biomes, commands, or protocol behavior is selected
- WHEN the taxonomy is reviewed
- THEN candidate plugin group names, feature plugin names, required dependencies, optional dependencies, default-membership decisions, and unsupported broad claims are distinguishable.

### Requirement: Behavior card contract

r[vanilla_composable_plugins.behavior_cards] Each follow-on wiki-guided plugin implementation SHOULD start from a behavior card that records source pages, version scope, pure rule core, Bevy/ECS shell, data dependencies, positive tests, negative tests, schedule impact, parity evidence, and non-claims.

#### Scenario: Behavior card is implementation-ready

r[vanilla_composable_plugins.behavior_cards.ready]
- GIVEN a bounded plugin feature is selected for implementation
- WHEN reviewers inspect its behavior card
- THEN they can identify the deterministic inputs and outputs for the pure core, the ECS resources/events/components owned by the shell, required game-data tables, required schedule evidence, and the exact compatibility claim that evidence may promote.

### Requirement: Functional core and Bevy shell boundary

r[vanilla_composable_plugins.core_shell] Wiki-derived plugin implementations MUST keep vanilla rule decisions in pure deterministic cores and keep ECS mutation, packet emission, logging, filesystem access, network access, and schedule registration in thin Bevy shell systems.

#### Scenario: Rule core is testable without server I/O

r[vanilla_composable_plugins.core_shell.testable]
- GIVEN a wiki-derived gameplay rule such as recipe matching, burn progress, hunger recovery, armor mitigation, effect ticking, block drop selection, or projectile damage attribution
- WHEN its core is tested
- THEN tests can exercise valid and invalid inputs without starting Valence, connecting a client, reading files, writing packets, or depending on wall-clock time.

### Requirement: Evidence and test policy

r[vanilla_composable_plugins.evidence_policy] Wiki-guided plugin work MUST include positive and negative tests for promoted rule cores and MUST require extracted-data checks or Paper/vanilla parity receipts before claiming target-version vanilla behavior.

#### Scenario: Wiki-derived behavior does not overclaim

r[vanilla_composable_plugins.evidence_policy.non_overclaiming]
- GIVEN a plugin feature is implemented from wiki-guided behavior
- WHEN its evidence is promoted
- THEN the claim names the bounded feature, target version, tested inputs, rejected invalid inputs, and non-claims such as broad vanilla parity, all recipes, all block entities, all mobs, public-server safety, or production readiness unless separately proven.

### Requirement: Plugin sequence and stop conditions

r[vanilla_composable_plugins.sequence] The roadmap SHOULD sequence follow-on plugin Cairns by dependency order, existing evidence-rail reuse, implementation risk, and explicit stop conditions.

#### Scenario: High-risk domains are deferred until prerequisites exist

r[vanilla_composable_plugins.sequence.deferred]
- GIVEN a high-complexity domain such as broad redstone, broad mob AI, world generation, or all-block update semantics is proposed
- WHEN the roadmap sequence is reviewed
- THEN prerequisite data, schedule, parity, and architecture gaps are named before implementation starts
- AND smaller bounded survival or combat seams may proceed independently when their evidence rails are sufficient.

### Requirement: Roadmap validation

r[vanilla_composable_plugins.validation] Roadmap work MUST record Cairn proposal, design, tasks, repository validation, and evidence-manifest checks for promoted roadmap artifacts before archive.

#### Scenario: Roadmap closeout is reviewable

r[vanilla_composable_plugins.validation.log]
- GIVEN the wiki-guided plugin roadmap is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show Cairn proposal/design/tasks gates, Cairn validation, promoted inventory or roadmap artifacts, evidence manifests when cited, and explicit non-claims for implementation, default Valence behavior, broad Minecraft compatibility, vanilla parity, public-server safety, and production readiness.

### Requirement: Furnace smelting behavior card

r[vanilla_composable_plugins.furnace_smelting_card] Furnace smelting follow-on work MUST start from a dedicated behavior card that records source pages, target Java/protocol scope, bounded selected-row claim, non-claims, pure rule-core inputs and outputs, thin Bevy/ECS shell boundaries, positive tests, negative tests, evidence requirements, and stop conditions.

#### Scenario: Furnace card bounds the first slice

r[vanilla_composable_plugins.furnace_smelting_card.bounded]
- GIVEN furnace smelting is selected as the first bounded survival/plugin seam
- WHEN reviewers inspect the behavior card
- THEN it names Java Edition 1.20.1 / protocol 763 scope, selected Minecraft Wiki source pages, target-version recipe/fuel data requirements, selected-row behavior, and future Paper/vanilla parity evidence
- AND it explicitly rejects broad vanilla parity, all-recipe breadth, all block entities, hoppers, XP, smoker/blast-furnace breadth, DefaultPlugins membership changes, public-server safety, and production readiness.

#### Scenario: Furnace card keeps the core testable

r[vanilla_composable_plugins.furnace_smelting_card.core_shell]
- GIVEN a future furnace smelting implementation uses the card
- WHEN its design is reviewed
- THEN furnace semantics are assigned to a pure deterministic rule core over in-memory furnace state, recipe tables, fuel tables, and named constants
- AND Bevy/ECS queries, world mutation, packet/event emission, schedule registration, file reads, data-pack parsing, network access, and logging remain outside the rule core.

### Requirement: Furnace smelting behavior card validation

r[vanilla_composable_plugins.furnace_smelting_card.validation] The furnace smelting card work MUST include focused validation with positive and negative self-tests for required card sections and workflow rules.

#### Scenario: Complete furnace card passes

r[vanilla_composable_plugins.furnace_smelting_card.validation.positive]
- GIVEN the furnace smelting behavior card contains required source, target scope, bounded claim, non-claim, pure core, ECS shell, test, evidence, and stop-condition sections
- WHEN the focused behavior-card validation runs
- THEN it passes with a deterministic success result.

#### Scenario: Incomplete furnace card fails clearly

r[vanilla_composable_plugins.furnace_smelting_card.validation.negative]
- GIVEN a fixture behavior card is missing source scope, target Java/protocol scope, bounded claim, non-claims, pure core boundaries, ECS shell boundaries, positive tests, negative tests, evidence requirements, or stop conditions
- WHEN the focused behavior-card validation runs
- THEN it fails with a diagnostic naming the missing rule.

### Requirement: Furnace smelting behavior card closeout

r[vanilla_composable_plugins.furnace_smelting_card.closeout] Furnace smelting behavior-card work MUST record focused validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, evidence-manifest checks, accepted-spec sync verification, and archive receipts before closeout.

#### Scenario: Furnace card closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_card.closeout.log]
- GIVEN the furnace smelting behavior-card change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused positive/negative validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, evidence-manifest freshness, accepted spec requirement IDs, and archive receipts
- AND the evidence preserves non-claims for Valence implementation, default plugin membership, broad Minecraft compatibility, broad vanilla parity, all recipes, all block entities, public-server safety, and production readiness.

### Requirement: Furnace smelting selected-row pure core

r[vanilla_composable_plugins.furnace_smelting_core] Furnace smelting implementation work MUST provide a pure deterministic selected-row standard-furnace core before any Bevy/ECS shell is introduced.

#### Scenario: Selected-row core is deterministic

r[vanilla_composable_plugins.furnace_smelting_core.deterministic]
- GIVEN a standard furnace state, in-memory selected recipe row, in-memory selected fuel row, and named constants for cook time and stack limits
- WHEN the pure selected-row core advances one tick
- THEN it returns a new furnace state plus a typed transition or typed error without reading files, fetching network pages, mutating Bevy world state, emitting packets/events, logging, or depending on wall-clock time.

#### Scenario: Selected-row core remains bounded

r[vanilla_composable_plugins.furnace_smelting_core.bounded]
- GIVEN the selected-row core passes local tests
- WHEN reviewers inspect promoted evidence
- THEN evidence claims only local selected-row unit semantics
- AND it explicitly rejects Valence runtime integration, DefaultPlugins membership changes, broad vanilla parity, all-recipe breadth, smoker/blast-furnace behavior, hoppers, XP, recipe-book behavior, chunk-unload behavior, public-server safety, and production readiness until separately proven.

### Requirement: Furnace smelting selected-row core tests

r[vanilla_composable_plugins.furnace_smelting_core.tests] The selected-row furnace core MUST include positive and negative tests for valid progress and rejected or blocked states.

#### Scenario: Positive selected-row states pass

r[vanilla_composable_plugins.furnace_smelting_core.tests.positive]
- GIVEN valid selected standard-furnace recipe and fuel rows
- WHEN tests run fuel-start, active-burn progress, compatible output merge, and completed-cook cases
- THEN the core returns expected states and transitions without consuming extra fuel or corrupting input/output slots.

#### Scenario: Negative selected-row states fail safely

r[vanilla_composable_plugins.furnace_smelting_core.tests.negative]
- GIVEN invalid input, missing fuel, wrong output item, full output stack, malformed recipe row, or unsupported furnace kind
- WHEN tests run the selected-row core
- THEN the core returns the expected pause transition or typed error while preserving state that must not change.

### Requirement: Furnace smelting selected-row core documentation

r[vanilla_composable_plugins.furnace_smelting_core.docs] Selected-row core work MUST document local semantics, data assumptions, test coverage, and non-claims.

#### Scenario: Core documentation is reviewable

r[vanilla_composable_plugins.furnace_smelting_core.docs.reviewable]
- GIVEN reviewers inspect selected-row core docs
- WHEN they compare docs with tests and evidence
- THEN they can identify implemented state fields, recipe/fuel assumptions, transitions, errors, positive tests, negative tests, and stop conditions before broader furnace work.

### Requirement: Furnace smelting selected-row core closeout

r[vanilla_composable_plugins.furnace_smelting_core.closeout] Selected-row core work MUST record baseline validation, focused core validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, flake evidence checks, and archive receipts before closeout.

#### Scenario: Core closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_core.closeout.log]
- GIVEN the selected-row furnace core change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline validation, focused positive/negative core tests, Cairn gates, Cairn validation, task-evidence validation, accepted spec requirement IDs, evidence-manifest freshness, flake evidence checks, and archive receipts
- AND the evidence preserves non-claims for Valence runtime integration, default plugin membership, broad Minecraft compatibility, broad vanilla parity, all recipes, all block entities, public-server safety, and production readiness.

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

### Requirement: Furnace smelting selected-row receipt baseline

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.baseline] Selected-row receipt handoff work MUST record the current fixture and core validation baseline before changing checker or handoff logic.

#### Scenario: Baseline is captured before handoff changes

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.baseline.captured]
- GIVEN the selected-row fixture and pure core already exist
- WHEN handoff implementation starts
- THEN baseline validation records the fixture validator result and core checker fixture-handoff result before new receipt-handoff logic is trusted.

### Requirement: Furnace smelting selected-row receipt handoff

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts] Selected-row furnace receipt-handoff work MUST verify that the validated Java Edition 1.20.1 / protocol 763 fixture row matches reviewable Paper/reference and Valence receipt evidence before promoting selected-row target-version behavior beyond local unit semantics.

#### Scenario: Handoff is bounded to one selected row

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.bounded]
- GIVEN the selected standard-furnace fixture and candidate Paper/reference plus Valence receipt evidence are available
- WHEN reviewers inspect the handoff contract
- THEN it maps target edition/version/protocol, furnace kind, input item, fuel item, output item, output count, cook ticks, burn ticks, backend identity, receipt paths, and required non-claims to normalized comparison fields
- AND it states that all-recipe breadth, all-fuel breadth, smoker behavior, blast-furnace behavior, hoppers, XP behavior, recipe-book synchronization, chunk-unload semantics, Valence runtime integration, default plugin membership, broad vanilla parity, public-server safety, and production readiness remain non-claims.

### Requirement: Furnace smelting selected-row receipt checker

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.checker] The handoff implementation MUST provide a focused checker with a pure deterministic comparison core plus a thin file-reading shell.

#### Scenario: Matching selected-row evidence passes

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.checker.positive]
- GIVEN the validated fixture row and Paper/reference plus Valence receipt inputs describe the same selected standard-furnace row
- WHEN the checker runs
- THEN it passes with deterministic diagnostics naming the matched input item, fuel item, output item, output count, cook ticks, burn ticks, and receipt inputs.

#### Scenario: Mismatched or overbroad evidence fails

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.checker.negative]
- GIVEN receipt evidence is missing, Valence-only, Paper-only, stale, malformed, scoped to the wrong row, mismatches item IDs, mismatches counts, mismatches cook ticks, mismatches burn ticks, omits required non-claims, or claims all-furnace/all-recipe breadth
- WHEN the checker runs
- THEN it fails with a diagnostic naming the rejected field or overclaim.

### Requirement: Furnace smelting selected-row receipt evidence

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.evidence] Selected-row handoff work MAY reuse archived Paper/reference and Valence furnace receipts only when the checker proves they match the validated selected-row fixture and preserve the required non-claims; otherwise it MUST stop or produce fresh selected-row receipt evidence before promotion.

#### Scenario: Archived receipts are reused safely

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.evidence.reuse]
- GIVEN archived survival-furnace smelting receipt artifacts are selected as handoff inputs
- WHEN the handoff evidence is promoted
- THEN the promoted log records checker success, exact receipt input paths, BLAKE3 coverage, target scope, selected-row metrics, and retained non-claims
- AND no new live Paper/Valence run is implied unless a fresh receipt log is cited.

### Requirement: Furnace smelting selected-row receipt docs

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.docs] Furnace selected-row documentation MUST distinguish local fixture/core semantics, selected-row receipt handoff evidence, and deferred Valence runtime shell work.

#### Scenario: Handoff docs do not overclaim

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.docs.non_overclaiming]
- GIVEN reviewers inspect furnace selected-row docs after handoff
- WHEN they compare docs with promoted evidence
- THEN they can identify what the handoff proves, which receipt artifacts were used, and which runtime or breadth claims remain deferred.

### Requirement: Furnace smelting selected-row receipt closeout

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.closeout] Selected-row receipt handoff work MUST record baseline validation, focused positive and negative checker tests, handoff validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, flake evidence checks, and archive receipts before closeout.

#### Scenario: Receipt handoff closeout is reviewable

r[vanilla_composable_plugins.furnace_smelting_selected_row_receipts.closeout.log]
- GIVEN the selected-row receipt handoff change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline fixture/core validation, focused checker positive and negative tests, selected-row handoff validation, Cairn gates, Cairn validation, task-evidence validation, accepted-spec IDs, evidence-manifest freshness, flake evidence checks, and archive receipts
- AND the evidence preserves non-claims for all-recipe breadth, all-fuel breadth, Valence runtime integration, default plugin membership, broad Minecraft compatibility, broad vanilla parity, public-server safety, and production readiness.

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

### Requirement: Crafting recipe behavior-card inventory

r[vanilla_composable_plugins.crafting_recipe_card.inventory] Crafting recipe behavior-card work MUST inventory the roadmap crafting scope, accepted behavior-card/core-shell policy, accepted crafting compatibility evidence, target Java/protocol scope, and unresolved target-version recipe-data prerequisites before drafting the card.

#### Scenario: Crafting prerequisites are reviewable

r[vanilla_composable_plugins.crafting_recipe_card.inventory.reviewable]
- GIVEN crafting recipe behavior-card work starts
- WHEN reviewers inspect the inventory
- THEN it names the roadmap crafting row, accepted `vanilla-composable-plugins` behavior-card/core-shell/evidence requirements, accepted `mc-compatibility` crafting recipe-breadth requirements, archived shaped/shapeless/invalid recipe-breadth receipts, typed-event migration evidence, target Java Edition and protocol scope, and unresolved target-version recipe-data prerequisites
- AND it distinguishes predecessor row evidence from future pure-core, Valence shell, all-recipe, recipe-book, data-pack, public-server safety, and production-readiness claims.

### Requirement: Crafting recipe behavior card

r[vanilla_composable_plugins.crafting_recipe_card] Crafting recipe follow-on implementation work MUST start from a dedicated behavior card that records source pages, Java Edition 1.20.1 / protocol 763 scope, selected recipe-matrix rows, non-claims, pure rule-core inputs and outputs, thin Bevy/ECS shell boundaries, positive tests, negative tests, evidence requirements, and stop conditions.

#### Scenario: Crafting card bounds the first recipe slice

r[vanilla_composable_plugins.crafting_recipe_card.bounded]
- GIVEN crafting recipes are selected as the next composable-plugin seam
- WHEN reviewers inspect the behavior card
- THEN it names a finite selected matrix with one shaped recipe, one shapeless recipe, one invalid or insufficient-input rejection, one configured collection mode, target-version recipe-data requirements, and predecessor receipt evidence
- AND it explicitly rejects all-recipe breadth, arbitrary collection modes, shift-click/drag/split breadth, data-pack loading, recipe-book UI behavior, automated crafter behavior, DefaultPlugins membership changes, broad vanilla parity, public-server safety, and production readiness.

#### Scenario: Crafting card keeps the core testable

r[vanilla_composable_plugins.crafting_recipe_card.core_shell]
- GIVEN future crafting recipe work uses the card
- WHEN its design is reviewed
- THEN recipe matching, result selection, rejected/no-result decisions, and malformed recipe diagnostics are assigned to a pure deterministic core over in-memory grid state, selected recipe rows, output-slot state, and collection requests
- AND Bevy/ECS queries, inventory mutation, packet emission, schedule registration, filesystem reads, data-pack parsing, network access, logging, and wall-clock time remain outside the rule core.

### Requirement: Crafting recipe behavior-card validation

r[vanilla_composable_plugins.crafting_recipe_card.validation] The crafting recipe behavior-card work MUST include focused validation with positive and negative self-tests for required card sections, core/shell separation, evidence boundaries, stop conditions, and overclaim rejection.

#### Scenario: Complete crafting card passes

r[vanilla_composable_plugins.crafting_recipe_card.validation.positive]
- GIVEN the crafting recipe behavior card contains required source, target scope, selected recipe matrix, bounded claim, non-claim, pure core, ECS shell, positive-test, negative-test, evidence, and stop-condition sections
- WHEN the focused behavior-card validation runs
- THEN it passes with deterministic diagnostics naming the accepted selected recipe matrix.

#### Scenario: Incomplete or overbroad crafting card fails clearly

r[vanilla_composable_plugins.crafting_recipe_card.validation.negative]
- GIVEN a fixture behavior card is missing source scope, target Java/protocol scope, selected recipe matrix, bounded claim, required non-claims, pure core boundaries, ECS shell boundaries, positive tests, negative tests, evidence requirements, or stop conditions
- OR it claims all recipes, arbitrary collection modes, shift-click/drag/split breadth, data-pack loading, recipe-book UI behavior, automated crafter behavior, DefaultPlugins membership changes, broad vanilla parity, public-server safety, or production readiness
- WHEN the focused behavior-card validation runs
- THEN it fails with a diagnostic naming the missing rule or rejected overclaim.

### Requirement: Crafting recipe behavior-card documentation

r[vanilla_composable_plugins.crafting_recipe_card.docs] Crafting recipe documentation MUST distinguish existing row receipts from future plugin implementation claims and identify the behavior card as the prerequisite before recipe-core or Valence shell work.

#### Scenario: Crafting docs do not overclaim

r[vanilla_composable_plugins.crafting_recipe_card.docs.non_overclaiming]
- GIVEN reviewers inspect crafting recipe docs after behavior-card work
- WHEN they compare the behavior card with existing crafting recipe-breadth receipts
- THEN they can identify which artifacts are predecessor row evidence, which behavior is only planned for a pure core, which shell work remains deferred, and which broad crafting claims remain out of scope.

### Requirement: Crafting recipe behavior-card closeout

r[vanilla_composable_plugins.crafting_recipe_card.closeout] Crafting recipe behavior-card work MUST record focused validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks for promoted artifacts, and explicit non-claims before closeout.

#### Scenario: Crafting card closeout is reviewable

r[vanilla_composable_plugins.crafting_recipe_card.closeout.log]
- GIVEN the crafting recipe behavior-card change is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show focused positive/negative validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted spec requirement IDs, evidence-manifest freshness, and archive receipts
- AND the evidence preserves non-claims for implementation, Valence runtime integration, DefaultPlugins membership changes, all recipes, data packs, recipe-book behavior, broad Minecraft compatibility, broad vanilla parity, public-server safety, and production readiness.

### Requirement: Crafting recipe selected-matrix core inventory

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.inventory] Crafting selected-matrix core work MUST inventory the accepted crafting behavior card, selected matrix rows, predecessor crafting receipts, typed-event migration evidence, local fixture-core assumptions, and unresolved target-version recipe-data prerequisites before implementing core semantics.

#### Scenario: Core prerequisites are reviewable

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.inventory.reviewable]
- GIVEN crafting selected-matrix core work starts
- WHEN reviewers inspect the inventory
- THEN the shaped chest row, shapeless oak-planks row, invalid stick-input rejection row, primary-click collection boundary, accepted behavior-card requirements, predecessor receipt evidence, and unresolved target-version recipe JSON extraction gap are named
- AND predecessor receipts are not treated as proof of a reusable core, all-recipe breadth, Valence shell behavior, public-server safety, or production readiness.

### Requirement: Crafting recipe selected-matrix pure core

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core] Crafting recipe implementation work MUST provide a pure deterministic selected-matrix recipe core before any target-version data loader, receipt handoff, Bevy/ECS shell, scenario rail, or default plugin membership is introduced.

#### Scenario: Selected-matrix core is deterministic

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.deterministic]
- GIVEN an in-memory crafting grid, selected in-memory recipe rows, output-slot state, collection request, and named grid/stack limits
- WHEN the pure selected-matrix core evaluates the selected shaped chest, shapeless oak-planks, invalid stick-input, or primary-click collection case
- THEN it returns a deterministic match, no-result, output-blocked, inventory-delta, or typed malformed-data diagnostic without reading files, fetching network pages, mutating Bevy world state, emitting packets/events, writing logs, inspecting environment variables, or depending on wall-clock time.

#### Scenario: Selected-matrix core remains bounded

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.bounded]
- GIVEN the selected-matrix core passes local tests
- WHEN reviewers inspect promoted evidence
- THEN evidence claims only local selected-matrix unit semantics over in-memory rows
- AND it explicitly rejects target-version recipe extraction, all-recipe breadth, arbitrary collection modes, shift-click/drag/split handling, data-pack loading, recipe-book UI behavior, automated crafter behavior, Valence runtime integration, DefaultPlugins membership changes, broad vanilla parity, broad Minecraft compatibility, public-server safety, and production readiness until separately proven.

### Requirement: Crafting recipe selected-matrix core tests

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.tests] The selected-matrix crafting core MUST include positive and negative tests for valid selected rows, collection behavior, malformed data, unsupported scope, and state preservation.

#### Scenario: Positive selected-matrix states pass

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.tests.positive]
- GIVEN valid selected in-memory recipe rows and compatible state
- WHEN tests run shaped chest matching, shapeless oak-planks matching, and primary-click collection with compatible inventory capacity
- THEN the core returns the expected selected recipe result, inventory delta, and preserved grid or inventory fields without hidden side effects.

#### Scenario: Negative selected-matrix states fail safely

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.tests.negative]
- GIVEN insufficient stick input, blocked output, missing selected data, duplicate recipe ids, malformed shaped rows, malformed shapeless rows, invalid item ids, zero output counts, unsupported recipe kinds, unsupported collection modes, recipe-book UI requests, automated crafter requests, or out-of-scope collection modes
- WHEN tests run the selected-matrix core
- THEN the core returns the expected no-result, output-blocked, or typed error diagnostic while preserving grid and inventory state that must not change.

### Requirement: Crafting recipe selected-matrix core documentation

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.docs] Selected-matrix crafting core work MUST document local semantics, selected row assumptions, named limits, test coverage, future evidence prerequisites, and non-claims.

#### Scenario: Core documentation is reviewable

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.docs.reviewable]
- GIVEN reviewers inspect selected-matrix crafting core docs
- WHEN they compare docs with tests and evidence
- THEN they can identify implemented grid fields, selected recipe rows, collection request boundaries, output-slot behavior, transitions or diagnostics, positive tests, negative tests, and stop conditions before broader crafting work.

### Requirement: Crafting recipe selected-matrix core closeout

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.closeout] Selected-matrix crafting core work MUST record baseline validation, focused core validation, Cairn proposal/design/tasks gates, Cairn validation, task-evidence validation, accepted-spec sync verification, evidence-manifest checks, and archive receipts before closeout.

#### Scenario: Core closeout is reviewable

r[vanilla_composable_plugins.crafting_recipe_selected_matrix_core.closeout.log]
- GIVEN selected-matrix crafting core work is ready to archive
- WHEN reviewers inspect task evidence
- THEN successful logs show baseline crafting-card validation, focused positive and negative core tests, Cairn gates, Cairn validation, task-evidence validation, accepted spec requirement IDs, evidence-manifest freshness, and archive receipts
- AND the evidence preserves non-claims for target-version recipe extraction, all-recipe breadth, arbitrary collection modes, Valence runtime integration, default plugin membership, broad Minecraft compatibility, broad vanilla parity, public-server safety, and production readiness.
