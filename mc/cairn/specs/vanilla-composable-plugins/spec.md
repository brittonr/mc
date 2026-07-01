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
